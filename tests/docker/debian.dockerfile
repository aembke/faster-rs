# https://github.com/docker/for-mac/issues/5548#issuecomment-1029204019
FROM rust:1.80-slim-bullseye

WORKDIR /project

RUN USER=root apt-get update && apt-get install -y mlocate build-essential libc++-dev libssl-dev dnsutils curl gnupg2 wget ca-certificates apt-transport-https pkg-config autoconf automake cmake git vim linux-perf libaio-dev uuid-dev libtbb-dev
# RUN USER=root apt-get update && apt-get install -y mlocate build-essential libssl-dev dnsutils curl gnupg2 wget ca-certificates apt-transport-https pkg-config autoconf automake cmake git vim linux-perf g++-7 libaio-dev uuid-dev libtbb-dev

COPY tests/docker/clang.list /etc/apt/sources.list.d/clang.list
RUN USER=root wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - && apt-get update && apt-get install -y libllvm-15-ocaml-dev libllvm15 llvm-15 llvm-15-dev llvm-15-doc llvm-15-examples llvm-15-runtime clang-15 lldb-15 lld-15



# For debugging
RUN cargo --version && rustc --version
RUN rustup component add clippy && rustup install nightly