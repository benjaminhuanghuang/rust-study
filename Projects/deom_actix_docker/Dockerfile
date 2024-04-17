FROM rust:1.57
WORKDIR /usr/src/actix_demo
COPY . .
RUN cargo install --path .
CMD ["actix_demo"]