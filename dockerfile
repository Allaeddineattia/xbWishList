FROM rust:1.71.0 as builder
RUN USER=root cargo new --bin xbWishlist
WORKDIR ./xbWishlist
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD ./src ./src

RUN rm ./target/release/deps/xb_wish_list*
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /xbWishlist/target/release/xb_wish_list /usr/local/bin/xb_wish_list

EXPOSE 8080

ENV MONGODB_URL mongodb://my_mongodb:27017

ENV WEB_SERVICE_URL 0.0.0.0:8080

ENV RUST_BACKTRACE=1

WORKDIR /usr/local/bin/

CMD ["xb_wish_list"]
