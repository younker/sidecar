FROM rust:1.44 as build

WORKDIR /usr/src/rust-server

# This funky approach is a workaround for cargo not supporting a --deps-only
# build flag. For more, see:
# https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
COPY Cargo.toml Cargo.toml

RUN mkdir src

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/myapp*

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/rust-server", "7878"]
