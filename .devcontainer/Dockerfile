# TODO(aatifsyed): multiarch
FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

###############################################
# Install kubo to use as a local ipfs network #
###############################################

# https://docs.ipfs.tech/install/command-line/#install-official-binary-distributions
RUN : \
    && curl --silent --show-error --location https://dist.ipfs.tech/kubo/v0.18.1/kubo_v0.18.1_linux-amd64.tar.gz \
    | tar --extract --verbose --ungzip \
    && cd kubo \
    && bash install.sh \
    && ipfs --version \
    && cd - \
    && rm -rf kubo

##################################
# Install smart contract tooling #
##################################

# Toolchain and local ethereum node
# We avoid the foundryup installer in favour of a standard installation so that `anvil` is guaranteed to be on PATH
# (important for ethers::utils::Anvil)
RUN : \
    && curl --silent --show-error --location https://github.com/foundry-rs/foundry/releases/download/nightly-f5e5eb52f62928c5b4daeec0fe963ad7f88cb464/foundry_nightly_linux_amd64.tar.gz \
    | tar --extract --verbose --ungzip --directory=/usr/local/bin --no-same-owner

# Solidity compiler
RUN : \
    && curl --silent --show-error --location https://github.com/ethereum/solidity/releases/download/v0.8.19/solc-static-linux \
    > /usr/local/bin/solc \
    && chmod +x /usr/local/bin/solc