# Running integration tests
Start a mock IPFS server, and set the `TEST_IPFS_SERVER` to the socket address of the *API server*.

E.g with Kubo (installed in the devcontainer)
```console
$ ipfs daemon --init --init-profile=test &
Initializing daemon...
...
API server listening on /ip4/127.0.0.1/tcp/11111
...
Daemon is ready

$ ipfs_daemon_pid=$!
$ TEST_IPFS_SERVER=127.0.0.1:11111 cargo test
...
$ kill -SIGINT "$ipfs_daemon_pid"
```


# Sample Hardhat Project

This project demonstrates a basic Hardhat use case. It comes with a sample contract, a test for that contract, and a script that deploys that contract.

Try running some of the following tasks:

```shell
npx hardhat help
npx hardhat test
REPORT_GAS=true npx hardhat test
npx hardhat node
npx hardhat run scripts/deploy.ts
```
