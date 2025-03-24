# Envoy WASM Filter Deployment

This repository contains a Rust-based WebAssembly (WASM) filter for Envoy proxy along with deployment scripts and configuration.

## Project Structure

```
.
├── src/
│   └── lib.rs              # Your Rust WASM filter code
├── Cargo.toml              # Rust dependencies and build configuration
├── .cargo/
│   └── config.toml         # Cargo configuration for WASM builds
├── config/
│   └── envoy.yaml          # Envoy proxy configuration
├── docker-compose.yaml     # Docker services configuration
├── deploy.sh               # Deployment script
└── test-filter.sh          # Script to test the filter
```

## Filter Features

This Envoy WASM filter provides the following functionality:

- Adds custom headers to requests (`x-wasm-filter`)
- Sets upstream request timeout
- Adds metadata to responses including:
  - Request path
  - Filter version
  - Timestamp
- Logs request/response information
- Can be configured via Envoy configuration

## Deployment Instructions

1. Make sure you have the following prerequisites installed:
   - Rust and Cargo
   - Docker and Docker Compose
   - curl (for testing)

2. Run the deployment script:
   ```bash
   chmod +x deploy.sh
   ./deploy.sh
   ```

3. Test the filter:
   ```bash
   chmod +x test-filter.sh
   ./test-filter.sh
   ```

4. To stop the service:
   ```bash
   docker-compose down
   ```

## Configuration

You can customize the filter behavior by modifying the JSON configuration in `config/envoy.yaml`:

```yaml
configuration:
  "@type": "type.googleapis.com/google.protobuf.StringValue"
  value: |
    {
      "header_name": "x-wasm-filter",
      "header_value": "envoy-rust-deployed",
      "upstream_timeout_ms": 3000
    }
```

## Accessing Services

- Main service: http://localhost:10000
- Envoy admin interface: http://localhost:9901
- Backend service (httpbin): http://localhost:10000/anything

## Troubleshooting

If you encounter issues, check the logs:

```bash
docker-compose logs envoy
```

Make sure your WASM filter is being loaded correctly by checking the Envoy startup logs.
