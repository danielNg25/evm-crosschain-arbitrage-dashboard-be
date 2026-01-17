# API Specification

## Base URL

```
http://localhost:8081/api/v1
```

## Authentication

Protected endpoints require an API key to be sent in the `X-API-Key` header.

```
X-API-Key: your-secret-api-key
```

The API key must be configured in your `config/config.toml` file under `[server]` section as `api_key = "your-secret-key"`, or set via the `API_KEY` environment variable.

---

## Health Check

### GET /health

Check if the API is running.

**Authentication:** None

**Response:** `200 OK`

```json
{
    "status": "ok"
}
```

---

## Config Endpoints

### GET /config

Get the current configuration.

**Authentication:** None

**Response:** `200 OK`

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

Update the configuration.

**Authentication:** Required (X-API-Key header)

**Request Body:**

```json
{
    "max_amount_usd": 2000.0,
    "recheck_interval": 120
}
```

**Response:** `200 OK`

```json
{
    "id": "507f1f77bcf86cd799439011",
    "max_amount_usd": 2000.0,
    "recheck_interval": 120,
    "created_at": 1234567890,
    "updated_at": 1234567891
}
```

**Error Responses:**

-   `401 Unauthorized` - Missing or invalid API key
-   `500 Internal Server Error` - Database error

---

## Network Endpoints

### GET /networks

Get all networks, including soft-deleted ones. Deleted networks are marked with `deleted: true`.

**Authentication:** None

**Response:** `200 OK`

```json
[
    {
        "id": "507f1f77bcf86cd799439011",
        "chain_id": 8453,
        "name": "Base",
        "rpcs": ["https://mainnet.base.org"],
        "websocket_urls": null,
        "block_explorer": "https://basescan.org",
        "wrap_native": "0x4200000000000000000000000000000000000006",
        "min_profit_usd": 10.0,
        "v2_factory_to_fee": {
            "0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6": 3000
        },
        "aero_factory_addresses": [
            "0x420DD381b31aEf6683db6B902084cB0FFECe40Da"
        ],
        "multicall_address": "0xcA11bde05977b3631167028862bE2a173976CA11",
        "max_blocks_per_batch": 1000,
        "wait_time_fetch": 1000,
        "created_at": 1234567890,
        "updated_at": 1234567890,
        "deleted": false
    }
]
```

### GET /networks/{chain_id}

Get a specific network by chain ID. Returns the network even if it's soft-deleted (marked with `deleted: true`).

**Authentication:** None

**Path Parameters:**

-   `chain_id` (number) - The chain ID of the network

**Response:** `200 OK`

```json
{
    "id": "507f1f77bcf86cd799439011",
    "chain_id": 8453,
    "name": "Base",
    "rpcs": ["https://mainnet.base.org"],
    "websocket_urls": null,
    "block_explorer": "https://basescan.org",
    "wrap_native": "0x4200000000000000000000000000000000000006",
    "min_profit_usd": 10.0,
    "v2_factory_to_fee": {
        "0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6": 3000
    },
    "aero_factory_addresses": ["0x420DD381b31aEf6683db6B902084cB0FFECe40Da"],
    "multicall_address": "0xcA11bde05977b3631167028862bE2a173976CA11",
    "max_blocks_per_batch": 1000,
    "wait_time_fetch": 1000,
    "created_at": 1234567890,
    "updated_at": 1234567890,
    "deleted": false
}
```

**Error Responses:**

-   `404 Not Found` - Network not found

### POST /networks

Create a new network. If a network with the same chain_id exists (even if soft-deleted), it will be restored and updated.

**Authentication:** Required (X-API-Key header)

**Request Body:**

```json
{
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
    "wait_time_fetch": 1000
}
```

**Response:** `201 Created`

```json
{
    "id": "507f1f77bcf86cd799439011",
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
    "wait_time_fetch": 1000,
    "created_at": 1234567890,
    "updated_at": 1234567890,
    "deleted": false
}
```

**Error Responses:**

