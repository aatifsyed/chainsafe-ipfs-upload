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

## Running smart contract stuff
```console
$ npx hardhat compile
...
$ npx hardhat node --port=0 &
Started HTTP and WebSocket JSON-RPC server at http://0.0.0.0:22222/
...
$ hardhat_daemon_pid=$!
$ npx hardhat run --network localhost scripts/deploy-box.js
...
Box deployed to: 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
$ TEST_ETHEREUM_SERVER=0.0.0.0:22222 cargo test
$ kill -SIGINT "$hardhat_daemon_pid"
```