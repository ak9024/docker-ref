FROM rust:slim-bullseye AS builder

WORKDIR /usr/src/rust

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=builder /usr/local/cargo/bin/rust /usr/local/bin/rust

CMD ["rust"]
