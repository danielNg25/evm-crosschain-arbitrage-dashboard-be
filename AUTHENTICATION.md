# API Authentication Guide

## Overview

The API uses **API Key authentication** via the `X-API-Key` header to protect sensitive endpoints.

## Protected Endpoints

-   `PUT /api/v1/config` - Update configuration (requires API key)

## Quick Setup

1. **Set API key in config file:**

```bash
echo 'api_key = "your-secret-key"' >> config/config.toml
```

2. **Or use environment variable:**

```bash
export API_KEY="your-secret-key"
```

3. **Use in requests:**

```bash
curl -H "X-API-Key: your-secret-key" ...
```

## Security Best Practices

✅ Generate strong keys: `openssl rand -base64 32`
✅ Use HTTPS in production
✅ Never commit keys to git
✅ Rotate keys regularly
❌ Don't hardcode keys in frontend code

See API_EXAMPLES.md for complete usage examples.
