FROM rust:1.35 as build
ARG origin

RUN mkdir -p $HOME/.cargo;
RUN if [ "$origin" = "cn" ]; then cp crate-io-china $HOME/.cargo/config; fi
# RUN if [ "$origin" = "cn" ]; then export http_proxy="http://127.0.0.1:8088"; export HTTP_PROXY="http://127.0.0.1:8088"; export https_proxy="http://127.0.0.1:8088"; export HTTPS_PROXY="http://127.0.0.1:8088"; echo $HTTP_PROXY; fi
RUN ls -la $HOME/.cargo;

RUN cargo install diesel_cli --no-default-features --features mysql

WORKDIR /usr/src/emtm

RUN USER=root cargo new --bin emtm-web
RUN USER=root cargo new --lib emtm-db
RUN USER=root cargo new --lib emtm-verify
WORKDIR /usr/src/emtm/emtm-web
COPY ./emtm-web/Cargo.toml ./Cargo.toml
WORKDIR /usr/src/emtm/emtm-db
COPY ./emtm-db/Cargo.toml ./Cargo.toml
WORKDIR /usr/src/emtm/emtm-verify
COPY ./emtm-verify/Cargo.toml ./Cargo.toml


WORKDIR /usr/src/emtm

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

WORKDIR /usr/src/emtm/emtm-web
COPY ./emtm-web/* .
WORKDIR /usr/src/emtm/emtm-db
COPY ./emtm-db/* .
WORKDIR /usr/src/emtm/emtm-verify
COPY ./emtm-verify/* .

WORKDIR /usr/src/emtm
COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/emtm-web"]

