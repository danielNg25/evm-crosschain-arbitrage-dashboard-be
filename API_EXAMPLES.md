# API Endpoints - cURL Examples

Base URL: `http://localhost:8081/api/v1`

## Authentication

Some endpoints require API key authentication. To use these endpoints, include the `X-API-Key` header in your request:

```bash
curl -X PUT http://localhost:8081/api/v1/config \
  -H "X-API-Key: your-api-key-here" \
  -H "Content-Type: application/json" \
  -d '{...}'
```

### Configuration

Set the API key in your `config/config.toml` file:

```toml
[server]
api_key = "your-secret-api-key-here"
```

Or set it via environment variable:

```bash
export API_KEY="your-secret-api-key-here"
```

**Note:** If no API key is configured, the protected endpoints will allow access (useful for development). In production, always set an API key.

### Frontend Usage

In JavaScript/TypeScript:

```javascript
fetch('http://localhost:8081/api/v1/config', {
    method: 'PUT',
    headers: {
        'Content-Type': 'application/json',
        'X-API-Key': 'your-api-key-here',
    },
    body: JSON.stringify({
        max_amount_usd: 2000.0,
        recheck_interval: 120,
    }),
});
```

---

## Health Check

### GET /health

Check if the API is running.

```bash
curl -X GET http://localhost:8081/api/v1/health
```

**Response:**

```json
{
    "status": "ok"
}
```

---

## Config Endpoints

### GET /config

Get the current configuration.

```bash
curl -X GET http://localhost:8081/api/v1/config
```

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439011",
    "max_amount_usd": 1000.0,
    "recheck_interval": 60,
    "created_at": 1234567890,
    "updated_at": 1234567890
}
```

### PUT /config

Update the configuration. **Requires API key authentication.**

```bash
curl -X PUT http://localhost:8081/api/v1/config \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key" \
  -d '{
    "max_amount_usd": 2000.0,
    "recheck_interval": 120
  }'
```

**Note:** The API key must be configured in your `config/config.toml` file under `[server]` section as `api_key = "your-secret-key"`, or set via the `API_KEY` environment variable.

**Request Body (all fields optional):**

```json
{
    "max_amount_usd": 2000.0,
    "recheck_interval": 120
}
```

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439011",
    "max_amount_usd": 2000.0,
    "recheck_interval": 120,
    "created_at": 1234567890,
    "updated_at": 1234567891
}
```

---

## Network Endpoints

### GET /networks

Get all networks.

```bash
curl -X GET http://localhost:8081/api/v1/networks
```

**Response:**

```json
[
    {
        "id": "507f1f77bcf86cd799439011",
        "chain_id": 1,
        "name": "Ethereum",
        "rpc": "https://eth-mainnet.g.alchemy.com/v2/...",
        "block_explorer": "https://etherscan.io",
        "created_at": 1234567890
    }
]
```

### GET /networks/{chain_id}

Get a specific network by chain ID.

```bash
curl -X GET http://localhost:8081/api/v1/networks/1
```

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439011",
    "chain_id": 1,
    "name": "Ethereum",
    "rpc": "https://eth-mainnet.g.alchemy.com/v2/...",
    "block_explorer": "https://etherscan.io",
    "created_at": 1234567890
}
```

### POST /networks

Create a new network. **Requires API key authentication.**

```bash
curl -X POST http://localhost:8081/api/v1/networks \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key" \
  -d '{
    "chain_id": 8453,
    "name": "Base",
    "rpcs": ["https://mainnet.base.org"],
    "websocket_urls": ["wss://mainnet.base.org"],
    "block_explorer": "https://basescan.org",
    "wrap_native": "0x4200000000000000000000000000000000000006",
    "min_profit_usd": 10.0,
    "v2_factory_to_fee": {
      "0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6": 3000
    },
    "aero_factory_addresses": ["0x420DD381b31aEf6683db6B902084cB0FFECe40Da"],
    "multicall_address": "0xcA11bde05977b3631167028862bE2a173976CA11",
    "max_blocks_per_batch": 1000,
    "wait_time_fetch": 100
  }'
