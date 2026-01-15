# Setup Guide

## Quick Start

1. **Configure the API key** (choose one method):

    **Option A: Config file**

    ```bash
    # Copy example config
    cp config/config.example.toml config/config.toml

    # Edit and set your API key
    nano config/config.toml
    ```

    **Option B: Environment variable**

    ```bash
    export API_KEY="$(openssl rand -base64 32)"
    ```

2. **Start MongoDB**

    ```bash
    docker run -d -p 27017:27017 --name mongodb mongo:latest
    # OR use your existing MongoDB instance
    ```

3. **Build and run**

    ```bash
    cargo build --release
    cargo run --release
    ```

4. **Test the API**

    ```bash
    # Health check
    curl http://localhost:8081/api/v1/health

    # Get networks
    curl http://localhost:8081/api/v1/networks

    # Update config (requires API key)
    curl -X PUT http://localhost:8081/api/v1/config \
      -H "Content-Type: application/json" \
      -H "X-API-Key: your-api-key-here" \
      -d '{"max_amount_usd": 1000.0}'
    ```

## Documentation

-   **[API_EXAMPLES.md](API_EXAMPLES.md)** - Complete API reference with curl examples
-   **[AUTHENTICATION.md](AUTHENTICATION.md)** - Authentication guide
-   **[config/config.example.toml](config/config.example.toml)** - Example configuration file

## API Endpoints

All endpoints are under `/api/v1`:

### Public Endpoints

-   `GET /health` - Health check
-   `GET /config` - Get configuration
-   `GET /networks` - List all networks
-   `GET /networks/{chain_id}` - Get network by chain ID
-   `GET /paths` - List all paths
-   `GET /pools` - List all pools
-   `GET /tokens` - List all tokens

### Protected Endpoints (require API key)

-   `PUT /config` - Update configuration
-   `POST /networks` - Create a new network
-   `PUT /networks/{chain_id}` - Update network fields
-   `PUT /networks/{chain_id}/factories` - Update both V2 factory fees and Aero factory addresses together
-   `DELETE /networks/{chain_id}` - Delete a network
-   `POST /paths` - Create a new path
-   `PUT /paths/{id}` - Update an existing path
-   `POST /pools` - Create a new pool
-   `PUT /pools/{id}` - Update an existing pool
-   `DELETE /networks/{chain_id}` - Delete a network

## Configuration

See `config/config.example.toml` for all available options.

Key settings:

-   `server.api_key` - API key for protected endpoints
-   `server.port` - Server port (default: 8081)
-   `database.uri` - MongoDB connection string
-   `cors.allowed_origins` - CORS allowed origins

## Development

Run with debug logging:

```bash
cargo run -- --log-level debug
```

## Production Checklist

-   [ ] Set a strong API key
-   [ ] Configure MongoDB with authentication
-   [ ] Set up HTTPS/TLS
-   [ ] Configure CORS for your frontend domain
-   [ ] Review and secure all endpoints
-   [ ] Set up monitoring and logging
-   [ ] Configure firewall rules
