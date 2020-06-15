FROM rust:1.44 as build

WORKDIR /usr/src/rust-server

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/rust-server", "7878"]
