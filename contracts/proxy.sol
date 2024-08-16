// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title Simple Immutable Proxy
 * @notice A basic proxy contract that delegates all calls to a fixed target implementation.
 * @dev Uses an immutable target address set during deployment.
 *      Ownership is assigned to tx.origin and checked on subsequent calls.
 */
contract proxy {
    /// @notice The address that deployed the proxy and is allowed to interact with it.
    address public owner;
    /// @notice The immutable address of the implementation contract.
    address immutable target;

    /**
     * @notice Deploys the proxy and the initial implementation logic.
     * @param _target The address of the implementation contract.
     * @param constructorData The ABI-encoded data for the implementation's constructor.
     */
    constructor(address _target, bytes memory constructorData) payable {
        owner = tx.origin; // Owner is the EOA that initiated the deployment transaction.
        target = _target;
        (bool success,) = target.delegatecall(constructorData); // Executes implementation's constructor logic
        require(success, "PROXY_CONSTRUCTOR_DELEGATECALL_FAILED");
    }

    /**
     * @notice Fallback function to delegate calls to the target implementation.
     * @dev Requires that the transaction origin matches the owner set during deployment.
     *      Forwards all ETH sent with the call.
     */
    fallback() external payable {
        require(tx.origin == owner, "PROXY_UNAUTHORIZED"); // Ensures only the original deployer EOA can call.
        (bool success,) = target.delegatecall(msg.data);
        require(success, "PROXY_FALLBACK_DELEGATECALL_FAILED");
    }
}
