FROM rust:1.77.0-alpine AS builder

RUN wget https://github.com/upx/upx/releases/download/v4.2.2/upx-4.2.2-amd64_linux.tar.xz && \
    tar xf upx-4.2.2-amd64_linux.tar.xz

RUN apk add musl-dev libressl-dev

COPY ./ /app

WORKDIR /app

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release && \
    /upx-4.2.2-amd64_linux/upx --best --lzma /app/target/release/online-shop

FROM scratch

COPY --from=builder /app/target/release/online-shop /app
COPY --from=builder /app/templates /templates

CMD ["/app"]
