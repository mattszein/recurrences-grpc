FROM rust:1.71.0-alpine3.18 AS base
RUN apk add --no-cache musl-dev protoc protobuf-dev

ENV APP_HOME /recurrences_grpc
RUN mkdir $APP_HOME
WORKDIR $APP_HOME
COPY . $APP_HOME

FROM base as development
RUN cargo build
CMD ["cargo run"]

FROM base as builder
RUN cargo build --release
CMD ["cargo run --release"]

FROM alpine:3.18 as production 
WORKDIR /usr/local/bin/
COPY --from=builder ./recurrences_grpc/target/release/recurrences-server .
CMD ["/usr/local/bin/recurrences-server"]
