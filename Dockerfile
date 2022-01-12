FROM rust:latest

WORKDIR /app

COPY . .

# Cache download
RUN cargo fetch 

RUN cargo build --release --target-dir /usr/local/cargo

RUN cargo install diesel_cli

CMD ["bash", "entrypoint.sh"]
