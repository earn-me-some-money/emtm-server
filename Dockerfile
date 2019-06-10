FROM rust:1.35 as build

WORKDIR /usr/src/emtm

COPY . .
RUN cargo build --release

CMD ["./target/release/emtm-web"]

FROM rust:1.35

# copy the build artifact from the build stage
COPY --from=build /usr/src/emtm/target/release/emtm .

# set the startup command to run your binary
CMD ["./emtm"]

