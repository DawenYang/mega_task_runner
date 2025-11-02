# Docker Setup for Newsletter Subscriber

This guide explains how to run the Newsletter Subscriber application using Docker.

## Prerequisites

- Docker (version 20.10 or higher)
- Docker Compose (version 2.0 or higher)

## Quick Start

### 1. Build and Run All Services

```bash
docker-compose up --build
```

This command will:
1. Build the Rust application
2. Start PostgreSQL database
3. Start Redis cache
4. Start the application

The services will start in the following order:
- PostgreSQL and Redis start first
- Application waits for both to be healthy before starting

### 2. Access the Application

- Application: http://localhost:8000
- PostgreSQL: localhost:5432
- Redis: localhost:6379

### 3. Check Health

```bash
curl http://localhost:8000/health
```

## Detailed Commands

### Build the Application Only

```bash
docker-compose build app
```

### Start Services in Background

```bash
docker-compose up -d
```

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f app
docker-compose logs -f postgres
docker-compose logs -f redis
```

### Stop Services

```bash
docker-compose down
```

### Stop and Remove Volumes (Clean Slate)

```bash
docker-compose down -v
```

### Rebuild After Code Changes

```bash
docker-compose up --build app
```

## Configuration

### Environment Variables

Edit `docker-compose.yml` to change these settings:

**Database:**
- `POSTGRES_USER`: Database username (default: newsletter)
- `POSTGRES_PASSWORD`: Database password
- `POSTGRES_DB`: Database name (default: newsletter)

**Redis:**
- `REDIS_URL`: Redis connection URL (default: redis://redis:6379)

**Application:**
- `APP_HOST`: Host to bind (default: 0.0.0.0)
- `APP_PORT`: Port to listen on (default: 8000)
- `APP_BASE_URL`: Base URL for the application
- `HMAC_SECRET`: Secret key for HMAC (CHANGE IN PRODUCTION!)
- `EMAIL_AUTH_TOKEN`: Your email service API token

### Configuration Files

The application uses YAML configuration files in the `src/configuration/` directory:

- `base.yaml`: Base configuration with all default values
- `local.yaml`: Local development overrides (also used for Docker)
- `production.yaml`: Production-specific overrides

When running in Docker, the app uses `local.yaml` and overrides connection details via environment variables to use Docker service names (`postgres`, `redis`).

## Database Migrations

If you need to run database migrations:

```bash
# Run migrations inside the app container
docker-compose exec app sqlx migrate run

# Or run a one-off command
docker-compose run --rm app sqlx migrate run
```

## Development Workflow

### 1. Make Code Changes

Edit your Rust source files as needed.

### 2. Rebuild and Restart

```bash
docker-compose up --build app
```

### 3. Run Tests

```bash
# Run tests in Docker
docker-compose run --rm app cargo test

# Or build a test image
docker build --target builder -t newsletter-test .
docker run --rm newsletter-test cargo test
```

## Production Considerations

### 1. Update Secrets

Before deploying to production, update these in `docker-compose.yml`:

```yaml
environment:
  POSTGRES_PASSWORD: strong-random-password-here
  HMAC_SECRET: strong-random-secret-here
  EMAIL_AUTH_TOKEN: your-actual-email-token
```

### 2. Use Environment Files

Create a `.env` file:

```bash
POSTGRES_PASSWORD=your-secure-password
HMAC_SECRET=your-secure-secret
EMAIL_AUTH_TOKEN=your-email-token
```

Update docker-compose.yml:

```yaml
services:
  postgres:
    environment:
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
```

### 3. Enable SSL for PostgreSQL

Update `docker.yaml`:

```yaml
database:
  require_ssl: true
```

### 4. Use Docker Secrets (Docker Swarm)

For production deployments with Docker Swarm, use secrets instead of environment variables.

## Troubleshooting

### Application Won't Start

**Check if dependencies are healthy:**

```bash
docker-compose ps
```

Both postgres and redis should show "healthy" status.

**Check application logs:**

```bash
docker-compose logs app
```

### Database Connection Issues

**Verify PostgreSQL is running:**

```bash
docker-compose exec postgres pg_isready -U newsletter
```

**Connect to database manually:**

```bash
docker-compose exec postgres psql -U newsletter -d newsletter
```

### Redis Connection Issues

**Test Redis connectivity:**

```bash
docker-compose exec redis redis-cli ping
```

Should return `PONG`.

### Rebuild from Scratch

If you encounter persistent issues:

```bash
# Stop everything
docker-compose down -v

# Remove old images
docker-compose rm -f

# Rebuild
docker-compose build --no-cache

# Start fresh
docker-compose up
```

### Port Already in Use

If ports 8000, 5432, or 6379 are already in use, update the port mappings in `docker-compose.yml`:

```yaml
ports:
  - "8001:8000"  # Change host port (left side) only
```

## Docker Architecture

### Multi-Stage Build

The Dockerfile uses a multi-stage build:

1. **Builder Stage**: Compiles Rust code with all dependencies
2. **Runtime Stage**: Minimal Debian image with only the binary

This results in a smaller final image (~100MB vs 2GB+).

### Networks

All services communicate via the `newsletter_network` bridge network:

- Services can reach each other using service names (e.g., `postgres`, `redis`)
- Application connects to `postgres:5432` instead of `localhost:5432`

### Volumes

Persistent data is stored in Docker volumes:

- `postgres_data`: Database files
- `redis_data`: Redis persistence files

These volumes persist even when containers are stopped.

## Health Checks

Each service has health checks configured:

**PostgreSQL:**
```bash
pg_isready -U newsletter
```

**Redis:**
```bash
redis-cli ping
```

The app service waits for both to be healthy before starting.

## Useful Commands

```bash
# View running containers
docker-compose ps

# Execute command in running container
docker-compose exec app /bin/bash

# View resource usage
docker stats

# Clean up unused Docker resources
docker system prune

# View volumes
docker volume ls

# Inspect a volume
docker volume inspect newsletter_subscriber_postgres_data
```

## Next Steps

1. Update email service credentials in `docker-compose.yml`
2. Add database migration files if needed
3. Configure CI/CD pipeline for automated builds
4. Set up monitoring and logging
5. Consider using Kubernetes for production deployments
