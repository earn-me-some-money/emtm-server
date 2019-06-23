FROM rust:1.35 as build
ARG origin
WORKDIR /usr/src/emtm

COPY . .

RUN mkdir -p $HOME/.cargo;
RUN if [ "$origin" = "cn" ]; then cp crate-io-china $HOME/.cargo/config; fi
# RUN if [ "$origin" = "cn" ]; then export http_proxy="http://127.0.0.1:8088"; export HTTP_PROXY="http://127.0.0.1:8088"; export https_proxy="http://127.0.0.1:8088"; export HTTPS_PROXY="http://127.0.0.1:8088"; echo $HTTP_PROXY; fi
RUN ls -la $HOME/.cargo;

WORKDIR /usr/src/emtm/emtm-db
RUN cargo install diesel_cli --no-default-features --features mysql

WORKDIR /usr/src/emtm
# change source for China

RUN cargo build --release

ENTRYPOINT ["./target/release/emtm-web"]

