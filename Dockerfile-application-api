FROM rust:latest AS builder

WORKDIR /www

COPY ./ .

RUN apt-get update  \
    && update-ca-certificates  \
    && apt-get install -y pkg-config libssl-dev \
    && cargo build --release


FROM debian:stable-slim AS runtime

WORKDIR /www

ENV TZ=Asia/Shanghai

COPY --from=builder /www/target/release/application-api ./app

RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone && date

CMD ["/www/app"]