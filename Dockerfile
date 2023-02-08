FROM rust:slim
COPY ./target/release/account-info-integration ./target/release/account-info-integration
ENTRYPOINT ["./target/release/account-info-integration"]