// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ActionExecutor {
    address public owner;

    constructor() payable { owner = msg.sender; }

    receive() external payable {}

    fallback() external payable { 
        require(msg.sender == owner); // Ownership check
        _executeActions(msg.data);
    }

    enum Operation {
        CLEARDATA, 
        SETDATA, 
        SETADDR, 
        SETVALUE,
        EXTCODECOPY, 
        CALL,
        CREATE, 
        DELEGATECALL
    }

    function _executeActions(bytes memory data) internal {
        uint256 offset = 0;
        address target;
        uint256 value;
        bytes memory txData;

        unchecked{
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
                } else if (op == Operation.CALL) {
                    bool success;
                    (success, ) = target.call{value: value}(txData);
                    value = 0;
                } else if (op == Operation.CREATE) {
                    assembly {
                        target := create(value, add(txData, 0x20), mload(txData))
                    }
                    value = 0;
                } else if (op == Operation.DELEGATECALL) {
                    bool success;
                    (success, ) = target.delegatecall(txData);
                }
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