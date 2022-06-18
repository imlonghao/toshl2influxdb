FROM rust:buster as builder
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM debian:bullseye
LABEL maintainer="imlonghao"
COPY --from=builder /app/target/release/toshl2influxdb /app
RUN apt update && apt install -y --no-install-recommends libssl-dev ca-certificates tini && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["/app"]
