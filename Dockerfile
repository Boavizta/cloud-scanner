FROM rust:1.77 as chef
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin cloud-scanner-cli

FROM alpine AS runtime
#update libcrypto3 libssl3 to fix security issues
RUN apk update && apk add --upgrade libcrypto3 libssl3
#RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cloud-scanner-cli /usr/local/bin/
#USER myuser

EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/cloud-scanner-cli"]
