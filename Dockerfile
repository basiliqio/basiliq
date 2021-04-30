FROM gcr.io/distroless/static:nonroot

COPY --chown=nonroot:nonroot ./target/x86_64-unknown-linux-musl/release/basiliq /app/basiliq

EXPOSE 8000 8443

ENTRYPOINT ["/app/basiliq"]
