# Running integration tests
Start a mock IPFS server, and set the `TEST_IPFS_SERVER` to the socket address of the *API server*.

E.g with Kubo (installed in the devcontainer)
```console
$ ipfs daemon --init --init-profile=test & ipfs_daemon_pid=$!
Initializing daemon...
...
API server listening on /ip4/127.0.0.1/tcp/11111
...
Daemon is ready
$ anvil & ethereum_daemon_pid=$!
...
Listening on 127.0.0.1:22222
$ TEST_IPFS_SERVER=127.0.0.1:11111 TEST_ETHEREUM_GATEWAY=http://127.0.0.1:22222 cargo test
...
$ kill -SIGINT "$ipfs_daemon_pid"
```
