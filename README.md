# Running the binary
## Run in devcontainer
Either run the commands from within VSCode (which should pick up the devcontainer) or
```bash
docker build --tag=scratch --file=.devcontainer/Dockerfile .devcontainer
docker run \
    --interactive \
    --rm \
    --tty \
    --user="$(id -u):$(id -g)" \
    --volume="$PWD:/tmp/chainsafe-ipfs-upload" \
    --workdir=/tmp/chainsafe-ipfs-upload \
    scratch
```

## Start an IPFS node
- Take note of the IP and port (the port will be random)
```console
$ ipfs daemon --init --init-profile=test & ipfs_daemon_pid=$!
Initializing daemon...
...
API server listening on /ip4/127.0.0.1/tcp/11111
...
Daemon is ready
$ ipfs_daemon_address=127.0.0.1:11111
```
## Start an ethereum node
- Take note of the ip and port
- Take note of an account's private key
```
$ anvil --accounts=1 & ethereum_daemon_pid=$!
...
Private Keys
==================

(0) 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
...
Listening on 127.0.0.1:8545
$ ethereum_daemon_url=http://127.0.0.1:8545
$ ethereum_wallet_secret_key=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```
## Run the binary
```
$ cargo run -- --file=README.md --ipfs="$ipfs_daemon_address" --ethereum="$ethereum_daemon_url" --secret-key="$ethereum_wallet_secret_key"
...
2023-03-08T07:28:28.421663Z  INFO chainsafe_ipfs_upload: uploaded file to ipfs cid=QmUVS8yEji7RofnuQ6ikq6rT43tz6mTNiBA67MWdW6934V
2023-03-08T07:28:35.435504Z  INFO chainsafe_ipfs_upload: stored cid in new ethereum contract address=b7f8bc63bbcad18155201308c8f3540b07f84f5e
b7f8bc63bbcad18155201308c8f3540b07f84f5e
```
## Cleanup the daemons
```bash
kill -SIGINT "$ipfs_daemon_pid"
kill -SIGINT "$ethereum_daemon_pid"
```

# Running integration tests
The integration tests currently rely on an IPFS server running.
Run as above (the port will be random):
```console
$ ipfs daemon --init --init-profile=test & ipfs_daemon_pid=$!
Initializing daemon...
...
API server listening on /ip4/127.0.0.1/tcp/11111
...
Daemon is ready
$ ipfs_daemon_address=127.0.0.1:11111
$ TEST_IPFS_SERVER=$ipfs_daemon_address cargo test
$ kill -SIGINT "$ipfs_daemon_pid"
```
