// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title Executor Contract
 * @notice A flexible contract that can execute a series of actions including flash loans
 * @dev This contract supports multiple flash loan protocols (Morpho, Aave, ERC3156) and
 *      allows for complex transaction execution with various low-level operations
 *
 * @custom:security-contact security@yourproject.com
 */
contract executor {
    /// @notice The address that can initiate actions
    address owner;
    /// @notice Address allowed to trigger callback functions
    address callbackAddress;

    /**
     * @notice Creates a new executor instance
     * @dev Sets the contract owner to the transaction origin
     */
    constructor() payable { 
        owner = tx.origin; // Note: tx.origin used intentionally for specific use case
    }

    receive() external payable {}
    fallback() external payable {}

    /**
     * @notice Supported operation types for the executor
     * @dev Each action corresponds to a specific operation in the execution flow
     */
    enum Action {
        EOF,            // End of flow marker
        CLEARDATA,      // Clear the transaction data buffer
        SETDATA,        // Set data at specific offset
        SETADDR,        // Set target address
        SETVALUE,       // Set ETH value for calls
        EXTCODECOPY,    // Copy external contract code
        CALL,           // Perform external call
        CREATE,         // Deploy new contract
        DELEGATECALL,   // Perform delegate call
        SETCALLBACK,    // Set callback address
        SETFAIL,        // Enable revert on call failure
        CLEARFAIL       // Disable revert on call failure
    }

    /**
     * @dev Internal callback handler for flash loan protocols
     * @param calldata_offset The offset in calldata where execution instructions begin
     * @notice Validates callback sender and executes the provided instructions
     */
    function _onCallback(uint256 calldata_offset) internal {
        require(msg.sender == callbackAddress, "Invalid callback sender");
        callbackAddress = address(0); // Reset callback address for security
        _executeActions(calldata_offset);
    }

    /**
     * @notice Morpho flash loan callback handler
     * @dev Calldata offset calculation simplified to constant 100
     * Original calculation: 4 (function selector) + 32 + calldataload(4 + 32)
     * @param amount The amount of tokens borrowed
     * @param data Additional call parameters
     */
    function onMorphoFlashLoan(uint256 amount, bytes calldata data) external {
        // Simplified from dynamic calculation to fixed offset
        // assembly {
        //     calldata_offset := add(4, add(32, calldataload(add(4, mul(1, 32)))))
        // }
        _onCallback(100); // Fixed offset based on Morpho's calldata layout
    }

    /**
     * @notice Aave flash loan callback handler
     * @dev Calldata offset calculation simplified to constant 196
     * Original calculation: 4 (selector) + 32 + calldataload(4 + 4*32)
     * @param asset The address of the flash-borrowed asset
     * @param amount The amount of the flash-borrowed asset
     * @param premium The fee of the flash-borrowed asset
     * @param initiator The address initiating the flash loan
     * @param params Arbitrary packed params to pass to the receiver as extra information
     * @return true if the flash loan was successful
     */
    function executeOperation(
        address asset,
        uint256 amount, 
        uint256 premium,
        address initiator,
        bytes calldata params
    ) external returns (bool) {
        // Simplified from dynamic calculation to fixed offset
        // assembly {
        //     calldata_offset :=  add(4, add(32, calldataload(add(4, mul(4, 32)))))
        // }
        _onCallback(196); // Fixed offset based on Aave's calldata layout
        return true;
    }


    /**
     * @notice ERC3156 flash loan callback handler
     * @dev Implements the ERC3156 flash loan receiver interface
     * @param initiator The initiating address of the flash loan
     * @param token The address of the flash-borrowed token
     * @param amount The amount of tokens borrowed
     * @param fee The fee to be paid for the flash loan
     * @param data Additional parameters (unused but required by interface)
     * @return Returns the ERC3156 flash loan receiver selector
     * @custom:security Uses inline assembly for efficient calldata parsing
     */
    function onFlashLoan(
        address initiator,
        address token,
        uint256 amount,
        uint256 fee,
        bytes calldata data
    ) external returns (bytes32) {
        uint256 calldata_offset;
        // Calculate offset: function selector (4) + 32 + position of data parameter (4*32)
        assembly {
            calldata_offset := add(4, add(32, calldataload(add(4, mul(4, 32)))))
        }
        _onCallback(calldata_offset);
        return keccak256("ERC3156FlashBorrower.onFlashLoan");
    }

    /**
     * @notice Main entry point for executing a series of actions
     * @dev Payable to allow receiving ETH for operations
     */
    function executeActions() external payable {
        _executeActions(4); // Skip function selector (4 bytes)
    }

    /**
     * @notice Internal function to execute a series of actions
     * @dev Processes a byte stream of actions with their parameters
     * 
     * Memory Management:
     * The contract maintains a single dynamic bytes array (txData) that serves as a
     * working buffer for all operations. This buffer is:
     * - Cleared and resized by CLEARDATA
     * - Written to by SETDATA and EXTCODECOPY
     * - Read from by CALL, DELEGATECALL, and CREATE
     * 
     * Memory Layout:
     * txData (bytes array):
     * - 0x00-0x20: Length of array (32 bytes)
     * - 0x20-onwards: Actual data bytes
     * 
     * All operations that write to txData must respect:
     * - Array bounds
     * - Proper offset calculation
     * - Word alignment for 32-byte operations
     * 
     * @param calldata_offset Starting position in calldata to read actions from
     * @custom:security Uses tx.origin intentionally for specific authorization model
     */
    function _executeActions(uint256 calldata_offset) internal {
        bytes calldata data = msg.data[calldata_offset:];
        uint256 offset = 0;
        address target;        // Target address for calls
        uint256 value;        // ETH value for calls
        bool fail = false;    // Fail flag for call operations
        bytes memory txData;  // Transaction data buffer
        require(tx.origin == owner, "Unauthorized"); // Ownership validation

        unchecked{
            while (offset < data.length) {
                Action op = Action(uint8(data[offset]));
                offset += 1;

                if (op == Action.EOF) {
                    break;
                }
                else if (op == Action.CLEARDATA) {
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
                    // First loop: Copy full 32-byte words efficiently using assembly
                    for (i = 0; i < data_size/32; i++) {
                        uint256 value_i;
                        (value_i, offset) = _parseUint256(data, offset);
                        assembly{
                            // Memory layout for txData:
                            // txData   : points to array struct
                            // +0x20    : skips length prefix
                            // +offset  : moves to target position
                            // +i*0x20  : moves to current 32-byte word
                            mstore(add(add(add(txData, 0x20), data_offset), mul(i, 0x20)), value_i)
                        }
                    }
                    // Second loop: Copy remaining bytes one by one
                    for (i = ((data_size/32) * 32); i < data_size; i++) {
                        txData[data_offset + i] = data[offset];
                        offset+=1;
                    }
                } else if (op == Action.SETADDR) {
                    (target, offset) = _parseAddress(data, offset);
                } else if (op == Action.SETVALUE) {
                    (value, offset) =  _parseUint256(data, offset);
                } else if (op == Action.EXTCODECOPY) {
                    // Parameters for extcodecopy:
                    // 1. address: 20-byte address of the contract to query
                    // 2. destOffset: memory position where code will be copied
                    // 3. offset: position in contract code to start copying
                    // 4. size: number of bytes to copy
                    address code_contract;
                    uint256 data_offset;
                    uint256 code_offset;
                    uint256 size;
                    (code_contract, offset) = _parseAddress(data, offset);
                    (data_offset, offset) = _parseUint16(data, offset);
                    (code_offset, offset) = _parseUint16(data, offset);
                    (size, offset) = _parseUint16(data, offset);
                    assembly {
                        // Memory layout for destination:
                        // txData   : array pointer
                        // +0x20    : skip length prefix
                        // +offset  : target position in array
                        extcodecopy(
                            code_contract,                    // source contract
                            add(txData, add(data_offset, 0x20)), // destination in memory
                            code_offset,                      // start position in source
                            size                             // number of bytes
                        )
                    }
                } else if (op == Action.CALL) {
                    // Perform external call with current txData buffer
                    // txData contains the complete calldata including:
                    // - function selector (4 bytes)
                    // - encoded parameters (remaining bytes)
                    bool success;
                    (success, ) = target.call{value: value}(txData);
                    if (fail) {
                        require(success, "CALL_FAILED");
                    }
                    value = 0; // Reset value for safety
                } else if (op == Action.CREATE) {                    
                    assembly {
                        // Memory layout for contract creation:
                        // txData    : points to array struct
                        // mload(txData): gets the length of the initialization code
                        // add(txData, 0x20): points to the actual initialization code
                        //
                        // create(value, offset, size):
                        // - value: amount of ETH to send
                        // - offset: memory position of init code
                        // - size: length of init code
                        target := create(
                            value,                  // ETH value for new contract
                            add(txData, 0x20),     // Skip array length word
                            mload(txData)          // Size of initialization code
                        )
                    }
                    value = 0; // Reset value after use
                } else if (op == Action.DELEGATECALL) {
                    // Perform delegatecall using current txData buffer
                    // Note: delegatecall runs code in the context of THIS contract:
                    // - uses this contract's storage
                    // - uses this contract's ETH balance
                    // - msg.sender remains the original caller
                    bool success;
                    (success, ) = target.delegatecall(txData);
                    if (fail) {
                        require(success, "DELCALL_FAILED");
                    }
                } else if (op == Action.SETCALLBACK) {
                    (callbackAddress, offset) = _parseAddress(data, offset);
                } else if (op == Action.SETFAIL) {
                    fail = true;
                } else if (op == Action.CLEARFAIL) {
                    fail = false;
                }

            }
        }
    }

    /**
     * @notice Parse a function selector from byte array
     * @dev Memory layout for bytes array:
     *      - 0x00-0x20: length of array (32 bytes)
     *      - 0x20+: actual bytes data
     *      The assembly loads 32 bytes starting at data[offset]
     * @param data Source byte array
     * @param offset Starting position in the array
     * @return bytes4 The parsed function selector
     * @return uint256 The new offset after parsing
     */
    function _parseFuncId(bytes memory data, uint256 offset) internal pure returns (bytes4, uint256) {
        bytes4 funcId;
        assembly {
            // data points to the bytes array in memory
            // add(data, 0x20) skips the length field
            // add(..., offset) moves to the desired position
            funcId := mload(add(add(data, offset), 0x20))
        }
        return (funcId, offset + 4);
    }

    /**
     * @notice Parse an Ethereum address from byte array
     * @dev Memory layout handling:
     *      1. data points to the bytes array struct in memory
     *      2. First 32 bytes at data contain the array length
     *      3. Actual bytes start at data + 0x20
     *      4. We load 32 bytes but only want last 20 bytes for address
     * 
     * @param data Source byte array
     * @param offset Starting position in the array
     * @return address The parsed address
     * @return uint256 The new offset after parsing
     */
    function _parseAddress(bytes memory data, uint256 offset) internal pure returns (address, uint256) {
        uint256 addr;
        assembly {
            // Load 32 bytes from position (data + 0x20 + offset)
            // -  pointer to bytes array struct
            // - 0x20: skip array length field
            // - offset: position in actual data
            addr := mload(add(add(data, offset), 0x20))
        }
        // Shift right by 96 bits (12 bytes) to get only the last 20 bytes
        // This aligns the address to the least significant bits
        addr = addr >> 96;
        return (address(uint160(addr)), offset + 20);
    }

    /**
     * @notice Parse a uint256 from byte array
     * @dev Uses assembly for efficient memory operations
     * @param data Source byte array
     * @param offset Starting position in the array
     * @return uint256 The parsed value
     * @return uint256 The new offset after parsing
     */
    function _parseUint256(bytes memory data, uint256 offset) internal pure returns (uint256, uint256) {
        uint256 value;
        assembly {
            value := mload(add(add(data, offset),0x20)) 
        }
        return (value, offset + 32);
    }

    /**
     * @notice Parse a uint16 from byte array
     * @dev Combines two bytes into a uint16
     * @param data Source byte array
     * @param offset Starting position in the array
     * @return uint256 The parsed value
     * @return uint256 The new offset after parsing
     */
    function _parseUint16(bytes memory data, uint256 offset) internal pure returns (uint256, uint256) {
        uint256 value = uint256(uint8(data[offset])) << 8 | uint256(uint8(data[offset + 1]));
        return (value, offset + 2);
    }
}
