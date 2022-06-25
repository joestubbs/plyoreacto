# Image: jstubbs/plyoreacto

FROM rust:1.61 as builder

RUN USER=root apt-get update && apt-get install -y libzmq3-dev

RUN USER=root cargo new --bin plyoreacto
WORKDIR /plyoreacto

# copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# build and cache only the dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/plyoreacto*
RUN cargo build --release

# final base image
FROM debian:buster-slim

# still need to install zmq
RUN USER=root apt-get update && apt-get install -y libzmq3-dev

# copy the build artifact from the build stage
COPY --from=builder /plyoreacto/target/release/plyoreacto .

# set the startup command to run your binary
CMD ["./plyoreacto"]