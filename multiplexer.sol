pragma solidity ^0.8.0;

contract ActionExecutorProxy {
    address public owner;
    address immutable target;
    constructor(address _target, bytes memory constructorData) payable {
        owner = msg.sender;
        target = _target;
        (bool success,) = target.delegatecall(constructorData);
        require(success, "DELEGATECALL_FAILED");
    }
    fallback() external payable {
        require(msg.sender == owner, "01"); // Ownership check
        (bool success,) = target.delegatecall(msg.data);
        require(success, "DELEGATECALL_FAILED");
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ActionExecutor {
    address public owner;

    constructor(bytes memory constructorData) payable {
        owner = msg.sender;
        _executeActions(constructorData);
    }

    receive() external payable {}

    fallback() external payable {
        require(msg.sender == owner, "01"); // Ownership check
        _executeActions(msg.data);
    }

    enum Operation {
        CLEARDATA, 
        SETDATA, 
        SETADDR, 
        CLRVALUE,
        SETVALUE, 
        EXTCODECOPY, 
        PATCH, 
        CALL, 
        CREATE, 
        DELEGATECALL
    }

    function _executeActions(bytes memory data) internal {
        uint256 offset = 0;
        address target;
        uint256 value;
        bool allowFail;
        bytes memory txData;


        while (offset < data.length) {
            Operation op = Operation(uint8(data[offset]));
            offset += 1;

            if (op == Operation.CLEARDATA) {
                uint256 size;
                (size, offset) = _parseUint16(data, offset);
                txData = new bytes(size);
            } 
            else if (op == Operation.SETDATA) {
                uint256 data_offset;
                uint256 n;
                (data_offset, offset) = _parseUint16(data, offset);
                (n, offset) = _parseUint16(data, offset);               
                
                for (uint256 i = 0; i < n; i++) {
                    uint256 value_i;
                    (value_i, offset) = _parseUint256(data, offset);
                    assembly{
                        mstore(add(add(txData, 0x20), mul(i, 0x20)), value_i)
                    }
                }
            } else if (op == Operation.SETADDR) {
                (target, offset) = _parseAddress(data, offset);
            } else if (op == Operation.SETVALUE) {
                (value, offset) =  _parseUint256(data, offset);
            } else if (op == Operation.CLRVALUE) {
                value = 0;
            } else if (op == Operation.EXTCODECOPY) {
                // address: 20-byte address of the contract to query.
                // destOffset: byte offset in the memory where the result will be copied.
                // offset: byte offset in the code to copy.
                // size: byte size to copy.
                address code_contract;
                uint256 data_offset;
                uint256 code_offset;
                uint256 size;
                (code_contract, offset) = _parseAddress(data, offset);
                (data_offset, offset) = _parseUint16(data, offset);
                (code_offset, offset) = _parseUint16(data, offset);
                (size, offset) = _parseUint16(data, offset);
                assembly {
                    extcodecopy(code_contract, add(txData, add(data_offset, 0x20)), code_offset, size)
                }
                offset += size;
            } else if (op == Operation.CALL) {
                (bool success,) = target.call{value: value}(txData);
                value = 0;
                if (!allowFail) {
                    require(success, "CALL_FAILED");
                }           
            } else if (op == Operation.CREATE) {
                assembly {
                    target := create(value, add(txData, 0x20), mload(txData))
                }
                value = 0;
            } else if (op == Operation.DELEGATECALL) {
                (bool _success,) = target.delegatecall(txData);
            }
        }
    }



    function _parseAddress(bytes memory data, uint256 offset) internal pure returns (address, uint256) {
        uint256 addr;
        assembly {
            addr := mload(add(data, offset))
        }
        addr = addr >> 96;
        return (address(uint160(addr)), offset + 20);
    }

    function _parseUint256(bytes memory data, uint256 offset) internal pure returns (uint256, uint256) {
        uint256 value;
        assembly {
            value := mload(add(data, offset)) 
        }
        return (value, offset + 32);
    }

    function _parseUint16(bytes memory data, uint256 offset) internal pure returns (uint256, uint256) {
        uint256 value = uint256(uint8(data[offset])) << 8 | uint256(uint8(data[offset + 1]));
        return (value, offset + 2);
    }

}

// contract Demultiplexer {
//     address public owner;

//     struct TxInfo {
//         address target;
//         uint128 value;
//         bool allowFail;
//         bytes data;
//     }

//     constructor(bytes memory constructorData) payable {
//         owner = msg.sender;
//         _executeActions(constructorData);
//     }

//     fallback() external payable {
//         _executeActions(msg.data);
//     }

//     function _executeActions(bytes memory inputData) internal {
//         require(msg.sender == owner, "01"); // Unauthorized
//         TxInfo memory txInfo;
//         uint256 offset = 0;

//         while (offset < inputData.length) {
//             uint8 opcode = uint8(inputData[offset]);
//             offset += 1;

//             if (opcode == 0x01) { // SETDATA operation
//                 (offset, data) = _parseBytes(inputData, offset);
//             } else if (opcode == 0x02) { // RESETDATA operation
//                 (offset, data) = _resetData(inputData, offset);
//             } else if (opcode == 0x03) { // CALL operation
//                 _performCall(txInfo);
//             } else if (opcode == 0x04) { // CREATE operation
//                 _performCreate(txInfo);
//             } else if (opcode == 0x05) { // SETTARGET operation
//                 (offset, target) = _parseAddress(inputData, offset);
//             } else if (opcode == 0x06) { // SETALLOWFAIL operation
//                 (offset, allowFail) = _parseBoolean(inputData, offset);
//             } else if (opcode == 0x07) { // PATCH operation
//                 (offset, data) = _applyPatch(inputData, offset, data);
//             } else {
//                 revert("04");
//             } 
//         }
//     }

//     function _parseBytes(bytes memory data, uint256 offset) internal pure returns (uint256, bytes memory) {
//         uint16 size;
//         assembly {
//             size := mload(add(data, add(offset, 2)))
//         }
//         bytes memory result = new bytes(size);
//         for (uint16 i = 0; i < size; i++) {
//             result[i] = data[offset + 2 + i];
//         }
//         return (offset + 2 + size, result);
//     }

//     function _resetData(bytes memory data, uint256 offset) internal pure returns (uint256, bytes memory) {
//         uint16 size;
//         assembly {
//             size := mload(add(data, offset))
//         }
//         return (offset + 2, new bytes(size));
//     }

//     function _parseAddress(bytes memory data, uint256 offset) internal pure returns (uint256, address) {
//         address result;
//         assembly {
//             result := mload(add(data, add(offset, 20)))
//         }
//         return (offset + 20, result);
//     }

//     function _parseBoolean(bytes memory data, uint256 offset) internal pure returns (uint256, bool) {
//         bool result = (data[offset] == 0x01);
//         return (offset + 1, result);
//     }

//     function _applyPatch(bytes memory data, uint256 offset, bytes memory currentData) internal pure returns (uint256, bytes memory) {
//         uint16 patchCount;
//         assembly {
//             patchCount := mload(add(data, offset))
//         }
//         offset += 2;

//         for (uint16 i = 0; i < patchCount; i++) {
//             uint16 patchOffset;
//             uint16 patchSize;

//             assembly {
//                 patchOffset := mload(add(data, offset))
//                 patchSize := mload(add(data, add(offset, 2)))
//             }
//             offset += 4;

//             for (uint16 j = 0; j < patchSize; j++) {
//                 currentData[patchOffset + j] = data[offset + j];
//             }
//             offset += patchSize;
//         }
//         return (offset, currentData);
//     }

//     function _performCall(TxInfo memory txInfo) internal {
//         (bool success, ) = target.call{value: value}(data);
//         if (!success && !allowFail) revert("02"); // Call failed
//     }

//     function _performCreate(TxInfo memory txInfo) internal {
//         uint256 value = value;
//         bytes memory initCode = data;
//         uint256 initCodeSize = initCode.length;
//         address newContract;
//         assembly {
//             newContract := create(value, add(initCode, 0x20), initCodeSize)
//         }
//         if (newContract == address(0) && !allowFail) revert("03"); // Create failed
//         // Set the newly created contract as the target for the next CALL/CREATE
//         target = newContract;
//     }
// }
