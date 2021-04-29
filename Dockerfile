FROM gcr.io/distroless/static:nonroot

COPY --chown=nonroot:nonroot ./basiliq /app/basiliq

CMD ["/app/basiliq"]
