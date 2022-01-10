FROM rust:latest

WORKDIR /app

COPY . .

# Cache download
RUN cargo fetch 

RUN cargo build --release

RUN cargo install diesel_cli

CMD ["bash", "entrypoint.sh"]