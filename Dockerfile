# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

FROM rust:1.55 as build-env
WORKDIR /app
ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/ram-stress /
CMD ["./ram-stress"]