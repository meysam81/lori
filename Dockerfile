FROM rust:1-alpine AS builder

WORKDIR /build

RUN apk add --update openssl-dev musl-dev

COPY . .

RUN cargo install --path . --root /build/out


FROM scratch AS runner

COPY --from=builder /build/out/bin/ruby /usr/local/bin/ruby

ENTRYPOINT ["/usr/local/bin/ruby"]
