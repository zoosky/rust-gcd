FROM rust:1.75 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release -p web

FROM debian:bullseye-slim
COPY --from=builder /usr/src/app/target/release/web /usr/local/bin/
EXPOSE 8080
CMD ["web"]