// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ActionExecutor {
    event LOG(string);
    event LOGBYTES(bytes);
    event LOGADDR(address);
    event LOGBOOL(bool);
    event LOGUINT(uint256);


    address public owner;

    constructor() payable { owner = msg.sender; }

    receive() external payable {}

    fallback() external payable {
        emit LOG("CALLBACK");
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
        emit LOG("EXEACTIONS");

        uint256 offset = 0;
        address target;
        uint256 value;
        bytes memory txData;

        unchecked{
            while (offset < data.length) {
                Operation op = Operation(uint8(data[offset]));
                offset += 1;

                if (op == Operation.CLEARDATA) {
                    emit LOG("CLEARDATA");
                    emit LOGUINT(offset);


                    uint256 size;
                    (size, offset) = _parseUint16(data, offset);
                    emit LOGUINT(size);

                    txData = new bytes(size);
                } 
                else if (op == Operation.SETDATA) {
                    emit LOG("SETDATA");
                    emit LOG("calldata_offset: ");

                    emit LOGUINT(offset);


                    uint256 data_offset;
                    uint256 data_size;
                    (data_offset, offset) = _parseUint16(data, offset);
                    (data_size, offset) = _parseUint16(data, offset);               
                    emit LOG("data_offset: ");
                    emit LOGUINT(data_offset);
                    emit LOG("data_size: ");
                    emit LOGUINT(data_size);

                    uint256 i;
                    for (i = 0; i < data_size/32; i++) {
                        uint256 value_i;
                        (value_i, offset) = _parseUint256(data, offset);
                        emit LOG("ITERATION: ");
                        emit LOGUINT(i);
                        emit LOGUINT(offset);
                        emit LOGUINT(value_i);
                        assembly{
                            mstore(add(add(add(txData, 0x20), data_offset), mul(i, 0x20)), value_i)
                        }
                    }
                    for (i=(data_size/32)*32; i < data_size; i++) {
                        emit LOG("ITERATION: ");
                        emit LOGUINT(i);
                        emit LOGUINT(offset);
                        emit LOGUINT(0xffffffff);

                        txData[data_offset + ((data_size/32) * 32) + i] = data[offset];

                        emit LOGUINT(data_offset + ((data_size/32) * 32) + i);
                        emit LOGUINT(offset);

                        offset+=1;
                    }
                    emit LOGUINT(offset);
                    emit LOG("SETDATA2");

                } else if (op == Operation.SETADDR) {
                    emit LOG("SETADDR");
                    emit LOGUINT(offset);


                    (target, offset) = _parseAddress(data, offset);
                } else if (op == Operation.SETVALUE) {
                    emit LOG("SETVALUE");
                    emit LOGUINT(offset);


                    (value, offset) =  _parseUint256(data, offset);
                } else if (op == Operation.EXTCODECOPY) {
                    emit LOG("EXTCODECOPY");

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
                    emit LOG("CALL");
                    emit LOGUINT(offset);

                    emit LOGADDR(target);
                    emit LOGBYTES(txData);
                    bool success;
                    (success, ) = target.call{value: value}(txData);
                    emit LOGBOOL(success);

                    value = 0;
                } else if (op == Operation.CREATE) {
                    emit LOG("CREATE");
                    assembly {
                        target := create(value, add(txData, 0x20), mload(txData))
                    }
                    emit LOGADDR(target);
                    value = 0;
                } else if (op == Operation.DELEGATECALL) {
                    emit LOG("DELEGATE");

                    bool success;
                    (success, ) = target.delegatecall(txData);
                }
            }
        }
    }

    function _parseAddress(bytes memory data, uint256 offset) internal pure returns (address, uint256) {
        uint256 addr;
        assembly {
            addr := mload(add(add(data, offset), 0x20))
        }
        addr = addr >> 96;
        return (address(uint160(addr)), offset + 20);
    }

    function _parseUint256(bytes memory data, uint256 offset) internal pure returns (uint256, uint256) {
        uint256 value;
        assembly {
            value := mload(add(add(data, offset),0x20)) 
        }
        return (value, offset + 32);
    }

    function _parseUint16(bytes memory data, uint256 offset) internal pure returns (uint256, uint256) {
        uint256 value = uint256(uint8(data[offset])) << 8 | uint256(uint8(data[offset + 1]));
        return (value, offset + 2);
    }

}