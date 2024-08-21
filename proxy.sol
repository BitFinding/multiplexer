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
        require(msg.sender == owner); // Ownership check
        (bool success,) = target.delegatecall(msg.data);
        require(success, "DELEGATECALL_FAILED");
    }
}