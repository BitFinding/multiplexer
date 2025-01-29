// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract executor {
    address owner;
    address callbackAddress;

    constructor() payable { owner = tx.origin; }

    receive() external payable {}
    fallback() external payable {}

    enum Action {
        CLEARDATA, 
        SETDATA, 
        SETADDR, 
        SETVALUE,
        EXTCODECOPY, 
        CALL,
        CREATE, 
        DELEGATECALL,
        SETCALLBACK
    }

    // Morpho
    function onMorphoFlashLoan(uint256 amount, bytes calldata data) external{
        this.onFlashLoan(msg.sender, address(0), amount, 0, data);
    }
    // Aave
    function executeOperation(
        address asset,
        uint256 amount, 
        uint256 premium,
        address initiator,
        bytes calldata params
    ) external returns (bool){
        this.onFlashLoan(initiator, asset, amount, premium, params);
        return true;
    }


    //  @dev Receive a flash loan.
    //  @param initiator The initiator of the loan.
    //  @param token The loan currency.
    //  @param amount The amount of tokens lent.
    //  @param fee The additional amount of tokens to repay.
    //  @param data Arbitrary data structure, intended to contain user-defined parameters.
    //  @return The keccak256 hash of "ERC3156FlashBorrower.onFlashLoan"
    function onFlashLoan(
        address /*initiator*/,
        address /*token*/,
        uint256 /*amount*/,
        uint256 /*fee*/,
        bytes calldata /*data*/
    ) external returns (bytes32){
        require(msg.sender == callbackAddress);
        callbackAddress = address(0);
        uint256 calldata_offset;
        assembly {
            calldata_offset := add(32, calldataload(add(4, mul(4, 32))))
        }
        _executeActions(calldata_offset);
        return keccak256("ERC3156FlashBorrower.onFlashLoan");
    }

    function executeActions() external payable{
        _executeActions(4);
    }

    function _executeActions(uint256 calldata_offset) internal {
        bytes calldata data = msg.data[calldata_offset:];
        uint256 offset = 0;
        address target;
        uint256 value;
        bytes memory txData;
        require(tx.origin == owner); // Ownership check via

        unchecked{
            while (offset < data.length) {
                Action op = Action(uint8(data[offset]));
                offset += 1;

                if (op == Action.CLEARDATA) {
                    uint256 size;
                    (size, offset) = _parseUint16(data, offset);
                    txData = new bytes(size);
                } 
                else if (op == Action.SETDATA) {
                    uint256 data_offset;
                    uint256 data_size;
                    (data_offset, offset) = _parseUint16(data, offset);
                    (data_size, offset) = _parseUint16(data, offset);               
                    uint256 i;
                    for (i = 0; i < data_size/32; i++) {
                        uint256 value_i;
                        (value_i, offset) = _parseUint256(data, offset);
                        assembly{
                            mstore(add(add(add(txData, 0x20), data_offset), mul(i, 0x20)), value_i)
                        }
                    }
                    for (i = ((data_size/32) * 32); i < data_size; i++) {
                        txData[data_offset + i] = data[offset];
                        offset+=1;
                    }
                } else if (op == Action.SETADDR) {
                    (target, offset) = _parseAddress(data, offset);
                } else if (op == Action.SETVALUE) {
                    (value, offset) =  _parseUint256(data, offset);
                } else if (op == Action.EXTCODECOPY) {
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
                } else if (op == Action.CALL) {
                    bool success;
                    (success, ) = target.call{value: value}(txData);
                    value = 0;
                } else if (op == Action.CREATE) {                    
                    assembly {
                        target := create(value, add(txData, 0x20), mload(txData))
                    }
                    value = 0;
                } else if (op == Action.DELEGATECALL) {
                    bool success;
                    (success, ) = target.delegatecall(txData);
                } else if (op == Action.SETCALLBACK) {
                    (callbackAddress, offset) = _parseAddress(data, offset);
                }

            }
        }
    }

    function _parseFuncId(bytes memory data, uint256 offset) internal pure returns (bytes4, uint256) {
        bytes4 funcId;
        assembly {
            funcId := mload(add(add(data, offset), 0x20))
        }
        return (funcId, offset + 4);
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