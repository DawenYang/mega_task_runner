# Configuration Guide

This application uses a layered configuration system with environment-specific files.

## Configuration Structure

```
src/configuration/
├── base.yaml        # Base configuration (defaults)
├── local.yaml       # Local development overrides (also used for Docker)
└── production.yaml  # Production overrides
```

## How It Works

The configuration system:
1. Loads `base.yaml` first (contains all default values)
2. Loads environment-specific file (`local.yaml` or `production.yaml`)
3. Applies environment variable overrides (with `APP_` prefix)

### Environment Selection

Set the `APP_ENVIRONMENT` variable to choose which config file to load:

```bash
# Local development (default) - also used for Docker
APP_ENVIRONMENT=local cargo run

# Production
APP_ENVIRONMENT=production cargo run
```

## Configuration Files

### base.yaml

Contains all default settings. All fields must be present here.

```yaml
application:
  port: 8000
  host: 0.0.0.0
  hmac_secret: "super-long-and-secret-random-key"
database:
  host: "127.0.0.1"
  port: 5432
  username: "postgres"
  password: "password"
  database_name: "newsletter"
  require_ssl: false
email_client:
  base_url: "localhost"
  sender_email: "test@gmail.com"
  authorization_token: "my-secret-token"
  timeout_milliseconds: 10000
redis_uri: "redis://127.0.0.1:6379"
```

### local.yaml

Overrides for local development and Docker. Only include fields that differ from base.

```yaml
application:
  host: 0.0.0.0
  base_url: "http://localhost:8000"
database:
  require_ssl: false
```

When running in Docker, environment variables override the connection details:
- `APP_DATABASE__HOST=postgres` (Docker service name)
- `APP_REDIS_URI=redis://redis:6379` (Docker service name)

### production.yaml

Production-specific settings. Secrets should come from environment variables.

```yaml
application:
  host: 0.0.0.0
database:
  require_ssl: true
email_client:
  base_url: "https://api.postmarkapp.com"
```

## Environment Variable Overrides

You can override any configuration value using environment variables with this format:

```
APP_<section>__<field>
```

**Important**: Use **double underscore** (`__`) between section and field.

### Examples

```bash
# Override application settings
APP_APPLICATION__PORT=3000
APP_APPLICATION__HOST=127.0.0.1
APP_APPLICATION__HMAC_SECRET=your-secret-here

# Override database settings
APP_DATABASE__HOST=db.example.com
APP_DATABASE__PORT=5433
APP_DATABASE__USERNAME=myuser
APP_DATABASE__PASSWORD=mypassword

# Override email settings
APP_EMAIL_CLIENT__SENDER_EMAIL=noreply@example.com
APP_EMAIL_CLIENT__AUTHORIZATION_TOKEN=your-token

# Override Redis
APP_REDIS_URI=redis://redis.example.com:6379
```

## Docker Configuration

In `docker-compose.yml`, the environment uses `local` and overrides via environment variables:

```yaml
environment:
  APP_ENVIRONMENT: local  # Uses local.yaml as base
  
  # Override database and Redis to use Docker service names
  APP_DATABASE__HOST: postgres
  APP_DATABASE__USERNAME: newsletter
  APP_DATABASE__PASSWORD: newsletter_password
  APP_REDIS_URI: redis://redis:6379
  
  # Override secrets
  APP_APPLICATION__HMAC_SECRET: super-secret-key
  APP_EMAIL_CLIENT__AUTHORIZATION_TOKEN: your-token
```

This approach:
- ✅ Uses `local.yaml` as base (same config for local dev and Docker)
- ✅ Overrides connection details for Docker service names
- ✅ Overrides secrets via environment variables
- ✅ Keeps sensitive data out of config files

## Configuration Priority

Settings are applied in this order (later overrides earlier):

1. **base.yaml** - Defaults
2. **{environment}.yaml** - Environment-specific (local/docker/production)
3. **Environment variables** - Runtime overrides

### Example

```yaml
# base.yaml
database:
  host: "127.0.0.1"
  port: 5432
```

```yaml
# docker.yaml
database:
  host: "postgres"
```

```bash
# Environment variable
APP_DATABASE__PORT=5433
```

**Result**: `host: "postgres"`, `port: 5433`

## Typos to Fix in configuration.rs

**Note**: The following typos exist in `src/configuration.rs` and should be fixed:

```rust
// Current (with typos):
pub struct DatabaseSettings {
    pub usename: String,        // ❌ Should be: username
    pub requrire_ssl: bool,     // ❌ Should be: require_ssl
}

// Should be:
pub struct DatabaseSettings {
    pub username: String,       // ✅
    pub require_ssl: bool,      // ✅
}
```

Also update the YAML files to match:
- `usename` → `username`
- `requrire_ssl` → `require_ssl`



## Quick Reference

| Environment | Config File | Use Case |
|-------------|-------------|----------|
| `local` | `local.yaml` | Local development and Docker containers |
| `production` | `production.yaml` | Production deployment |

## Best Practices

1. **Never commit secrets** to YAML files
   - Use environment variables for sensitive data
   - Add secrets to `.env` (and add `.env` to `.gitignore`)

2. **Keep base.yaml complete**
   - All fields should have default values
   - Environment files only override what's needed

3. **Use environment variables for deployment**
   - Docker: Set in `docker-compose.yml`
   - Kubernetes: Use ConfigMaps/Secrets
   - Cloud: Use platform secrets manager

4. **Document environment variables**
   - List required variables in README
   - Provide example `.env.example` file

## Testing Configuration

```bash
# Test local config
APP_ENVIRONMENT=local cargo run

# Test local config with overrides (simulating Docker)
APP_ENVIRONMENT=local \
APP_DATABASE__HOST=localhost \
APP_REDIS_URI=redis://localhost:6379 \
cargo run

# Test production config
APP_ENVIRONMENT=production cargo run
```
