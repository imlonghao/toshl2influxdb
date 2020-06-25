FROM rust:buster as builder
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM debian:buster
LABEL maintainer="imlonghao"
COPY --from=builder /app/target/release/toshl2influxdb /app
RUN apt update && apt install -y --no-install-recomments libssl-dev && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["/app"]
