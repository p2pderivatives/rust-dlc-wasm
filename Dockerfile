FROM rust

ENV WORKSPACE /workspace
WORKDIR ${WORKSPACE}
RUN rustup target add wasm32-unknown-unknown
RUN apt-get update && apt-get install -y git
RUN cargo install wasm-pack
RUN apt-get install -y clang
COPY . .
RUN cargo update
RUN cargo build
CMD [ "wasm-pack", "build" ]
