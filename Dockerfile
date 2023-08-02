FROM rust:1.71.0-alpine3.18
RUN apk add --no-cache musl-dev

ENV APP_HOME /recurrences_grpc
RUN mkdir $APP_HOME
WORKDIR $APP_HOME
COPY . $APP_HOME

CMD ["ls"]
