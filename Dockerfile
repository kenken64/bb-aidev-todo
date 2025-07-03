# Stage 1: Build Angular Frontend
FROM node:20-alpine AS frontend-builder

WORKDIR /frontend

# Copy package files for dependency caching
COPY frontend/package*.json ./
RUN npm ci --production=false

# Copy frontend source and build for production
COPY frontend/ ./
RUN npm run build

# Stage 2: Build Rust Backend
FROM rust:1.82-alpine AS backend-builder

# Install build dependencies
RUN apk add --no-cache musl-dev sqlite-dev pkgconfig

WORKDIR /backend

# Copy all source files
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Build the application with release optimizations
RUN cargo build --release

# Stage 3: Runtime - Backend Only (serving frontend as static files)
FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache sqlite ca-certificates wget

# Create non-root user for security
RUN addgroup -g 1001 -S appgroup && \
    adduser -S appuser -u 1001 -G appgroup

WORKDIR /app

# Copy backend binary from builder stage
COPY --from=backend-builder /backend/target/release/todo-backend ./

# Copy frontend static files from builder stage
COPY --from=frontend-builder /frontend/dist/frontend ./static/

# Create data directory for SQLite with proper permissions
RUN mkdir -p data && chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Expose the port that Railway will use
EXPOSE $PORT

# Environment variables for the backend
ENV RUST_LOG=info \
    DATABASE_URL=sqlite:/app/data/todos.db \
    STATIC_DIR=/app/static

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:${PORT:-3000}/api/todos || exit 1

# Run the backend (which serves both API and static frontend)
CMD ["./todo-backend"]