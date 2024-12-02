FROM amazonlinux:latest as builder

# Install necessary packages
RUN yum update -y && \
    yum install -y \
    openssl \
    openssl-devel \
    gcc \
    make \
    && yum clean all

# Dockerfileの場所はapiなので、その親をコピーして親の名前のディレクトリをworkdirにする。これでcommonが参照できる
COPY .. /bento-backend
WORKDIR /bento-backend

# rustをインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# pathを通す
ENV PATH="/root/.cargo/bin:${PATH}"

# buildする
RUN cargo build --release --manifest-path=api/Cargo.toml

FROM amazonlinux:latest

# linuxにapiだけコピー
COPY --from=builder /bento-backend/target/release/api /usr/local/bin/api

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/api"]