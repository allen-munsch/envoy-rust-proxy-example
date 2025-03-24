FROM envoyproxy/envoy:v1.28.0

COPY envoy-wasm-filter.wasm /etc/envoy/
COPY envoy.yaml /etc/envoy/envoy.yaml

CMD ["/usr/local/bin/envoy", "-c", "/etc/envoy/envoy.yaml", "--service-cluster", "proxy"]