FROM rust:latest

WORKDIR /app

COPY Cargo* ./

COPY src/main.rs ./src/

RUN cargo fetch 

COPY . .

RUN cargo build --release --target-dir /usr/local/cargo

RUN cargo install diesel_cli

CMD ["bash", "entrypoint.sh"]
