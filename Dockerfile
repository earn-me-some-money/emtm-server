FROM rust:1.35 as build
ARG origin
WORKDIR /usr/src/emtm

COPY . .

# change source for China
RUN mkdir -p $HOME/.cargo;
RUN if [ "$origin" = "cn" ]; then cp crate-io-ustc $HOME/.cargo/config; fi
# RUN if [ "$origin" = "cn" ]; then export http_proxy="http://127.0.0.1:8088"; export HTTP_PROXY="http://127.0.0.1:8088"; export https_proxy="http://127.0.0.1:8088"; export HTTPS_PROXY="http://127.0.0.1:8088"; echo $HTTP_PROXY; fi
RUN ls -la $HOME/.cargo;

RUN cargo update
RUN cargo build --release

CMD ["./target/release/emtm-web"]

FROM alpine:3.7
RUN apk add --no-cache mysql-client
# copy the build artifact from the build stage
COPY --from=build /usr/src/emtm/target/release/emtm-web .

# set the startup command to run your binary
CMD ["./emtm-web"]

