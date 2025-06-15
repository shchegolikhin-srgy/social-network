FROM rust:1.70

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 4002


CMD ["./target/release/auth-service"]
