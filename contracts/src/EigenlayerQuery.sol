// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {console} from "forge-std/console.sol";

import "../lib/coprocessor-base-contract/src/CoprocessorAdapter.sol";

contract EigenlayerQuery is CoprocessorAdapter {
    mapping(bytes32 => bool) public queriesInProgress;
    mapping(address => mapping(address =>uint256)) public operatorBalances;
    
    constructor(address _taskIssuerAddress, bytes32 _machineHash)
        CoprocessorAdapter(_taskIssuerAddress, _machineHash)
    {}

    function queryOperatorBalance(address operator, address erc20) external {
        bytes32 blockHash = blockhash(block.number - 1);
        bytes memory payload = abi.encode(blockHash, operator, erc20);
        bytes32 payloadHash = keccak256(payload);
        
        require(!queriesInProgress[payloadHash], "query is already in progress");
        queriesInProgress[payloadHash] = true;

        callCoprocessor(payload);
    }

    function handleNotice(bytes32 payloadHash, bytes memory notice) internal override {
        require(queriesInProgress[payloadHash], "no query in progress for received notice");
        
        (bytes32 _blockHash, address operator, address erc20, uint256 balance) = abi.decode(
            notice,
            (bytes32, address, address, uint256 )
        );

        delete queriesInProgress[payloadHash];
        operatorBalances[operator][erc20] = balance;
    }

    function operatorBalance(address operator, address erc20) public view returns (uint256) {
        return operatorBalances[operator][erc20];
    }
}
