FROM rust AS builder

WORKDIR /usr/src/quicksauce

COPY Cargo.toml Cargo.lock ./

COPY src ./src

COPY .env ./

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /the/workdir/quicksauce

COPY --from=builder /usr/src/quicksauce/target/release/quicksauce .

COPY .env ./

EXPOSE 8080

CMD ["./quicksauce"]