-   `400 Bad Request` - Invalid request data
-   `401 Unauthorized` - Missing or invalid API key
-   `500 Internal Server Error` - Database error

### PUT /networks/{chain_id}

Update an existing network. Only provided fields will be updated.

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `chain_id` (number) - The chain ID of the network to update

**Request Body:** (All fields optional)

```json
{
    "name": "Base Mainnet",
    "rpcs": ["https://mainnet.base.org", "https://base.llamarpc.com"],
    "min_profit_usd": 15.0,
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

**Response:** `200 OK`

```json
{
    "id": "507f1f77bcf86cd799439011",
    "chain_id": 8453,
    "name": "Base Mainnet",
    "rpcs": ["https://mainnet.base.org", "https://base.llamarpc.com"],
    "websocket_urls": null,
    "block_explorer": "https://basescan.org",
    "wrap_native": "0x4200000000000000000000000000000000000006",
    "min_profit_usd": 15.0,
    "v2_factory_to_fee": {
        "0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6": 3000,
        "0x420DD381b31aEf6683db6B902084cB0FFECe40Da": 500
    },
    "aero_factory_addresses": [
        "0x420DD381b31aEf6683db6B902084cB0FFECe40Da",
        "0x5C7BCd6E7De5423a257D81B442095A1a6ced35C5"
    ],
    "multicall_address": "0xcA11bde05977b3631167028862bE2a173976CA11",
    "max_blocks_per_batch": 1000,
    "wait_time_fetch": 1000,
    "created_at": 1234567890,
    "updated_at": 1234567891,
    "deleted": false
}
```

**Error Responses:**

-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Network not found
-   `500 Internal Server Error` - Database error

### PUT /networks/{chain_id}/factories

Update both V2 factory fees and Aero factory addresses together.

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `chain_id` (number) - The chain ID of the network to update

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

**Response:** `200 OK`

```json
{
    "id": "507f1f77bcf86cd799439011",
    "chain_id": 8453,
    "name": "Base",
    "rpcs": ["https://mainnet.base.org"],
    "websocket_urls": null,
    "block_explorer": "https://basescan.org",
    "wrap_native": "0x4200000000000000000000000000000000000006",
    "min_profit_usd": 10.0,
    "v2_factory_to_fee": {
        "0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6": 3000,
        "0x420DD381b31aEf6683db6B902084cB0FFECe40Da": 500
    },
    "aero_factory_addresses": [
        "0x420DD381b31aEf6683db6B902084cB0FFECe40Da",
        "0x5C7BCd6E7De5423a257D81B442095A1a6ced35C5"
    ],
    "multicall_address": "0xcA11bde05977b3631167028862bE2a173976CA11",
    "max_blocks_per_batch": 1000,
    "wait_time_fetch": 1000,
    "created_at": 1234567890,
    "updated_at": 1234567891,
    "deleted": false
}
```

**Error Responses:**

-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Network not found
-   `500 Internal Server Error` - Database error

### DELETE /networks/{chain_id}

Soft delete a network (sets deleted_at timestamp).

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `chain_id` (number) - The chain ID of the network to delete

**Response:** `204 No Content`

**Error Responses:**

-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Network not found or already deleted
-   `500 Internal Server Error` - Database error

### POST /networks/{chain_id}/undelete

Restore a soft-deleted network by setting `deleted_at` to null.

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `chain_id` (number) - The chain ID of the network to restore

**Response:** `200 OK`

```json
{
    "id": "507f1f77bcf86cd799439011",
    "chain_id": 8453,
    "name": "Base",
    "rpcs": ["https://mainnet.base.org"],
    "websocket_urls": null,
    "block_explorer": "https://basescan.org",
    "wrap_native": "0x4200000000000000000000000000000000000006",
    "min_profit_usd": 10.0,
    "v2_factory_to_fee": {
        "0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6": 3000
    },
    "aero_factory_addresses": ["0x420DD381b31aEf6683db6B902084cB0FFECe40Da"],
    "multicall_address": "0xcA11bde05977b3631167028862bE2a173976CA11",
    "max_blocks_per_batch": 1000,
    "wait_time_fetch": 1000,
    "created_at": 1234567890,
    "updated_at": 1234567891,
    "deleted": false
}
```

**Error Responses:**

-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Network not found
-   `500 Internal Server Error` - Database error

### DELETE /networks/{chain_id}/hard

Permanently delete a network from the database. **Only works on networks that are already soft-deleted.**

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `chain_id` (number) - The chain ID of the network to hard delete

**Response:** `204 No Content`

**Error Responses:**

-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Network not found or not soft-deleted
-   `500 Internal Server Error` - Database error

---

## Path Endpoints

### GET /paths

Get all paths, including soft-deleted ones. Deleted paths are marked with `deleted: true`.

**Authentication:** None

**Response:** `200 OK`

```json
[
    {
        "id": "507f1f77bcf86cd799439011",
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
        ],
        "created_at": 1234567890,
        "updated_at": 1234567890,
        "deleted": false
    }
]
```

### GET /paths/{id}

Get a specific path by ID. Returns the path even if it's soft-deleted (marked with `deleted: true`).

**Authentication:** None

**Path Parameters:**

-   `id` (string) - MongoDB ObjectId of the path

**Response:** `200 OK`

```json
{
    "id": "507f1f77bcf86cd799439011",
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
    ],
    "created_at": 1234567890,
    "updated_at": 1234567890,
    "deleted": false
}
```

**Error Responses:**

-   `400 Bad Request` - Invalid ID format
-   `404 Not Found` - Path not found

### GET /paths/anchor-token/{anchor_token}

Get paths by anchor token address.

**Authentication:** None

**Path Parameters:**

-   `anchor_token` (string) - The anchor token address

**Response:** `200 OK`

```json
[
    {
        "id": "507f1f77bcf86cd799439011",
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
        ],
        "created_at": 1234567890,
        "updated_at": 1234567890,
        "deleted": false
    }
]
```

### GET /paths/chain/{chain_id}

Get paths by chain ID. Returns all paths including soft-deleted ones (marked with `deleted: true`).

**Authentication:** None

**Path Parameters:**

-   `chain_id` (number) - The chain ID to filter by

**Response:** `200 OK`

```json
[
    {
        "id": "507f1f77bcf86cd799439011",
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
        ],
        "created_at": 1234567890,
        "updated_at": 1234567890,
        "deleted": false
    }
]
```

### POST /paths

Create a new path. All pools referenced in the path will be automatically created if they don't exist.

**Authentication:** Required (X-API-Key header)

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

**Response:** `201 Created`

```json
{
    "id": "507f1f77bcf86cd799439011",
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
    ],
    "created_at": 1234567890,
    "updated_at": 1234567890,
    "deleted": false
}
```

**Error Responses:**

-   `400 Bad Request` - Invalid request data
-   `401 Unauthorized` - Missing or invalid API key
-   `500 Internal Server Error` - Database error

### PUT /paths/{id}

Update an existing path.

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `id` (string) - MongoDB ObjectId of the path to update

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

**Response:** `200 OK`

```json
{
    "id": "507f1f77bcf86cd799439011",
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
    ],
    "created_at": 1234567890,
    "updated_at": 1234567891,
    "deleted": false
}
```

**Error Responses:**

-   `400 Bad Request` - Invalid ID format or request data
-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Path not found
-   `500 Internal Server Error` - Database error

### DELETE /paths/{id}

Soft delete a path (sets deleted_at timestamp).

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `id` (string) - MongoDB ObjectId of the path to delete

**Response:** `204 No Content`

**Error Responses:**

-   `400 Bad Request` - Invalid ID format
-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Path not found or already deleted
-   `500 Internal Server Error` - Database error

### POST /paths/{id}/undelete

Restore a soft-deleted path by setting `deleted_at` to null.

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `id` (string) - MongoDB ObjectId of the path to restore

**Response:** `200 OK`

```json
{
    "id": "507f1f77bcf86cd799439011",
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
    ],
    "created_at": 1234567890,
    "updated_at": 1234567891,
    "deleted": false
}
```

**Error Responses:**

-   `400 Bad Request` - Invalid ID format
-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Path not found
-   `500 Internal Server Error` - Database error

### DELETE /paths/{id}/hard

Permanently delete a path from the database. **Only works on paths that are already soft-deleted.**

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `id` (string) - MongoDB ObjectId of the path to hard delete

**Response:** `204 No Content`

**Error Responses:**

-   `400 Bad Request` - Invalid ID format
-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Path not found or not soft-deleted
-   `500 Internal Server Error` - Database error

---

## Pool Endpoints

### GET /pools

Get all pools.

**Authentication:** None

**Response:** `200 OK`

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

**Authentication:** None

**Path Parameters:**

-   `network_id` (number) - The network ID to filter by

**Response:** `200 OK`

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

**Authentication:** None

**Path Parameters:**

-   `network_id` (number) - The network ID
-   `address` (string) - The pool address (hex string)

**Response:** `200 OK`

```json
{
    "id": "507f1f77bcf86cd799439011",
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    "created_at": 1234567890,
    "updated_at": 1234567890
}
```

**Error Responses:**

-   `400 Bad Request` - Invalid address format
-   `404 Not Found` - Pool not found

### GET /pools/network/{network_id}/count

Get the count of pools for a specific network.

**Authentication:** None

**Path Parameters:**

-   `network_id` (number) - The network ID to count

**Response:** `200 OK`

```json
{
    "count": 150
}
```

### POST /pools

Create a new pool. The pool will be verified on-chain before being created. If a pool with the same network_id and address exists (even if soft-deleted), it will be restored.

**Authentication:** Required (X-API-Key header)

**Request Body:**

```json
{
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"
}
```

**Response:** `201 Created`

```json
{
    "id": "507f1f77bcf86cd799439011",
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    "created_at": 1234567890,
    "updated_at": 1234567890
}
```

**Error Responses:**

-   `400 Bad Request` - Invalid request data or pool verification failed
-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Network not found
-   `500 Internal Server Error` - Database error

### PUT /pools/{id}

Update an existing pool.

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `id` (string) - MongoDB ObjectId of the pool to update

**Request Body:** (All fields optional)

```json
{
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"
}
```

**Response:** `200 OK`

```json
{
    "id": "507f1f77bcf86cd799439011",
    "network_id": 1,
    "address": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    "created_at": 1234567890,
    "updated_at": 1234567891
}
```

**Error Responses:**

-   `400 Bad Request` - Invalid ID format or request data
-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Pool not found
-   `500 Internal Server Error` - Database error

### DELETE /pools/{id}

Soft delete a pool (sets deleted_at timestamp).

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `id` (string) - MongoDB ObjectId of the pool to delete

**Response:** `204 No Content`

**Error Responses:**

-   `400 Bad Request` - Invalid ID format
-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Pool not found or already deleted
-   `500 Internal Server Error` - Database error

### DELETE /pools/{id}/hard

Permanently delete a pool from the database. **Only works on pools that are already soft-deleted.**

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `id` (string) - MongoDB ObjectId of the pool to hard delete

**Response:** `204 No Content`

**Error Responses:**

-   `400 Bad Request` - Invalid ID format
-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Pool not found or not soft-deleted
-   `500 Internal Server Error` - Database error

---

## Token Endpoints

### GET /tokens

Get all tokens.

**Authentication:** None

**Response:** `200 OK`

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

**Authentication:** None

**Path Parameters:**

-   `network_id` (number) - The network ID to filter by

**Response:** `200 OK`

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

**Authentication:** None

**Path Parameters:**

-   `network_id` (number) - The network ID
-   `address` (string) - The token address (hex string)

**Response:** `200 OK`

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

**Error Responses:**

-   `400 Bad Request` - Invalid address format
-   `404 Not Found` - Token not found

### GET /tokens/network/{network_id}/count

Get the count of tokens for a specific network.

**Authentication:** None

**Path Parameters:**

-   `network_id` (number) - The network ID to count

**Response:** `200 OK`

```json
{
    "count": 250
}
```

### DELETE /tokens/network/{network_id}/address/{address}

Soft delete a token (sets deleted_at timestamp).

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `network_id` (number) - The network ID
-   `address` (string) - The token address (hex string)

**Response:** `204 No Content`

**Error Responses:**

-   `400 Bad Request` - Invalid address format
-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Token not found or already deleted
-   `500 Internal Server Error` - Database error

### DELETE /tokens/network/{network_id}/address/{address}/hard

Permanently delete a token from the database. **Only works on tokens that are already soft-deleted.**

**Authentication:** Required (X-API-Key header)

**Path Parameters:**

-   `network_id` (number) - The network ID
-   `address` (string) - The token address (hex string)

**Response:** `204 No Content`

**Error Responses:**

-   `400 Bad Request` - Invalid address format
-   `401 Unauthorized` - Missing or invalid API key
-   `404 Not Found` - Token not found or not soft-deleted
-   `500 Internal Server Error` - Database error

---

## Data Types

### PoolDirection

```json
{
    "pool": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    "token_in": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    "token_out": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
}
```

-   `pool` (string) - Pool contract address
-   `token_in` (string) - Input token address
-   `token_out` (string) - Output token address

### SingleChainPathsWithAnchorToken

```json
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
```

-   `paths` (array of arrays) - Array of pool paths, where each path is an array of PoolDirection objects
-   `chain_id` (number) - Chain ID
-   `anchor_token` (string) - Anchor token address

---

## Error Responses

All endpoints may return the following error responses:

### 400 Bad Request

```json
{
    "error": "Invalid request format or missing required fields"
}
```

### 401 Unauthorized

```json
{
    "error": "Missing or invalid API key"
}
```

### 404 Not Found

```json
{
    "error": "Resource not found"
}
```

### 500 Internal Server Error

```json
{
    "error": "Internal server error or database operation failed"
}
```

---

## Notes

1. **Soft Delete**: All DELETE operations perform soft deletes by setting the `deleted_at` timestamp. For Networks and Paths, GET endpoints return all records (including deleted ones) with a `deleted: true` field. For other resources (Pool, Token), soft-deleted records are automatically excluded from GET queries.

    **Hard Delete**: Use `DELETE /{resource}/{id}/hard` endpoints to permanently remove records from the database. **Hard delete only works on records that are already soft-deleted.** This provides a safety mechanism to prevent accidental permanent deletion.

2. **Undelete**: Use `POST /networks/{chain_id}/undelete` to restore a soft-deleted network, or `POST /paths/{id}/undelete` to restore a soft-deleted path by setting `deleted_at` to null.

3. **Create with Restore**: For Network and Pool, if you create a resource that already exists (even if soft-deleted), it will be restored (deleted_at set to null) and updated with the new data.

4. **Pool Verification**: When creating a pool via POST /pools, the pool is verified on-chain to ensure it's a valid Uniswap V2/V3 pool before being persisted.

5. **Path Pool Creation**: When creating a path via POST /paths, all pools referenced in the path are automatically created if they don't exist.

6. **Address Validation**: All addresses (pool addresses, token addresses, factory addresses, etc.) are validated for correct format before being saved to the database.

7. **Path Validation**: When creating or updating paths, the API validates that:

    - The first `token_in` in each path equals the `anchor_token`
    - Each `token_out` equals the next `token_in` (path connectivity)

8. **Timestamps**: All timestamps (`created_at`, `updated_at`, `deleted_at`) are Unix timestamps in seconds (u64).

9. **Address Format**: All Ethereum addresses should be provided as hex strings with the `0x` prefix (e.g., `"0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"`).

10. **ObjectId Format**: MongoDB ObjectIds are returned as hex strings (24 characters).
