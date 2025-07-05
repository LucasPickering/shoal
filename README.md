# Shoal API

A simple HTTP API for managing fish, built with Rust and Axum.

## Features

- **RESTful API** - Complete CRUD operations for fish management
- **OpenAPI Documentation** - Interactive API docs with Swagger UI at `/docs`
- **Async/Await** - Built with Tokio for high-performance async operations
- **Type Safety** - Leverages Rust's type system for reliable API contracts
- **In-Memory Storage** - Thread-safe storage using RwLock

## Quick Start

```bash
# Run the server
cargo run

# The server will start on http://127.0.0.1:3000
```

## API Endpoints

### Health Check

- `GET /health` - Check if the service is running

### Fish Management

- `GET /api/fish` - Get all fish
- `GET /api/fish/{id}` - Get a specific fish by ID
- `POST /api/fish` - Create a new fish
- `PUT /api/fish/{id}` - Update an existing fish
- `DELETE /api/fish/{id}` - Delete a fish

### Documentation

- `GET /docs` - Interactive OpenAPI documentation (Swagger UI)

## Example Usage

The server starts with sample data (Nemo and Dory). You can interact with the API using curl:

```bash
# Get all fish
curl http://127.0.0.1:3000/api/fish

# Create a new fish
curl -X POST http://127.0.0.1:3000/api/fish \
  -H "Content-Type: application/json" \
  -d '{"name": "Bubbles", "species": "Goldfish", "age": 1, "weight_kg": 0.05}'

# Get a specific fish
curl http://127.0.0.1:3000/api/fish/1

# Update a fish
curl -X PUT http://127.0.0.1:3000/api/fish/1 \
  -H "Content-Type: application/json" \
  -d '{"name": "Updated Name", "age": 3}'

# Delete a fish
curl -X DELETE http://127.0.0.1:3000/api/fish/1
```

## Data Models

### Fish

```json
{
  "id": 1,
  "name": "Nemo",
  "species": "Clownfish",
  "age": 2,
  "weight_kg": 0.1
}
```

### Response Format

All responses follow a consistent format:

```json
{
  "success": true,
  "data": {
    /* response data */
  },
  "message": "Success message"
}
```

## Dependencies

- **axum** - Web framework
- **tokio** - Async runtime
- **utoipa** - OpenAPI documentation generation
- **utoipa-swagger-ui** - Swagger UI integration
- **serde** - JSON serialization/deserialization
