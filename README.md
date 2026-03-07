# ALICE API Gateway SaaS

Managed API gateway powered by ALICE-API. Register, proxy, rate-limit, and analyze your APIs via a simple management interface.

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

## Status

| Check | Status |
|-------|--------|
| `cargo check` | passing |
| `tsc --noEmit` | passing |
| API health | `/health` |

## Quick Start

```bash
docker compose up -d
```

Frontend: http://localhost:3000
API Gateway: http://localhost:8080
Gateway Engine: http://localhost:8081

## Architecture

```
Browser / Client
      |
      v
Frontend (Next.js)      :3000
      |
      v
API Gateway (Proxy)     :8080
      |
      v
Gateway Engine          :8081
(API management core)
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/v1/gateway/apis` | Register a new API upstream |
| `GET` | `/api/v1/gateway/apis` | List all registered APIs |
| `DELETE` | `/api/v1/gateway/apis/{api_id}` | Delete an API registration |
| `POST` | `/api/v1/gateway/traffic` | Get traffic analysis for an API |
| `POST` | `/api/v1/gateway/ratelimit` | Configure rate limiting |
| `POST` | `/api/v1/gateway/openapi` | Auto-generate OpenAPI spec |
| `GET` | `/api/v1/gateway/stats` | Get gateway statistics |
| `GET` | `/health` | Service health check |

### Register API

```json
POST /api/v1/gateway/apis
{
  "name": "my-api",
  "upstream_url": "http://backend:3000",
  "path_prefix": "/api/v1/my-api",
  "rate_limit_per_minute": 1000,
  "auth_required": true
}
```

### Traffic Analysis

```json
POST /api/v1/gateway/traffic
{
  "api_id": "api-001",
  "time_range_minutes": 60
}
```

Response:
```json
{
  "total_requests": 150000,
  "avg_latency_ms": 12.5,
  "p99_latency_ms": 120.8,
  "error_rate_percent": 0.3,
  "status_distribution": { "2xx": 98.5, "4xx": 1.2, "5xx": 0.3 }
}
```

## Core Technology

- **Rust/Axum**: Low-latency reverse proxy
- **DashMap**: Lock-free concurrent rate limiting
- **Token Bucket**: Configurable rate limiting strategies
- **JWT + API Key**: Dual authentication support

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `GATEWAY_ENGINE_ADDR` | `0.0.0.0:8081` | Engine bind address |
| `NEXT_PUBLIC_API_URL` | `http://localhost:8080` | API gateway URL for frontend |

## License

AGPL-3.0. Commercial dual-license available — contact for pricing.