```

**Request Body (all fields required except optional ones):**

```json
{
    "chain_id": 8453,
    "name": "Base",
    "rpcs": ["https://mainnet.base.org"],
    "websocket_urls": ["wss://mainnet.base.org"], // Optional
    "block_explorer": "https://basescan.org", // Optional
    "wrap_native": "0x4200000000000000000000000000000000000006",
    "min_profit_usd": 10.0,
    "v2_factory_to_fee": {
        // Optional
        "0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6": 3000
    },
    "aero_factory_addresses": ["0x420DD381b31aEf6683db6B902084cB0FFECe40Da"], // Optional
    "multicall_address": "0xcA11bde05977b3631167028862bE2a173976CA11", // Optional
    "max_blocks_per_batch": 1000,
    "wait_time_fetch": 100
}
```

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439012",
    "chain_id": 8453,
    "name": "Base",
    "rpc": "https://mainnet.base.org",
    "block_explorer": "https://basescan.org",
    "created_at": 1234567890
}
```

### PUT /networks/{chain_id}

Update an existing network. **Requires API key authentication.**

```bash
curl -X PUT http://localhost:8081/api/v1/networks/8453 \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key" \
  -d '{
    "name": "Base Mainnet",
    "min_profit_usd": 15.0,
    "rpcs": ["https://mainnet.base.org", "https://base.llamarpc.com"]
  }'
```

**Request Body (all fields optional, only include fields you want to update):**

```json
{
    "name": "Base Mainnet", // Optional
    "rpcs": ["https://mainnet.base.org", "https://base.llamarpc.com"], // Optional
    "websocket_urls": ["wss://mainnet.base.org"], // Optional
    "block_explorer": "https://basescan.org", // Optional
    "wrap_native": "0x4200000000000000000000000000000000000006", // Optional
    "min_profit_usd": 15.0, // Optional
    "v2_factory_to_fee": {
        // Optional
        "0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6": 3000
    },
    "aero_factory_addresses": ["0x420DD381b31aEf6683db6B902084cB0FFECe40Da"], // Optional
    "multicall_address": "0xcA11bde05977b3631167028862bE2a173976CA11", // Optional
    "max_blocks_per_batch": 1000, // Optional
    "wait_time_fetch": 100 // Optional
}
```

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439012",
    "chain_id": 8453,
    "name": "Base Mainnet",
    "rpc": "https://mainnet.base.org",
    "block_explorer": "https://basescan.org",
    "created_at": 1234567890
}
```

### PUT /networks/{chain_id}/factories

Update both V2 factory fees and Aero factory addresses together for a specific network. **Requires API key authentication.**

```bash
curl -X PUT http://localhost:8081/api/v1/networks/8453/factories \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key" \
  -d '{
    "v2_factory_to_fee": {
      "0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6": 3000,
      "0x420DD381b31aEf6683db6B902084cB0FFECe40Da": 500
    },
    "aero_factory_addresses": [
      "0x420DD381b31aEf6683db6B902084cB0FFECe40Da",
      "0x5C7BCd6E7De5423a257D81B442095A1a6ced35C5"
    ]
  }'
```

**Request Body:**

```json
{
    "v2_factory_to_fee": {
        "0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6": 3000,
        "0x420DD381b31aEf6683db6B902084cB0FFECe40Da": 500
    },
    "aero_factory_addresses": [
        "0x420DD381b31aEf6683db6B902084cB0FFECe40Da",
        "0x5C7BCd6E7De5423a257D81B442095A1a6ced35C5"
    ]
}
```

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439012",
    "chain_id": 8453,
    "name": "Base",
    "rpc": "https://mainnet.base.org",
    "block_explorer": "https://basescan.org",
    "created_at": 1234567890
}
```

### DELETE /networks/{chain_id}

Delete a network by chain_id. **Requires API key authentication.**

