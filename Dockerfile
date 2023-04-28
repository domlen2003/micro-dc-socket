FROM rust:latest AS build

# create a new empty shell project
RUN USER=root cargo new --bin micro_dc_socket
WORKDIR /micro_dc_socket

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/micro_dc_socket*
RUN cargo build --release

# our final base
FROM gcr.io/distroless/cc

# copy the build artifact from the build stage
COPY --from=build /micro_dc_socket/target/release/micro_dc_socket .

# set the startup command to run your binary
CMD ["./micro_dc_socket"]