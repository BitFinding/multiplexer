// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Demultiplexer {
    address public owner;

    struct TxInfo {
        address target;
        uint128 value;
        bool allowFail;
        bytes data;
    }

    constructor(bytes memory constructorData) payable {
        owner = msg.sender;
        _executeActions(constructorData);
    }

    fallback() external payable {
        _executeActions(msg.data);
    }

    function _executeActions(bytes memory inputData) internal {
        require(msg.sender == owner, "01"); // Unauthorized
        TxInfo memory txInfo;
        uint256 offset = 0;

        while (offset < inputData.length) {
            uint8 opcode = uint8(inputData[offset]);
            offset += 1;

            if (opcode == 0x01) { // SETDATA operation
                (offset, txInfo.data) = _parseBytes(inputData, offset);
            } else if (opcode == 0x02) { // RESETDATA operation
                (offset, txInfo.data) = _resetData(inputData, offset);
            } else if (opcode == 0x03) { // CALL operation
                _performCall(txInfo);
            } else if (opcode == 0x04) { // CREATE operation
                _performCreate(txInfo);
            } else if (opcode == 0x05) { // SETTARGET operation
                (offset, txInfo.target) = _parseAddress(inputData, offset);
            } else if (opcode == 0x06) { // SETALLOWFAIL operation
                (offset, txInfo.allowFail) = _parseBoolean(inputData, offset);
            } else if (opcode == 0x07) { // PATCH operation
                (offset, txInfo.data) = _applyPatch(inputData, offset, txInfo.data);
            } else {
                revert("04");
            } 
        }
    }

    function _parseBytes(bytes memory data, uint256 offset) internal pure returns (uint256, bytes memory) {
        uint16 size;
        assembly {
            size := mload(add(data, add(offset, 2)))
        }
        bytes memory result = new bytes(size);
        for (uint16 i = 0; i < size; i++) {
            result[i] = data[offset + 2 + i];
        }
        return (offset + 2 + size, result);
    }

    function _resetData(bytes memory data, uint256 offset) internal pure returns (uint256, bytes memory) {
        uint16 size;
        assembly {
            size := mload(add(data, offset))
        }
        return (offset + 2, new bytes(size));
    }

    function _parseAddress(bytes memory data, uint256 offset) internal pure returns (uint256, address) {
        address result;
        assembly {
            result := mload(add(data, add(offset, 20)))
        }
        return (offset + 20, result);
    }

    function _parseBoolean(bytes memory data, uint256 offset) internal pure returns (uint256, bool) {
        bool result = (data[offset] == 0x01);
        return (offset + 1, result);
    }

    function _applyPatch(bytes memory data, uint256 offset, bytes memory currentData) internal pure returns (uint256, bytes memory) {
        uint16 patchCount;
        assembly {
            patchCount := mload(add(data, offset))
        }
        offset += 2;

        for (uint16 i = 0; i < patchCount; i++) {
            uint16 patchOffset;
            uint16 patchSize;

            assembly {
                patchOffset := mload(add(data, offset))
                patchSize := mload(add(data, add(offset, 2)))
            }
            offset += 4;

            for (uint16 j = 0; j < patchSize; j++) {
                currentData[patchOffset + j] = data[offset + j];
            }
            offset += patchSize;
        }
        return (offset, currentData);
    }

    function _performCall(TxInfo memory txInfo) internal {
        (bool success, ) = txInfo.target.call{value: txInfo.value}(txInfo.data);
        if (!success && !txInfo.allowFail) revert("02"); // Call failed
    }

    function _performCreate(TxInfo memory txInfo) internal {
        uint256 value = txInfo.value;
        bytes memory initCode = txInfo.data;
        uint256 initCodeSize = initCode.length;
        address newContract;
        assembly {
            newContract := create(value, add(initCode, 0x20), initCodeSize)
        }
        if (newContract == address(0) && !txInfo.allowFail) revert("03"); // Create failed
        // Set the newly created contract as the target for the next CALL/CREATE
        txInfo.target = newContract;
    }
}