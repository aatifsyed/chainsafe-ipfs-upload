FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

###############################################
# Install kubo to use as a local ipfs network #
###############################################

# https://docs.ipfs.tech/install/command-line/#install-official-binary-distributions
RUN : \
    && wget https://dist.ipfs.tech/kubo/v0.18.1/kubo_v0.18.1_linux-amd64.tar.gz \
    && tar -xvzf kubo_v0.18.1_linux-amd64.tar.gz \
    && rm kubo_v0.18.1_linux-amd64.tar.gz \
    && cd kubo \
    && bash install.sh \
    && ipfs --version \
    && cd - \
    && rm -rf kubo

##################################
# Install smart contract tooling #
##################################

# Toolchain and local ethereum node
RUN : \
    && curl -L https://foundry.paradigm.xyz \
    | bash \
    && foundryup

# Solidity compiler
RUN : \
    && curl --location https://github.com/ethereum/solidity/releases/download/v0.8.19/solc-static-linux \
    > /usr/local/bin/solc \
    && chmod +x /usr/local/bin/solc