```bash
curl -X DELETE http://localhost:8081/api/v1/networks/8453 \
  -H "X-API-Key: your-secret-api-key"
```

**Response:**

-   **204 No Content** - Network deleted successfully

**Error Response (404 Not Found):**

```json
{
    "error": "Network with chain_id 8453 not found"
}
```

---

## Path Endpoints

### GET /paths

Get all paths.

```bash
curl -X GET http://localhost:8081/api/v1/paths
```

**Response:**

```json
[
  {
    "id": "507f1f77bcf86cd799439011",
    "paths": [...],
    "created_at": 1234567890,
    "updated_at": 1234567890
  }
]
```

### GET /paths/{id}

Get a specific path by ID.

```bash
curl -X GET http://localhost:8081/api/v1/paths/507f1f77bcf86cd799439011
```

**Response:**

```json
{
  "id": "507f1f77bcf86cd799439011",
  "paths": [...],
  "created_at": 1234567890,
  "updated_at": 1234567890
}
```

### GET /paths/anchor-token/{anchor_token}

Get paths by anchor token address.

```bash
curl -X GET http://localhost:8081/api/v1/paths/anchor-token/0x6B175474E89094C44Da98b954EedeAC495271d0F
```

**Response:**

```json
[
  {
    "id": "507f1f77bcf86cd799439011",
    "paths": [...],
    "created_at": 1234567890,
    "updated_at": 1234567890
  }
]
```

### GET /paths/chain/{chain_id}

Get paths by chain ID.

```bash
curl -X GET http://localhost:8081/api/v1/paths/chain/1
```

**Response:**

```json
[
  {
    "id": "507f1f77bcf86cd799439011",
    "paths": [...],
    "created_at": 1234567890,
    "updated_at": 1234567890
  }
]
```

### POST /paths

Create a new path. **Requires API key authentication.**

```bash
curl -X POST http://localhost:8081/api/v1/paths \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key" \
  -d '{
    "paths": [
      {
        "paths": [
          [
            {
              "pool": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
              "token_in": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
              "token_out": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
            }
          ]
        ],
        "chain_id": 1,
        "anchor_token": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
      }
    ]
  }'
```

**Request Body:**

```json
{
    "paths": [
        {
            "paths": [
                [
                    {
                        "pool": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
                        "token_in": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
                        "token_out": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
                    }
                ]
            ],
            "chain_id": 1,
            "anchor_token": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
        }
    ]
}
```

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439011",
    "paths": [...],
    "created_at": 1234567890,
    "updated_at": 1234567890
}
```

### PUT /paths/{id}

Update an existing path. **Requires API key authentication.**

```bash
curl -X PUT http://localhost:8081/api/v1/paths/507f1f77bcf86cd799439011 \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key" \
  -d '{
    "paths": [
      {
        "paths": [
          [
            {
              "pool": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
              "token_in": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
              "token_out": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
            }
          ]
        ],
        "chain_id": 1,
        "anchor_token": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
      }
    ]
  }'
```

**Request Body:**

```json
{
    "paths": [
        {
            "paths": [
                [
                    {
                        "pool": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
                        "token_in": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
                        "token_out": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
                    }
                ]
            ],
            "chain_id": 1,
            "anchor_token": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
        }
    ]
}
```

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439011",
    "paths": [...],
    "created_at": 1234567890,
    "updated_at": 1234567890
}
```

---

## Pool Endpoints

### GET /pools

Get all pools.

```bash
curl -X GET http://localhost:8081/api/v1/pools
```

**Response:**

```json
[
    {
        "id": "507f1f77bcf86cd799439011",
        "network_id": 1,
        "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
        "created_at": 1234567890,
        "updated_at": 1234567890
    }
]
```

### GET /pools/network/{network_id}

Get all pools for a specific network.

```bash
curl -X GET http://localhost:8081/api/v1/pools/network/1
```

**Response:**

```json
[
    {
        "id": "507f1f77bcf86cd799439011",
        "network_id": 1,
        "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
        "created_at": 1234567890,
        "updated_at": 1234567890
    }
]
```

