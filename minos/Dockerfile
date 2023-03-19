FROM rust:1.67 as builder
WORKDIR /usr/src/minos
COPY . .
RUN cargo install --path .

CMD ["minos"]
