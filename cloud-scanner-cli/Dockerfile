# used for building releases - done once in a while
# and pretty slow but can handle multi platform

FROM rust:1.82.0 AS builder

RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
WORKDIR /app

COPY . .
RUN cargo build --release --bin cloud-scanner-cli

FROM alpine AS runtime
#update libcrypto3 libssl3 to fix security issues
RUN apk update && apk add --upgrade libcrypto3 libssl3

#RUN addgroup -S myuser && adduser -S myuser -G myuser

COPY --from=builder /app/target/release/cloud-scanner-cli /usr/local/bin/
#USER myuser

EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/cloud-scanner-cli"]
