FROM rustlang/rust:nightly-alpine AS build

WORKDIR /usr/src/snakeware_test
COPY . .
RUN apk add --no-cach musl-dev && cargo install --path .

FROM rustlang/rust:nightly-alpine
COPY --from=build /usr/local/cargo/bin/snakeware_test /usr/local/bin/snakeware_test
ENTRYPOINT ["snakeware_test"]