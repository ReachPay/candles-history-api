FROM rust:slim
COPY ./target/release/orders-rest-api ./target/release/orders-rest-api
ENTRYPOINT ["./target/release/orders-rest-api"]