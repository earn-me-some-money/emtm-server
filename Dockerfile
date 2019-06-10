FROM rust:1.35

WORKDIR /usr/src/emtm
COPY . .
RUN cargo install --path .

CMD ["emtm"]

