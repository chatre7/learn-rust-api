[![CI](https://github.com/chatre7/learn-rust-api/actions/workflows/ci.yml/badge.svg)](https://github.com/chatre7/learn-rust-api/actions/workflows/ci.yml)

Rust Sample REST API (Axum + SQLx + PostgreSQL)

Features
- CRUD for entity `Book` (id, title, author, timestamps)
- PostgreSQL with SQLx (async) and embedded migrations
- Clean architecture: domain, repo, service, handlers, infra
- Unit tests for service and handlers using in-memory repo
- Dockerfile and docker-compose for local setup

Quickstart
1) Run Postgres:
   - docker compose up -d db
2) Configure env:
   - copy `.env.example` to `.env` and adjust `DATABASE_URL`
3) Run locally:
   - cargo run
4) Run tests:
   - cargo test
5) Container:
   - docker compose up --build

HTTP Endpoints
- GET /health -> "ok"
- POST /books {"title","author"}
- GET /books?offset=0&limit=20
- GET /books/:id
- PUT /books/:id {"title?","author?"}
- DELETE /books/:id

Project Structure
- src/domain: entities and DTOs
- src/repo: repository trait and SQLx implementation
- src/service: business logic and validation
- src/handlers: Axum handlers and tests
- src/infrastructure: database pool and migrations runner
- src/routes: router assembly

Notes
- On startup, migrations in `migrations/` run automatically.
- Limit is clamped to [1,100] for listing.
