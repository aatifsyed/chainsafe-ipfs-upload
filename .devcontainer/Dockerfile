FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# TODO(aatifsyed): not using iroh anymore - remove this
RUN : \
    && env DEBIAN_FRONTEND=noninteractive apt update \
    && env DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/* \
    && protoc --version

###############################################
# Install kubo to use as a local ipfs network #
###############################################

# TODO(aatifsyed): can we use e.g https://github.com/ipfs/js-ipfs instead?

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

#############################################
# Install hardhat, a local ethereum network #
#############################################

# Install latest LTS version of node
RUN : \
    && curl -fsSL https://deb.nodesource.com/setup_18.x \
    | bash - \ 
    && env DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends \
    nodejs \
    && rm -rf /var/lib/apt/lists/* \
    && npm --version
