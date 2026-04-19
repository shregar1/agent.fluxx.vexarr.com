# Fluxx OpenTelemetry Collector Distribution
# Pre-configured collector that routes all telemetry to a Fluxx agent.
#
# Build:
#   docker build -t fluxx/otelcol .
#
# Run:
#   docker run -e FLUXX_ENDPOINT=host.docker.internal:4317 fluxx/otelcol

FROM otel/opentelemetry-collector-contrib:latest

COPY otelcol-fluxx.yaml /etc/otelcol-contrib/config.yaml

ENV FLUXX_ENDPOINT=localhost:4317
ENV FLUXX_HTTP_ENDPOINT=http://localhost:3000
ENV FLUXX_API_KEY=

EXPOSE 4317 4318 8888 13133

ENTRYPOINT ["/otelcol-contrib"]
CMD ["--config", "/etc/otelcol-contrib/config.yaml"]
