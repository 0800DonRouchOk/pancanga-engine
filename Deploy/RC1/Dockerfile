FROM rust:1.85-bookworm AS build

WORKDIR /app
COPY . .

WORKDIR /app/03_Source/rust
RUN cargo build --release -p pancanga-engine --example rc1_experience

FROM debian:bookworm-slim

RUN useradd --create-home --shell /usr/sbin/nologin appuser

COPY --from=build /app/03_Source/rust/target/release/examples/rc1_experience /usr/local/bin/rc1_experience

USER appuser
EXPOSE 8080

CMD ["/usr/local/bin/rc1_experience"]
