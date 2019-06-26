FROM rust:1.35 as build
ARG origin

RUN mkdir -p $HOME/.cargo;
RUN if [ "$origin" = "cn" ]; then cp crate-io-china $HOME/.cargo/config; fi
# RUN if [ "$origin" = "cn" ]; then export http_proxy="http://127.0.0.1:8088"; export HTTP_PROXY="http://127.0.0.1:8088"; export https_proxy="http://127.0.0.1:8088"; export HTTPS_PROXY="http://127.0.0.1:8088"; echo $HTTP_PROXY; fi
RUN ls -la $HOME/.cargo;

RUN cargo install diesel_cli --no-default-features --features mysql

WORKDIR /usr/src/emtm

# Cache optimization
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./emtm-web/Cargo.toml ./emtm-web/Cargo.toml
COPY ./emtm-verify/Cargo.toml ./emtm-verify/Cargo.toml
COPY ./emtm-db/Cargo.toml ./emtm-db/Cargo.toml
RUN mkdir emtm-db/src \
    && echo "fn main() {}" > emtm-db/src/main.rs \
    && mkdir emtm-verify/src \
    && echo "fn main() {}" > emtm-verify/src/main.rs \
    && mkdir emtm-web/src \
    && echo "fn main() {}" > emtm-web/src/main.rs

RUN cargo build --release 
RUN rm -rf emtm-db emtm-verify emtm-web

COPY ./emtm-db ./emtm-db
COPY ./emtm-verify ./emtm-verify
COPY ./emtm-web ./emtm-web
COPY ./scripts ./scripts

RUN cargo build --release

RUN mkdir bin && mv ./target/release/emtm-web ./bin
# smaller image
RUN rm -rf target

ENTRYPOINT ["./bin/emtm-web"]
