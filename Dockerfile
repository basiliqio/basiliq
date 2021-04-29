FROM gcr.io/distroless/static:nonroot

COPY --chown=nonroot:nonroot ./target/release/basiliq /app/basiliq

CMD ["/app/basiliq"]
