FROM rust:buster as builder
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM debian:buster
LABEL maintainer="imlonghao"
COPY --from=builder /app/target/release/toshl2influxdb /app
RUN apt update && apt install -y --no-install-recommends libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini
ENTRYPOINT ["/tini", "--"]
CMD ["/app"]
