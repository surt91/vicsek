FROM rust:1.56 as builder
WORKDIR /usr/src/vicsek
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
# FIXME: there is not really a qt5 dependency, but this pulls in everything we need
# and I did not yet figure out what exactly we do need
RUN apt-get update && apt-get install -y libqt5gui5 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/vicsek/target/release/vicsek /usr/local/bin/vicsek
ENTRYPOINT ["vicsek"]