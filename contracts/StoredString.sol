// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.19;

/// TODO(aatifsyed): add tests for the smart contract, and test with foundry tooling
contract StoredString {
    string public stored_string;

    constructor(string memory initial_string) {
        stored_string = initial_string;
    }
}
