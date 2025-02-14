pragma solidity ^0.8.0;

contract proxy {
    address public owner;
    address immutable target;
    constructor(address _target, bytes memory constructorData) payable {
        owner = tx.origin;
        target = _target;
        (bool success,) = target.delegatecall(constructorData);
        require(success, "DELEGATECALL_FAILED");
    }
    fallback() external payable {
        require(tx.origin == owner); // Ownership check
        (bool success,) = target.delegatecall(msg.data);
        require(success, "DELEGATECALL_FAILED");
    }
}