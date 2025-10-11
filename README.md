# Shoal API

**[Website](https://shoal.lucaspickering.me/)**

A simple HTTP API for managing fish, built with Rust and Axum. This is an example API built for testing [slumber](github.com/LucasPickering/slumber). It features temporary sessions that allow you to create, modify, and delete fish in a private sandbox. Sessions expire after 1 hour, so they're intended only for quick testing and examples.

## Deployment

- Create `.env` and set `DEPLOY_HOST=<user@ip>`
- `mise build`
- `mise deploy`
