FROM rust:slim
COPY ./target/release/candles-history-api ./target/release/candles-history-api
ENTRYPOINT ["./target/release/candles-history-api"]