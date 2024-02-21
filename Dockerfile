FROM rust:1.76.0-alpine3.19 AS builder

RUN apk add musl-dev && \
    wget https://github.com/upx/upx/releases/download/v4.2.2/upx-4.2.2-amd64_linux.tar.xz && \
    tar xf upx-4.2.2-amd64_linux.tar.xz

COPY ./ /app

WORKDIR /app

RUN cargo build --release && \
    /upx-4.2.2-amd64_linux/upx --best --lzma /app/target/release/online-shop


FROM scratch

COPY --from=builder /app/target/release/online-shop /app

EXPOSE 80

CMD ["/app"]
