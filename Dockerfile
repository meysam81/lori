FROM curlimages/curl AS entrypoint

ARG DUMB_INIT='https://github.com/Yelp/dumb-init/releases/download/v1.2.5/dumb-init_1.2.5_x86_64'

USER root
RUN curl -sSLo /usr/local/bin/dumb-init ${DUMB_INIT} && \
    chmod +x /usr/local/bin/dumb-init

FROM rust:1-alpine AS builder

WORKDIR /build

RUN apk add --update openssl-dev musl-dev

COPY . .

RUN cargo install --path . --root /build/out


FROM scratch AS runner

COPY --from=builder /build/out/bin/lori /usr/local/bin/lori
COPY --from=entrypoint /usr/local/bin/dumb-init /usr/local/bin/dumb-init

ENTRYPOINT ["/usr/local/bin/dumb-init", "--"]
CMD ["/usr/local/bin/lori"]
