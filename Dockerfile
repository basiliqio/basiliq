FROM gcr.io/distroless/static:nonroot

COPY --chown=nonroot:nonroot ./target/x86_64-unknown-linux-musl/release/basiliq /app/basiliq

CMD ["/app/basiliq"]
