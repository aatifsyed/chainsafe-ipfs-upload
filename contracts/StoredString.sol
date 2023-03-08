// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.19;

contract StoredString {
    string public stored_string;

    constructor(string memory initial_string) {
        stored_string = initial_string;
    }
}
