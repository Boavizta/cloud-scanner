# used for building releases - done once in a while
# and pretty slow but can handle multi platform

FROM rust:1.86.0 AS builder

RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
WORKDIR /app

COPY . .
RUN cargo build --release --bin cloud-scanner-cli

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/cloud-scanner-cli /usr/local/bin/

EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/cloud-scanner-cli"]
