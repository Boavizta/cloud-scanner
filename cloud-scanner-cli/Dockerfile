# used for building releases - done once in a while
# and pretty slow but can handle multi platform

FROM rust:1.82.0 AS builder

RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
WORKDIR /app

COPY . .
RUN cargo build --release --bin cloud-scanner-cli

FROM debian:bookworm-slim AS runtime

COPY --from=builder /app/target/release/cloud-scanner-cli /usr/local/bin/

EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/cloud-scanner-cli"]
