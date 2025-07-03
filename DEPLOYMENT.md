# Backend-Only Deployment with Embedded Frontend

This deployment strategy builds the Angular frontend during the Docker multi-stage build and embeds it into the Rust backend as static files. The result is a **single backend service** that serves both the API and the frontend.

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────────┐
│              Docker Build               │
│  ┌─────────────┐    ┌─────────────────┐ │
│  │   Stage 1   │    │     Stage 2     │ │
│  │   Angular   │    │      Rust       │ │
│  │   Builder   │    │     Builder     │ │
│  └─────────────┘    └─────────────────┘ │
│         │                    │         │
│         ▼                    ▼         │
│  ┌─────────────────────────────────────┐ │
│  │          Stage 3: Runtime           │ │
│  │   Rust Backend + Static Frontend    │ │
│  └─────────────────────────────────────┘ │
└─────────────────────────────────────────┘
                    │
                    ▼
            ┌───────────────┐
            │ Railway       │
            │ Single Service│
            └───────────────┘
```

## 🐳 Docker Multi-Stage Build

### Stage 1: Frontend Builder
- **Base Image**: `node:20-alpine`
- **Purpose**: Build Angular application for production
- **Output**: Static files in `/frontend/dist/frontend/`
- **Optimizations**: 
  - NPM dependency caching
  - Production build with optimizations

### Stage 2: Backend Builder  
- **Base Image**: `rust:1.75-alpine`
- **Purpose**: Compile Rust backend with release optimizations
- **Output**: Optimized binary `/backend/target/release/todo-backend`
- **Optimizations**:
  - Cargo dependency caching
  - Release build with maximum optimizations

### Stage 3: Runtime
- **Base Image**: `alpine:3.18`
- **Contents**: 
  - Rust backend binary (serves API + static files)
  - Angular static files (embedded)
  - SQLite database
- **Security**: Non-root user, minimal attack surface
- **Size**: ~50MB final image

## 🚀 Deployment Process

### 1. Code Push
```bash
git push origin main
```

### 2. GitHub Actions Pipeline
1. **Test Phase**:
   - Rust backend tests (`cargo test`)
   - Angular frontend tests (Jest/Karma)
   - Frontend build validation

2. **Deploy Phase**:
   - Railway triggers Docker build
   - Multi-stage build process
   - Single backend service deployment

### 3. Railway Deployment
- **Service Type**: Backend service only
- **Build**: Docker-based using Dockerfile
- **Port**: Dynamic (Railway assigns)
- **Database**: SQLite with persistent volumes

## ⚙️ Configuration

### Required GitHub Secrets
```
RAILWAY_TOKEN=your_railway_project_token
RAILWAY_SERVICE_ID=your_backend_service_id
```

### Environment Variables (Set by Railway)
```bash
PORT=8080                                    # Railway dynamic port
HOST=0.0.0.0                                # Container binding
RUST_LOG=info                               # Logging level
DATABASE_URL=sqlite:/app/data/todos.db       # SQLite database path
STATIC_DIR=/app/static                       # Frontend static files path
```

### Railway Configuration (`railway.toml`)
```toml
[build]
builder = "DOCKERFILE"
dockerfilePath = "Dockerfile"

[deploy]
startCommand = "./todo-backend"
restartPolicyType = "ON_FAILURE"
restartPolicyMaxRetries = 10
```

## 🛣️ URL Structure

After deployment, the single Railway service serves:

```
https://your-app.railway.app/
├── /                          # Angular frontend (SPA)
├── /api/todos                 # REST API endpoints
│   ├── GET /api/todos         # List todos
│   ├── POST /api/todos        # Create todo
│   ├── PUT /api/todos/:id     # Update todo
│   └── DELETE /api/todos/:id  # Delete todo
└── /assets/*                  # Static assets (CSS, JS, images)
```

## 🔧 Local Development

### With Docker
```bash
# Build the image
docker build -t todo-backend .

# Run the container
docker run -p 3000:3000 todo-backend

# Access the app
open http://localhost:3000
```

### With Docker Compose
```bash
# Start with persistent data
docker-compose up -d

# View logs
docker-compose logs -f

# Stop
docker-compose down
```

### Development Mode (without Docker)
```bash
# Terminal 1: Start Rust backend
cargo run

# Terminal 2: Start Angular with proxy
cd frontend && npm start

# Access development app
open http://localhost:4200
```

## 📊 Benefits of This Approach

### ✅ Advantages
1. **Single Service**: Only one Railway service to manage and pay for
2. **No CORS Issues**: Frontend and API served from same origin
3. **Atomic Deployments**: Frontend and backend deployed together
4. **Simplified Configuration**: No service communication setup
5. **Better Performance**: Static files served directly by Rust
6. **Cost Effective**: Single service pricing instead of two services

### ⚠️ Trade-offs
1. **Monolithic Deployment**: Frontend changes require backend rebuild
2. **Build Time**: Longer builds due to multi-stage process
3. **Resource Coupling**: Frontend and backend share the same resources

## 🔍 Monitoring & Debugging

### Railway Dashboard
- **Logs**: Combined frontend build + backend runtime logs
- **Metrics**: CPU, memory, network usage for single service
- **Deployments**: Single deployment timeline

### Health Checks
```bash
# API Health
curl https://your-app.railway.app/api/todos

# Frontend Health
curl https://your-app.railway.app/
```

### Log Analysis
```bash
# View Railway logs
railway logs

# Filter for backend logs
railway logs | grep "todo-backend"
```

## 🚨 Troubleshooting

### Build Issues
```bash
# Check GitHub Actions logs
# Look for Node.js or Rust compilation errors
```

### Runtime Issues
```bash
# Check Railway service logs
# Verify static files are being served
# Confirm API endpoints respond correctly
```

### Database Issues
```bash
# Verify SQLite file permissions
# Check persistent storage configuration
# Ensure database migrations run correctly
```

## 🎯 Production Considerations

### Performance
- **Static File Caching**: Rust serves with appropriate cache headers
- **Gzip Compression**: Enable for static assets
- **Database Connection Pooling**: SQLx handles connection management

### Security
- **Non-root Container**: Runs as `appuser` (UID 1001)
- **Minimal Attack Surface**: Alpine base with only necessary packages
- **HTTPS**: Automatically provided by Railway

### Scaling
- **Vertical Scaling**: Railway auto-scales resources
- **Database**: SQLite suitable for moderate loads
- **CDN**: Consider for static assets at scale

This deployment strategy provides a simple, cost-effective solution for full-stack applications while maintaining the benefits of modern containerization and CI/CD practices.