FROM rust:1.64

RUN apt-get update && apt-get -y install cowsay
ENV PATH="${PATH}:/usr/games"

WORKDIR /usr/src/build-scripts-tutorial
COPY src src
COPY build.rs .
COPY Cargo.toml .

RUN cargo install --path .

CMD ["build-scripts-tutorial"]