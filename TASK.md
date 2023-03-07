# Technical Exercise - Rust

_ChainSafe Systems Protocol Engineering_

Scope: Rust, Web3

### IPFS Upload & Contract Registry

##### Simple Summary

Upload a file to IPFS and store the resulting CID in a smart contract on an EVM-based blockchain.

##### Requirements

Implement a command-line tool that satisfies the following requirements.

1. The command-line tool accepts a local file as an argument.
2. It uploads the file to the IPFS network
  - use an IPFS library to handle the upload
  - uploading the file should give you a CID for the next step
3. It stores the CID in a smart contract
  - you are free to use any EVM-based network, preferably an Ethereum testnet or a local environment (e.g., `ganache`)
  - the structure of the contract is not essential for this task as long as the CID is stored somehow

##### Resources

Deploying Smart Contracts:

- <https://ethereumdev.io/deploying-your-first-smart-contract>
- <https://ethereum.org/en/developers/docs/smart-contracts/deploying>
- <https://docs.openzeppelin.com/learn/deploying-and-interacting>

Running IPFS Nodes:

- <https://docs.ipfs.io/how-to/command-line-quick-start>

Rust libraries to consider:

- <https://github.com/ferristseng/rust-ipfs-api>
- <https://github.com/gakonst/ethers-rs>
- <https://github.com/tomusdrw/rust-web3>

### Further Instructions

##### Goals

This exercise aims to understand better how you approach software development. While we are concerned with the correctness of the implementation, we will also be paying attention to the overall layout of the code and the contents of the repo. Consider the necessary scripts, testing, and documentation for your submission to be deemed a complete piece of well-engineered software.

##### Duration
You will be given approximately one week to complete the exercise. All exercises are designed to take no more than 4 hours. Incomplete submissions will be accepted and reviewed to the best of our ability.

##### Submission

All assignments must be submitted as a Git repo. In the case of a private Github repository, please invite the following users to your repository: `lemmih`, `LesnyRumcajs`, `q9f`, `lerajk`.

##### Feedback

The interviewer will review the submission and collect feedback for every submission. This feedback will be shared via email or through a technical interview.
