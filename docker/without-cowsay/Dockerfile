FROM rust:1.64

WORKDIR /usr/src/build-scripts-tutorial
COPY src src
COPY build.rs .
COPY Cargo.toml .

RUN cargo install --path .

CMD ["build-scripts-tutorial"]