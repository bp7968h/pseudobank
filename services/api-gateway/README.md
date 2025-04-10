# API Gateway
This is one of the core service for the banking system simulation, built on top of `hyper.rs`. Most of the features are implemented from scratch to learn what goes under the hood in these systems for example; proxying request, routing request, validation using middleware, rate limiting and so on.

## Features
- Route requests to appropriate microservices
- JWT validation
- Rate limiting
- Request/response logging
- Circuit breaking with Resilience4j

## Example Service Registration Request
Here's what a service registration request might look like:
```json
POST /register HTTP/1.1
Content-Type: application/json
{
  "service_name": "user-service",
  "base_url": "http://user-service:8080",
  "routes": [
    {
      "path": "/api/users",
      "methods": ["GET", "POST"],
      "rate_limit": 100,
      "auth_required": true
    },
    {
      "path": "/api/users/{id}",
      "methods": ["GET", "PUT", "DELETE"],
      "rate_limit": 50,
      "auth_required": true
    }
  ],
  "health_check_path": "/health",
  "timeout_ms": 5000
}
```