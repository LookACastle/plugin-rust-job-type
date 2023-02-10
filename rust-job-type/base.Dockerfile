FROM rust:1.60-slim-buster

WORKDIR /src/rust_wrapper

RUN cargo init
COPY rust_wrapper/Cargo.toml rust_wrapper/Cargo.lock /src/rust_wrapper/
RUN cargo build

COPY rust_wrapper/src /src/rust_wrapper/src/
RUN cargo check && cargo clean
RUN rm -rf /src/rust_wrapper/src/handler

CMD /src/rust_wrapper/target/debug/rust_wrapper < /dev/null
LABEL racetrack-component="job"
