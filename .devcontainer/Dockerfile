FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

RUN : \
    && env DEBIAN_FRONTEND=noninteractive apt update \
    && env DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/* \
    && protoc --version

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
