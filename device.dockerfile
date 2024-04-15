FROM rust:1.77 as BUILDER

WORKDIR /app

RUN apt-get update 
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

# build shared
COPY Cargo.* ./
COPY shared_model/Cargo.* ./shared_model/
COPY device_operator/Cargo.* ./device_operator/
RUN mkdir ./shared_model/src && echo 'fn main() { println!("Dummy!"); }' > ./shared_model/src/lib.rs
RUN mkdir ./device_operator/src && echo 'fn main() { println!("Dummy!"); }' > ./device_operator/src/main.rs

RUN cargo build --all --target x86_64-unknown-linux-musl || true

# # Install real code bin
RUN rm -rf ./device_operator/src/ && rm -rf ./shared_model/src/
COPY shared_model/src ./shared_model/src
COPY device_operator/src ./device_operator/src
RUN touch -a -m ./device_operator/src/main.rs && touch -a -m ./shared_model/src/lib.rs
RUN cargo build --all --target x86_64-unknown-linux-musl
RUN ls /app/target/x86_64-unknown-linux-musl/debug/

# Make end images
FROM alpine:latest
COPY --from=BUILDER /app/target/x86_64-unknown-linux-musl/debug/hermes-device .
USER 1000
CMD ["./hermes-device"]