### GET /pools/network/{network_id}/address/{address}

Get a specific pool by network ID and address.

```bash
curl -X GET http://localhost:8081/api/v1/pools/network/1/address/0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640
```

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439011",
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    "created_at": 1234567890,
    "updated_at": 1234567890
}
```

### GET /pools/network/{network_id}/count

Get the count of pools for a specific network.

```bash
curl -X GET http://localhost:8081/api/v1/pools/network/1/count
```

**Response:**

```json
{
    "count": 150
}
```

### POST /pools

Create a new pool. **Requires API key authentication.**

```bash
curl -X POST http://localhost:8081/api/v1/pools \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key" \
  -d '{
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"
  }'
```

**Request Body:**

```json
{
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"
}
```

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439011",
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    "created_at": 1234567890,
    "updated_at": 1234567890
}
```

### PUT /pools/{id}

Update an existing pool. **Requires API key authentication.**

```bash
curl -X PUT http://localhost:8081/api/v1/pools/507f1f77bcf86cd799439011 \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key" \
  -d '{
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"
  }'
```

**Request Body:**

```json
{
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"
}
```

**Note:** Both fields are optional. You can update just `network_id` or just `address`, or both.

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439011",
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    "created_at": 1234567890,
    "updated_at": 1234567890
}
```

---

## Token Endpoints

### GET /tokens

Get all tokens.

```bash
curl -X GET http://localhost:8081/api/v1/tokens
```

**Response:**

```json
[
    {
        "id": "507f1f77bcf86cd799439011",
        "network_id": 1,
        "address": "0x6B175474E89094C44Da98b954EedeAC495271d0F",
        "name": "Dai Stablecoin",
        "symbol": "DAI",
        "decimals": 18,
        "created_at": 1234567890,
        "updated_at": 1234567890
    }
]
```

### GET /tokens/network/{network_id}

Get all tokens for a specific network.

```bash
curl -X GET http://localhost:8081/api/v1/tokens/network/1
```

**Response:**

```json
[
    {
        "id": "507f1f77bcf86cd799439011",
        "network_id": 1,
        "address": "0x6B175474E89094C44Da98b954EedeAC495271d0F",
        "name": "Dai Stablecoin",
        "symbol": "DAI",
        "decimals": 18,
        "created_at": 1234567890,
        "updated_at": 1234567890
    }
]
```

### GET /tokens/network/{network_id}/address/{address}

Get a specific token by network ID and address.

```bash
curl -X GET http://localhost:8081/api/v1/tokens/network/1/address/0x6B175474E89094C44Da98b954EedeAC495271d0F
```

**Response:**

```json
{
    "id": "507f1f77bcf86cd799439011",
    "network_id": 1,
    "address": "0x6B175474E89094C44Da98b954EedeAC495271d0F",
    "name": "Dai Stablecoin",
    "symbol": "DAI",
    "decimals": 18,
    "created_at": 1234567890,
    "updated_at": 1234567890
}
```

### GET /tokens/network/{network_id}/count

Get the count of tokens for a specific network.

```bash
curl -X GET http://localhost:8081/api/v1/tokens/network/1/count
```

**Response:**

```json
{
    "count": 5000
}
```

---

## Error Responses

All endpoints may return the following error responses:

### 400 Bad Request

```json
{
    "error": "Invalid address format: ..."
}
```

### 404 Not Found

```json
{
    "error": "Network with chain_id 999 not found"
}
```

### 401 Unauthorized

```json
{
    "error": "Invalid API key"
}
```

or

```json
{
    "error": "API key required"
}
```

### 500 Internal Server Error

```json
{
    "error": "Database error: ..."
}
```

---

## Notes

-   Replace `localhost:8081` with your actual server host and port
-   All timestamps are Unix timestamps (seconds since epoch)
-   Addresses should be in checksummed format (mixed case) or lowercase
-   Chain IDs are unsigned 64-bit integers
-   MongoDB ObjectIds are represented as hexadecimal strings
