# CURSED Deployment Guide

## Production Deployment for CURSED Applications

This guide covers everything you need to know about deploying CURSED applications to production environments, from local development to enterprise-scale cloud deployments.

## Table of Contents

- [Quick Start Deployment](#quick-start-deployment)
- [Native Compilation](#native-compilation)
- [WebAssembly Deployment](#webassembly-deployment)
- [Container Deployment](#container-deployment)
- [Cloud Platform Integration](#cloud-platform-integration)
- [CI/CD Pipelines](#cicd-pipelines)
- [Performance Optimization](#performance-optimization)
- [Monitoring and Observability](#monitoring-and-observability)
- [Security Best Practices](#security-best-practices)

## Quick Start Deployment

### Local Development to Production

**1. Development Setup:**
```bash
# Create a new CURSED project
mkdir my-cursed-app && cd my-cursed-app

# Initialize project structure
echo 'sus message tea = "Hello, Production!"
vibez.spill(message)' > main.csd

# Test locally
cursed-zig main.csd
```

**2. Production Build:**
```bash
# Compile for production
cursed-zig --compile --optimize --target=native main.csd

# Create deployment package
mkdir -p dist/
cp main-native dist/my-cursed-app
chmod +x dist/my-cursed-app

# Test production build
./dist/my-cursed-app
```

**3. Simple Deployment:**
```bash
# Copy to production server
scp -r dist/ user@production-server:/opt/my-cursed-app/

# Run on production server
ssh user@production-server
cd /opt/my-cursed-app
./my-cursed-app
```

### Zero-Downtime Deployment Script

```bash
#!/bin/bash
# deploy.sh - Zero-downtime CURSED application deployment

set -euo pipefail

APP_NAME="my-cursed-app"
DEPLOY_DIR="/opt/${APP_NAME}"
BACKUP_DIR="/opt/${APP_NAME}-backup"
BINARY_PATH="./dist/${APP_NAME}"

echo "🚀 Starting deployment of ${APP_NAME}..."

# Health check function
health_check() {
    local port=${1:-8080}
    timeout 30 bash -c "while ! nc -z localhost ${port}; do sleep 1; done"
    echo "✅ Health check passed"
}

# Backup current version
if [ -d "${DEPLOY_DIR}" ]; then
    echo "📦 Creating backup..."
    mv "${DEPLOY_DIR}" "${BACKUP_DIR}"
fi

# Deploy new version
echo "📂 Deploying new version..."
mkdir -p "${DEPLOY_DIR}"
cp "${BINARY_PATH}" "${DEPLOY_DIR}/"
chmod +x "${DEPLOY_DIR}/${APP_NAME}"

# Start new version
echo "🔄 Starting new version..."
cd "${DEPLOY_DIR}"
./${APP_NAME} &
NEW_PID=$!

# Verify deployment
echo "🔍 Verifying deployment..."
sleep 5
if kill -0 $NEW_PID 2>/dev/null; then
    echo "✅ Deployment successful!"
    if [ -d "${BACKUP_DIR}" ]; then
        echo "🗑️ Cleaning up backup..."
        rm -rf "${BACKUP_DIR}"
    fi
else
    echo "❌ Deployment failed! Rolling back..."
    kill $NEW_PID 2>/dev/null || true
    if [ -d "${BACKUP_DIR}" ]; then
        mv "${BACKUP_DIR}" "${DEPLOY_DIR}"
        cd "${DEPLOY_DIR}" && ./${APP_NAME} &
    fi
    exit 1
fi

echo "🎉 Deployment completed successfully!"
```

## Native Compilation

### Single Binary Deployment

**Basic Compilation:**
```bash
# Compile for current platform
cursed-zig --compile main.csd                    # Output: main-native

# Compile with optimizations
cursed-zig --compile --optimize main.csd         # Maximum performance

# Compile with debug info
cursed-zig --compile --debug main.csd            # For debugging in production
```

**Cross-Compilation:**
```bash
# Linux x86_64
cursed-zig --compile --target=x86_64-linux main.csd

# Linux ARM64 (for AWS Graviton, Apple Silicon servers)
cursed-zig --compile --target=aarch64-linux main.csd

# macOS (Intel)
cursed-zig --compile --target=x86_64-macos main.csd

# macOS (Apple Silicon)
cursed-zig --compile --target=aarch64-macos main.csd

# Windows
cursed-zig --compile --target=x86_64-windows main.csd

# Embedded targets
cursed-zig --compile --target=arm-linux-gnueabihf main.csd
```

**Advanced Compilation Options:**
```bash
# Profile-Guided Optimization (PGO)
cursed-zig --compile --pgo-generate main.csd     # Generate profile data
./main-native < typical_workload_input.txt       # Run with typical workload
cursed-zig --compile --pgo-use main.csd          # Optimize with profile data

# Link-Time Optimization (LTO)
cursed-zig --compile --lto main.csd              # Aggressive optimization

# Static linking (no external dependencies)
cursed-zig --compile --static main.csd           # Self-contained binary

# Strip debug symbols (smaller binary)
cursed-zig --compile --strip main.csd            # Minimal binary size
```

### Multi-Platform Build Script

```bash
#!/bin/bash
# build-all-targets.sh - Build for all supported platforms

set -euo pipefail

APP_NAME="my-cursed-app"
SOURCE="main.csd"
BUILD_DIR="dist"

# Supported targets
TARGETS=(
    "x86_64-linux"
    "aarch64-linux"
    "x86_64-macos"
    "aarch64-macos"
    "x86_64-windows"
)

echo "🏗️ Building ${APP_NAME} for all platforms..."

mkdir -p "${BUILD_DIR}"

for target in "${TARGETS[@]}"; do
    echo "📦 Building for ${target}..."
    
    output_name="${APP_NAME}-${target}"
    if [[ $target == *"windows"* ]]; then
        output_name="${output_name}.exe"
    fi
    
    cursed-zig --compile --optimize --target="${target}" \
               --output="${BUILD_DIR}/${output_name}" "${SOURCE}"
    
    echo "✅ ${target} build complete: ${BUILD_DIR}/${output_name}"
done

echo "🎉 All builds completed!"

# Generate checksums
cd "${BUILD_DIR}"
sha256sum * > checksums.txt
echo "🔐 Checksums generated: ${BUILD_DIR}/checksums.txt"
```

### Optimized Production Builds

**Performance-Critical Applications:**
```bash
# Maximum performance build
cursed-zig --compile \
    --optimize=ReleaseFast \
    --lto \
    --pgo-use \
    --target=native \
    --cpu=native \
    main.csd

# The resulting binary will be optimized for:
# - Maximum execution speed
# - Link-time optimization across modules
# - Profile-guided optimization
# - Native CPU features utilization
```

**Size-Optimized Builds:**
```bash
# Minimum size build
cursed-zig --compile \
    --optimize=ReleaseSmall \
    --strip \
    --compress \
    main.csd

# Typical size reductions:
# - Standard build: 15-25 MB
# - Size-optimized: 8-12 MB
# - Compressed: 3-5 MB
```

## WebAssembly Deployment

### Browser Applications

**Basic WASM Compilation:**
```bash
# Compile to WebAssembly
cursed-zig --compile --target=wasm32-wasi main.csd

# Output files:
# - main.wasm (WebAssembly module)
# - main.js (JavaScript wrapper)
```

**Web Application Structure:**
```html
<!DOCTYPE html>
<html>
<head>
    <title>CURSED Web Application</title>
</head>
<body>
    <div id="output"></div>
    <script type="module">
        import init, { run_cursed_app } from './main.js';
        
        async function main() {
            await init();
            
            // Override vibez.spill to write to web page
            const output = document.getElementById('output');
            window.cursed_output = (text) => {
                output.innerHTML += text + '<br>';
            };
            
            run_cursed_app();
        }
        
        main();
    </script>
</body>
</html>
```

**Advanced WASM Features:**
```javascript
// main.js - Advanced WASM integration
import init, { 
    run_cursed_app,
    call_cursed_function,
    get_cursed_memory
} from './cursed-app.js';

class CursedApp {
    constructor() {
        this.memory = null;
        this.instance = null;
    }
    
    async initialize() {
        const wasm = await init();
        this.instance = wasm;
        this.memory = this.instance.memory;
        
        // Set up WASI bindings
        this.setupWASI();
    }
    
    setupWASI() {
        // Implement file system interface
        this.instance.wasiBindings = {
            fd_read: this.fdRead.bind(this),
            fd_write: this.fdWrite.bind(this),
            random_get: this.randomGet.bind(this),
        };
    }
    
    fdWrite(fd, iovs_ptr, iovs_len, nwritten_ptr) {
        // Handle output to browser console or DOM
        const buffer = new Uint8Array(this.memory.buffer);
        let text = '';
        
        for (let i = 0; i < iovs_len; i++) {
            const ptr = iovs_ptr + i * 8;
            const buf_ptr = new Uint32Array(buffer, ptr, 1)[0];
            const buf_len = new Uint32Array(buffer, ptr + 4, 1)[0];
            const str = new TextDecoder().decode(buffer.subarray(buf_ptr, buf_ptr + buf_len));
            text += str;
        }
        
        console.log(text);
        return 0; // Success
    }
    
    async runApplication(input) {
        const result = await call_cursed_function('main', input);
        return result;
    }
}

// Usage
const app = new CursedApp();
await app.initialize();
const result = await app.runApplication('user input');
```

### Server-Side WebAssembly (WASI)

**Node.js Deployment:**
```javascript
// server.js - Node.js WASM server
import { readFile } from 'fs/promises';
import { WASI } from 'wasi';
import { argv, env } from 'process';

async function runCursedApp() {
    const wasi = new WASI({
        args: argv,
        env,
        preopens: {
            '/sandbox': '/tmp',
        },
    });
    
    const wasm = await WebAssembly.compile(
        await readFile('./main.wasm')
    );
    
    const instance = await WebAssembly.instantiate(wasm, {
        wasi_snapshot_preview1: wasi.wasiImport,
    });
    
    wasi.start(instance);
}

runCursedApp().catch(console.error);
```

**Deno Deployment:**
```typescript
// server.ts - Deno WASM server
import { serve } from "https://deno.land/std@0.190.0/http/server.ts";

const wasmModule = await WebAssembly.compileStreaming(
    fetch("./main.wasm")
);

async function handler(request: Request): Promise<Response> {
    const instance = await WebAssembly.instantiate(wasmModule, {
        // WASI bindings
        wasi_snapshot_preview1: {
            fd_write: (fd: number, iovs: number, iovs_len: number, nwritten: number) => {
                // Handle output
                return 0;
            },
            proc_exit: (code: number) => {
                // Handle exit
            },
        },
    });
    
    // Call CURSED application
    const result = (instance.exports as any).main();
    
    return new Response(`CURSED result: ${result}`, {
        headers: { "Content-Type": "text/plain" },
    });
}

serve(handler, { port: 8080 });
```

### Edge Computing Deployment

**Cloudflare Workers:**
```javascript
// worker.js - Cloudflare Workers deployment
import wasmModule from './main.wasm';

export default {
    async fetch(request) {
        const instance = await WebAssembly.instantiate(wasmModule, {
            env: {
                cursed_log: (ptr, len) => {
                    const buffer = new Uint8Array(instance.exports.memory.buffer);
                    const text = new TextDecoder().decode(buffer.subarray(ptr, ptr + len));
                    console.log(text);
                },
            },
        });
        
        const result = instance.exports.main();
        
        return new Response(`Result: ${result}`, {
            headers: { 'Content-Type': 'text/plain' },
        });
    },
};
```

**Vercel Edge Functions:**
```typescript
// api/cursed-app.ts
import type { NextRequest } from 'next/server';

export const config = {
    runtime: 'edge',
};

export default async function handler(request: NextRequest) {
    const wasmResponse = await fetch(new URL('./main.wasm', import.meta.url));
    const wasmBytes = await wasmResponse.arrayBuffer();
    const wasmModule = await WebAssembly.instantiate(wasmBytes);
    
    const result = (wasmModule.instance.exports as any).main();
    
    return new Response(JSON.stringify({ result }), {
        headers: { 'Content-Type': 'application/json' },
    });
}
```

## Container Deployment

### Docker Deployment

**Multi-stage Dockerfile:**
```dockerfile
# Dockerfile - Optimized CURSED application container
FROM ubuntu:22.04 AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

# Install Zig
RUN curl -L https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz | tar -xJ \
    && mv zig-linux-x86_64-0.13.0 /opt/zig
ENV PATH="/opt/zig:$PATH"

# Install CURSED compiler
COPY . /src
WORKDIR /src
RUN zig build

# Build application
COPY app/ /app
WORKDIR /app
RUN /src/zig-out/bin/cursed-zig --compile --optimize main.csd

# Production stage
FROM ubuntu:22.04 AS runtime

# Install runtime dependencies (minimal)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/false cursed

# Copy application binary
COPY --from=builder /app/main-native /usr/local/bin/cursed-app
RUN chmod +x /usr/local/bin/cursed-app

# Security: Run as non-root user
USER cursed
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD cursed-app --health-check || exit 1

CMD ["cursed-app"]
```

**Docker Compose for Development:**
```yaml
# docker-compose.yml
version: '3.8'

services:
  cursed-app:
    build:
      context: .
      dockerfile: Dockerfile
      target: runtime
    ports:
      - "8080:8080"
    environment:
      - ENV=production
      - LOG_LEVEL=info
    volumes:
      - app-logs:/var/log/cursed-app
    healthcheck:
      test: ["CMD", "cursed-app", "--health-check"]
      interval: 30s
      timeout: 10s
      retries: 3
    restart: unless-stopped
    deploy:
      resources:
        limits:
          cpus: '1.0'
          memory: 512M
        reservations:
          cpus: '0.5'
          memory: 256M

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis-data:/data

  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: cursed_app
      POSTGRES_USER: cursed
      POSTGRES_PASSWORD: secure_password
    volumes:
      - postgres-data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

volumes:
  app-logs:
  redis-data:
  postgres-data:
```

### Kubernetes Deployment

**Deployment Manifest:**
```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cursed-app
  labels:
    app: cursed-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: cursed-app
  template:
    metadata:
      labels:
        app: cursed-app
    spec:
      containers:
      - name: cursed-app
        image: your-registry/cursed-app:latest
        ports:
        - containerPort: 8080
        env:
        - name: ENV
          value: "production"
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: cursed-app-secrets
              key: database-url
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
        securityContext:
          runAsNonRoot: true
          runAsUser: 1000
          allowPrivilegeEscalation: false
          readOnlyRootFilesystem: true
      imagePullSecrets:
      - name: registry-secret
---
apiVersion: v1
kind: Service
metadata:
  name: cursed-app-service
spec:
  selector:
    app: cursed-app
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8080
  type: LoadBalancer
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: cursed-app-ingress
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts:
    - api.example.com
    secretName: cursed-app-tls
  rules:
  - host: api.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: cursed-app-service
            port:
              number: 80
```

**Horizontal Pod Autoscaler:**
```yaml
# k8s/hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: cursed-app-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: cursed-app
  minReplicas: 3
  maxReplicas: 20
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

## Cloud Platform Integration

### AWS Deployment

**Lambda Function:**
```bash
# Build for Lambda
cursed-zig --compile --target=x86_64-linux --optimize main.csd

# Create deployment package
mkdir lambda-package
cp main-native lambda-package/bootstrap
chmod +x lambda-package/bootstrap

cd lambda-package
zip -r ../cursed-lambda.zip .
cd ..

# Deploy with AWS CLI
aws lambda create-function \
    --function-name cursed-app \
    --runtime provided.al2 \
    --role arn:aws:iam::ACCOUNT:role/lambda-execution-role \
    --handler bootstrap \
    --zip-file fileb://cursed-lambda.zip \
    --timeout 30 \
    --memory-size 512
```

**ECS Deployment:**
```json
{
    "family": "cursed-app",
    "taskRoleArn": "arn:aws:iam::ACCOUNT:role/cursed-task-role",
    "executionRoleArn": "arn:aws:iam::ACCOUNT:role/cursed-execution-role",
    "networkMode": "awsvpc",
    "requiresCompatibilities": ["FARGATE"],
    "cpu": "1024",
    "memory": "2048",
    "containerDefinitions": [
        {
            "name": "cursed-app",
            "image": "ACCOUNT.dkr.ecr.REGION.amazonaws.com/cursed-app:latest",
            "portMappings": [
                {
                    "containerPort": 8080,
                    "protocol": "tcp"
                }
            ],
            "essential": true,
            "logConfiguration": {
                "logDriver": "awslogs",
                "options": {
                    "awslogs-group": "/ecs/cursed-app",
                    "awslogs-region": "us-west-2",
                    "awslogs-stream-prefix": "ecs"
                }
            },
            "environment": [
                {
                    "name": "ENV",
                    "value": "production"
                }
            ],
            "secrets": [
                {
                    "name": "DATABASE_PASSWORD",
                    "valueFrom": "arn:aws:secretsmanager:REGION:ACCOUNT:secret:cursed-db-password"
                }
            ]
        }
    ]
}
```

### Google Cloud Platform

**Cloud Run Deployment:**
```yaml
# service.yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: cursed-app
  annotations:
    run.googleapis.com/ingress: all
spec:
  template:
    metadata:
      annotations:
        autoscaling.knative.dev/maxScale: "1000"
        run.googleapis.com/cpu-throttling: "false"
        run.googleapis.com/execution-environment: gen2
    spec:
      containerConcurrency: 100
      timeoutSeconds: 300
      containers:
      - image: gcr.io/PROJECT-ID/cursed-app:latest
        ports:
        - containerPort: 8080
        env:
        - name: ENV
          value: production
        resources:
          limits:
            cpu: 2000m
            memory: 4Gi
        startupProbe:
          httpGet:
            path: /health
          periodSeconds: 1
          failureThreshold: 30
```

**Compute Engine with Managed Instance Groups:**
```bash
#!/bin/bash
# startup-script.sh for Compute Engine instances

set -euo pipefail

# Install dependencies
apt-get update
apt-get install -y curl unzip

# Download and install CURSED application
curl -L "https://github.com/your-org/cursed-app/releases/latest/download/cursed-app-linux-x86_64" \
     -o /usr/local/bin/cursed-app
chmod +x /usr/local/bin/cursed-app

# Create systemd service
cat > /etc/systemd/system/cursed-app.service << EOF
[Unit]
Description=CURSED Application
After=network.target

[Service]
Type=simple
User=cursed-app
ExecStart=/usr/local/bin/cursed-app
Restart=always
RestartSec=10
Environment=ENV=production

[Install]
WantedBy=multi-user.target
EOF

# Create service user
useradd -r -s /bin/false cursed-app

# Start service
systemctl enable cursed-app
systemctl start cursed-app

# Configure health check
cat > /usr/local/bin/health-check.sh << 'EOF'
#!/bin/bash
curl -f http://localhost:8080/health || exit 1
EOF
chmod +x /usr/local/bin/health-check.sh
```

### Microsoft Azure

**Container Apps:**
```yaml
# container-app.yaml
apiVersion: 2022-03-01
type: Microsoft.App/containerApps
properties:
  managedEnvironmentId: /subscriptions/SUBSCRIPTION/resourceGroups/RG/providers/Microsoft.App/managedEnvironments/cursed-env
  configuration:
    activeRevisionsMode: Single
    ingress:
      external: true
      targetPort: 8080
      traffic:
      - weight: 100
        latestRevision: true
    secrets:
    - name: database-password
      value: your-secret-value
  template:
    containers:
    - image: your-registry.azurecr.io/cursed-app:latest
      name: cursed-app
      resources:
        cpu: 1.0
        memory: 2Gi
      env:
      - name: ENV
        value: production
      - name: DATABASE_PASSWORD
        secretRef: database-password
    scale:
      minReplicas: 2
      maxReplicas: 10
      rules:
      - name: http-scaling
        http:
          metadata:
            concurrentRequests: 50
```

## CI/CD Pipelines

### GitHub Actions

**Complete CI/CD Pipeline:**
```yaml
# .github/workflows/deploy.yml
name: Build and Deploy

on:
  push:
    branches: [main]
    tags: ['v*']
  pull_request:
    branches: [main]

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Zig
        uses: goto-bus-stop/setup-zig@v2
        with:
          version: 0.13.0
          
      - name: Build CURSED compiler
        run: zig build
        
      - name: Run tests
        run: |
          zig build test
          ./zig-out/bin/cursed-zig comprehensive_stdlib_test.csd
          
      - name: Memory safety check
        run: |
          sudo apt-get update && sudo apt-get install -y valgrind
          valgrind --error-exitcode=1 --leak-check=full \
            ./zig-out/bin/cursed-zig test_suite/basic_syntax.csd

  build:
    needs: test
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target:
          - x86_64-linux
          - aarch64-linux
          - x86_64-macos
          - aarch64-macos
          - x86_64-windows
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Zig
        uses: goto-bus-stop/setup-zig@v2
        with:
          version: 0.13.0
          
      - name: Build CURSED compiler
        run: zig build
        
      - name: Build application
        run: |
          ./zig-out/bin/cursed-zig --compile --optimize \
            --target=${{ matrix.target }} \
            --output=cursed-app-${{ matrix.target }} \
            main.csd
            
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: cursed-app-${{ matrix.target }}
          path: cursed-app-${{ matrix.target }}*

  docker:
    needs: test
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
        
      - name: Log in to Container Registry
        uses: docker/login-action@v3
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}
          
      - name: Extract metadata
        id: meta
        uses: docker/metadata-action@v5
        with:
          images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}
          tags: |
            type=ref,event=branch
            type=ref,event=pr
            type=semver,pattern={{version}}
            type=semver,pattern={{major}}.{{minor}}
            
      - name: Build and push Docker image
        uses: docker/build-push-action@v5
        with:
          context: .
          platforms: linux/amd64,linux/arm64
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
          cache-from: type=gha
          cache-to: type=gha,mode=max

  deploy-staging:
    needs: [build, docker]
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    environment: staging
    steps:
      - name: Deploy to staging
        run: |
          echo "Deploying to staging environment..."
          # Add your staging deployment commands here
          
  deploy-production:
    needs: [build, docker]
    if: startsWith(github.ref, 'refs/tags/v')
    runs-on: ubuntu-latest
    environment: production
    steps:
      - name: Deploy to production
        run: |
          echo "Deploying to production environment..."
          # Add your production deployment commands here

  release:
    needs: [build, docker]
    if: startsWith(github.ref, 'refs/tags/v')
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4
      
      - name: Download artifacts
        uses: actions/download-artifact@v3
        
      - name: Create release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            cursed-app-*/cursed-app-*
          generate_release_notes: true
```

### GitLab CI/CD

```yaml
# .gitlab-ci.yml
stages:
  - build
  - test
  - package
  - deploy

variables:
  DOCKER_IMAGE: $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA
  DOCKER_LATEST: $CI_REGISTRY_IMAGE:latest

build:
  stage: build
  image: ubuntu:22.04
  before_script:
    - apt-get update && apt-get install -y curl xz-utils
    - curl -L https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz | tar -xJ
    - export PATH="$(pwd)/zig-linux-x86_64-0.13.0:$PATH"
  script:
    - zig build
    - ./zig-out/bin/cursed-zig --compile --optimize main.csd
  artifacts:
    paths:
      - zig-out/
      - main-native
    expire_in: 1 hour

test:
  stage: test
  needs: [build]
  script:
    - zig build test
    - ./zig-out/bin/cursed-zig comprehensive_stdlib_test.csd

docker:
  stage: package
  image: docker:latest
  services:
    - docker:dind
  needs: [test]
  script:
    - docker login -u $CI_REGISTRY_USER -p $CI_REGISTRY_PASSWORD $CI_REGISTRY
    - docker build -t $DOCKER_IMAGE -t $DOCKER_LATEST .
    - docker push $DOCKER_IMAGE
    - docker push $DOCKER_LATEST
  only:
    - main
    - tags

deploy-staging:
  stage: deploy
  image: alpine/kubectl
  needs: [docker]
  environment:
    name: staging
    url: https://staging.example.com
  script:
    - kubectl set image deployment/cursed-app cursed-app=$DOCKER_IMAGE
    - kubectl rollout status deployment/cursed-app
  only:
    - main

deploy-production:
  stage: deploy
  image: alpine/kubectl
  needs: [docker]
  environment:
    name: production
    url: https://api.example.com
  script:
    - kubectl set image deployment/cursed-app cursed-app=$DOCKER_IMAGE
    - kubectl rollout status deployment/cursed-app
  when: manual
  only:
    - tags
```

## Performance Optimization

### Compilation Optimizations

**Profile-Guided Optimization (PGO):**
```bash
# Step 1: Build with profiling enabled
cursed-zig --compile --pgo-generate main.csd

# Step 2: Run typical workloads to collect profile data
./main-native < production_workload_1.txt
./main-native < production_workload_2.txt
./main-native < production_workload_3.txt

# Step 3: Build optimized version using profile data
cursed-zig --compile --pgo-use --optimize main.csd

# Expected improvements:
# - 15-25% performance improvement on hot paths
# - Better branch prediction
# - Optimized instruction cache usage
```

**Link-Time Optimization (LTO):**
```bash
# Enable LTO for maximum optimization
cursed-zig --compile --lto --optimize main.csd

# Benefits:
# - Cross-module inlining
# - Dead code elimination across modules
# - Better constant propagation
# - 5-15% performance improvement typical
```

### Runtime Performance Tuning

**Memory Management Optimization:**
```cursed
yeet "memz"

fr fr Use arena allocators for bulk operations
slay process_large_dataset(data []Item) []Result {
    sus arena Arena = Arena.init()
    defer arena.cleanup()
    
    sus results []Result = arena.alloc_array<Result>(data.length())
    
    bestie i := 0; i < data.length(); i++ {
        results[i] = process_item(data[i])
    }
    
    damn results.copy()  fr fr Copy to heap before arena cleanup
}

fr fr Pool allocations for frequent allocations/deallocations
sus pool ObjectPool<Connection> = ObjectPool.init<Connection>()

slay handle_request() {
    sus conn Connection = pool.acquire()
    defer pool.release(conn)
    
    fr fr Use connection
}
```

**Concurrency Optimization:**
```cursed
yeet "concurrenz"

fr fr Use worker pools for CPU-intensive tasks
sus worker_pool WorkerPool = WorkerPool.init(8)  fr fr 8 worker goroutines
defer worker_pool.cleanup()

slay process_items_parallel(items []Item) []Result {
    sus results_chan chan<Result> = make_channel<Result>()
    sus workers_done drip = 0
    
    fr fr Distribute work among workers
    bestie item in items {
        worker_pool.submit({
            sus result Result = process_item(item)
            results_chan <- result
        })
    }
    
    fr fr Collect results
    sus results []Result = []
    bestie i := 0; i < items.length(); i++ {
        results.push(<-results_chan)
    }
    
    damn results
}
```

**Database Connection Optimization:**
```cursed
yeet "dbz"

fr fr Connection pooling for database operations
squad DatabaseManager {
    pool ConnectionPool,
    
    slay init(config DatabaseConfig) DatabaseManager {
        damn DatabaseManager{
            pool: ConnectionPool.init(config.max_connections)
        }
    }
    
    slay execute_query<T>(self, query tea, params []QueryParam) []T {
        sus conn Connection = self.pool.acquire()
        defer self.pool.release(conn)
        
        damn conn.query<T>(query, params)
    }
}

fr fr Prepared statements for frequently executed queries
sus prepared_stmt PreparedStatement = db.prepare("SELECT * FROM users WHERE id = ?")
defer prepared_stmt.cleanup()

bestie user_id in user_ids {
    sus user User = prepared_stmt.execute<User>([user_id])
    process_user(user)
}
```

### Application-Level Optimization

**Caching Strategies:**
```cursed
yeet "cachez"

fr fr In-memory caching with TTL
sus cache LRUCache<tea, User> = LRUCache.init(1000)  fr fr 1000 entries max

slay get_user_cached(user_id tea) User {
    ready (cache.contains(user_id)) {
        damn cache.get(user_id)
    }
    
    sus user User = database.get_user(user_id)
    cache.set(user_id, user, 300)  fr fr 5 minute TTL
    damn user
}

fr fr Redis caching for distributed systems
yeet "redisz"

sus redis RedisClient = RedisClient.connect("redis://localhost:6379")

slay get_user_distributed(user_id tea) yikes<tea> {
    fr fr Try Redis first
    sus cached_user tea = redis.get("user:{user_id}") fam {
        when "NotFound" -> {
            fr fr Cache miss - get from database
            sus user User = database.get_user(user_id)
            sus user_json tea = jsonz.serialize(user)
            redis.setex("user:{user_id}", 300, user_json)
            damn user
        }
        when e -> shook e
    }
    
    damn jsonz.deserialize<User>(cached_user)
}
```

### Monitoring Performance in Production

**Built-in Performance Metrics:**
```cursed
yeet "metricz"

squad PerformanceMonitor {
    request_count drip,
    response_times []normie,
    memory_usage drip,
    
    slay record_request(self, duration normie) {
        self.request_count++
        self.response_times.push(duration)
        
        ready (self.response_times.length() > 1000) {
            fr fr Keep only last 1000 samples
            self.response_times = self.response_times[500..]
        }
    }
    
    slay get_metrics(self) Metrics {
        damn Metrics{
            total_requests: self.request_count,
            avg_response_time: average(self.response_times),
            p95_response_time: percentile(self.response_times, 95),
            current_memory_mb: self.memory_usage / 1024 / 1024
        }
    }
}

sus monitor PerformanceMonitor = PerformanceMonitor{}

fr fr Middleware for HTTP requests
slay request_handler(request Request) Response {
    sus start_time normie = timez.now_ms()
    
    sus response Response = handle_request(request)
    
    sus duration normie = timez.now_ms() - start_time
    monitor.record_request(duration)
    
    damn response
}
```

## Monitoring and Observability

### Application Metrics

**Prometheus Integration:**
```cursed
yeet "promz"

squad MetricsCollector {
    request_counter Counter,
    response_histogram Histogram,
    active_connections_gauge Gauge,
    
    slay init() MetricsCollector {
        damn MetricsCollector{
            request_counter: Counter.new("http_requests_total", "Total HTTP requests"),
            response_histogram: Histogram.new("http_request_duration_seconds", "HTTP request duration"),
            active_connections_gauge: Gauge.new("active_connections", "Active connections")
        }
    }
    
    slay record_request(self, method tea, status drip, duration normie) {
        self.request_counter.inc_with_labels([
            ("method", method),
            ("status", status.to_string())
        ])
        self.response_histogram.observe(duration)
    }
}

sus metrics MetricsCollector = MetricsCollector.init()

fr fr Expose metrics endpoint
slay metrics_handler() tea {
    damn promz.export_metrics(metrics)
}
```

**Structured Logging:**
```cursed
yeet "logz"

squad Logger {
    level LogLevel,
    output LogOutput,
    
    slay info(self, message tea, fields []Field) {
        self.log(LogLevel.Info, message, fields)
    }
    
    slay error(self, message tea, error tea, fields []Field) {
        sus error_fields []Field = fields.copy()
        error_fields.push(Field{ key: "error", value: error })
        self.log(LogLevel.Error, message, error_fields)
    }
    
    slay log(self, level LogLevel, message tea, fields []Field) {
        ready (level < self.level) { damn }
        
        sus log_entry LogEntry = LogEntry{
            timestamp: timez.now_iso(),
            level: level,
            message: message,
            fields: fields
        }
        
        self.output.write(jsonz.serialize(log_entry))
    }
}

sus logger Logger = Logger{
    level: LogLevel.Info,
    output: LogOutput.Stdout
}

fr fr Usage in application
slay handle_user_request(user_id tea, request Request) {
    logger.info("Processing user request", [
        Field{ key: "user_id", value: user_id },
        Field{ key: "endpoint", value: request.path },
        Field{ key: "method", value: request.method }
    ])
    
    sus start_time normie = timez.now_ms()
    
    sus response Response = process_request(request) fam {
        when e -> {
            logger.error("Request processing failed", e, [
                Field{ key: "user_id", value: user_id },
                Field{ key: "duration_ms", value: (timez.now_ms() - start_time).to_string() }
            ])
            shook e
        }
    }
    
    logger.info("Request completed successfully", [
        Field{ key: "user_id", value: user_id },
        Field{ key: "status_code", value: response.status.to_string() },
        Field{ key: "duration_ms", value: (timez.now_ms() - start_time).to_string() }
    ])
}
```

### Health Checks and Readiness Probes

**Comprehensive Health Checks:**
```cursed
yeet "healthz"

squad HealthChecker {
    database_pool DatabasePool,
    redis_client RedisClient,
    external_services []ServiceEndpoint,
    
    slay check_health(self) HealthStatus {
        sus checks []HealthCheck = []
        
        fr fr Database connectivity
        checks.push(self.check_database())
        
        fr fr Redis connectivity
        checks.push(self.check_redis())
        
        fr fr External service dependencies
        bestie service in self.external_services {
            checks.push(self.check_external_service(service))
        }
        
        fr fr System resources
        checks.push(self.check_memory_usage())
        checks.push(self.check_disk_space())
        
        sus overall_status HealthStatus = HealthStatus.Healthy
        bestie check in checks {
            ready (check.status == HealthStatus.Unhealthy) {
                overall_status = HealthStatus.Unhealthy
                break
            } otherwise ready (check.status == HealthStatus.Degraded) {
                overall_status = HealthStatus.Degraded
            }
        }
        
        damn overall_status
    }
    
    slay check_database(self) HealthCheck {
        sus check HealthCheck = HealthCheck{ name: "database" }
        
        sus start_time normie = timez.now_ms()
        
        sus result = self.database_pool.ping() fam {
            when e -> {
                check.status = HealthStatus.Unhealthy
                check.message = "Database connection failed: {e}"
                damn check
            }
        }
        
        sus duration normie = timez.now_ms() - start_time
        
        ready (duration > 1000) {  fr fr > 1 second is concerning
            check.status = HealthStatus.Degraded
            check.message = "Database response slow: {duration}ms"
        } otherwise {
            check.status = HealthStatus.Healthy
            check.message = "Database healthy: {duration}ms"
        }
        
        damn check
    }
}

sus health_checker HealthChecker = HealthChecker{
    database_pool: app.database_pool,
    redis_client: app.redis_client,
    external_services: app.external_services
}

fr fr Health check endpoint
slay health_endpoint() Response {
    sus status HealthStatus = health_checker.check_health()
    
    sus response_code drip = sick status {
        when HealthStatus.Healthy -> 200
        when HealthStatus.Degraded -> 200  fr fr Still serving traffic
        when HealthStatus.Unhealthy -> 503
    }
    
    damn Response{
        status: response_code,
        headers: [("Content-Type", "application/json")],
        body: jsonz.serialize(status)
    }
}
```

### Distributed Tracing

**OpenTelemetry Integration:**
```cursed
yeet "tracez"

squad TraceSpan {
    trace_id tea,
    span_id tea,
    parent_span_id tea,
    operation_name tea,
    start_time drip,
    end_time drip,
    tags []TraceTag,
    
    slay finish(self) {
        self.end_time = timez.now_micro()
        tracez.export_span(self)
    }
    
    slay set_tag(self, key tea, value tea) {
        self.tags.push(TraceTag{ key: key, value: value })
    }
}

slay start_span(operation_name tea, parent TraceSpan) TraceSpan {
    damn TraceSpan{
        trace_id: parent.trace_id,
        span_id: tracez.generate_span_id(),
        parent_span_id: parent.span_id,
        operation_name: operation_name,
        start_time: timez.now_micro(),
        tags: []
    }
}

fr fr Usage in application
slay process_order(order Order, trace TraceSpan) yikes<tea> {
    sus span TraceSpan = start_span("process_order", trace)
    defer span.finish()
    
    span.set_tag("order_id", order.id)
    span.set_tag("customer_id", order.customer_id)
    
    fr fr Validate order
    sus validation_span TraceSpan = start_span("validate_order", span)
    validate_order(order) fam {
        when e -> {
            validation_span.set_tag("error", e)
            validation_span.finish()
            shook e
        }
    }
    validation_span.finish()
    
    fr fr Process payment
    sus payment_span TraceSpan = start_span("process_payment", span)
    process_payment(order.payment_info) fam {
        when e -> {
            payment_span.set_tag("error", e)
            payment_span.finish()
            shook e
        }
    }
    payment_span.finish()
    
    span.set_tag("status", "completed")
}
```

## Security Best Practices

### Application Security

**Input Validation and Sanitization:**
```cursed
yeet "validatorz"

squad InputValidator {
    slay validate_email(email tea) yikes<tea> {
        ready (email.length() == 0) {
            yikes "Email cannot be empty"
        }
        
        ready (!email.contains("@")) {
            yikes "Invalid email format"
        }
        
        fr fr More comprehensive email validation
        sus email_regex Regex = Regex.compile(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        ready (!email_regex.matches(email)) {
            yikes "Invalid email format"
        }
    }
    
    slay validate_user_input(input tea) tea {
        fr fr Remove dangerous characters
        sus sanitized tea = input
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("&", "&amp;")
            .replace("'", "&#39;")
            .replace("\"", "&quot;")
        
        fr fr Limit length
        ready (sanitized.length() > 1000) {
            sanitized = sanitized[0..1000]
        }
        
        damn sanitized
    }
    
    slay validate_sql_safe(query tea) yikes<tea> {
        sus dangerous_keywords []tea = [
            "DROP", "DELETE", "TRUNCATE", "ALTER", "CREATE",
            "EXEC", "EXECUTE", "UNION", "INSERT", "UPDATE"
        ]
        
        sus upper_query tea = query.upper()
        
        bestie keyword in dangerous_keywords {
            ready (upper_query.contains(keyword)) {
                yikes "Potentially dangerous SQL keyword detected: {keyword}"
            }
        }
    }
}

sus validator InputValidator = InputValidator{}
```

**Authentication and Authorization:**
```cursed
yeet "authz"
yeet "jwtiz"

squad AuthManager {
    jwt_secret tea,
    token_expiry drip,
    
    slay authenticate_user(username tea, password tea) yikes<tea> {
        fr fr Hash password and compare with stored hash
        sus stored_hash tea = database.get_password_hash(username) fam {
            when "UserNotFound" -> yikes "Invalid credentials"
            when e -> shook e
        }
        
        ready (!cryptz.bcrypt_verify(password, stored_hash)) {
            yikes "Invalid credentials"
        }
        
        fr fr Generate JWT token
        sus claims JWTClaims = JWTClaims{
            sub: username,
            exp: timez.now() + self.token_expiry,
            iat: timez.now()
        }
        
        damn jwtiz.sign(claims, self.jwt_secret)
    }
    
    slay authorize_request(token tea, required_role tea) yikes<tea> {
        sus claims JWTClaims = jwtiz.verify(token, self.jwt_secret) fam {
            when e -> yikes "Invalid or expired token"
        }
        
        ready (claims.exp < timez.now()) {
            yikes "Token expired"
        }
        
        sus user_roles []tea = database.get_user_roles(claims.sub)
        ready (!user_roles.contains(required_role)) {
            yikes "Insufficient permissions"
        }
    }
}

sus auth_manager AuthManager = AuthManager{
    jwt_secret: env.get("JWT_SECRET"),
    token_expiry: 3600  fr fr 1 hour
}
```

**Rate Limiting:**
```cursed
yeet "ratez"

squad RateLimiter {
    limits Map<tea, RateLimit>,
    
    slay check_rate_limit(self, client_ip tea, endpoint tea) yikes<tea> {
        sus key tea = "{client_ip}:{endpoint}"
        sus limit RateLimit = self.limits.get(key)
        
        ready (limit == null) {
            limit = RateLimit{
                requests: 0,
                window_start: timez.now(),
                max_requests: 100,  fr fr 100 requests per minute
                window_duration: 60
            }
            self.limits.set(key, limit)
        }
        
        sus current_time drip = timez.now()
        
        fr fr Reset window if expired
        ready (current_time - limit.window_start > limit.window_duration) {
            limit.requests = 0
            limit.window_start = current_time
        }
        
        ready (limit.requests >= limit.max_requests) {
            yikes "Rate limit exceeded"
        }
        
        limit.requests++
    }
}

sus rate_limiter RateLimiter = RateLimiter{ limits: Map.init() }

fr fr Middleware
slay rate_limit_middleware(request Request, next slay(Request) Response) Response {
    sus client_ip tea = request.get_client_ip()
    
    rate_limiter.check_rate_limit(client_ip, request.path) fam {
        when e -> {
            damn Response{
                status: 429,
                headers: [("Retry-After", "60")],
                body: "Rate limit exceeded"
            }
        }
    }
    
    damn next(request)
}
```

### Cryptographic Security

**Secure Password Handling:**
```cursed
yeet "cryptz"

slay hash_password(password tea) tea {
    fr fr Use bcrypt with cost factor 12 (secure for 2025)
    damn cryptz.bcrypt_hash(password, 12)
}

slay verify_password(password tea, hash tea) lit {
    damn cryptz.bcrypt_verify(password, hash)
}

slay generate_secure_token() tea {
    fr fr Generate 32-byte random token
    sus random_bytes []drip = cryptz.random_bytes(32)
    damn cryptz.base64_encode(random_bytes)
}
```

**Data Encryption:**
```cursed
squad DataEncryption {
    master_key []drip,
    
    slay encrypt_sensitive_data(self, data tea) tea {
        fr fr Generate random IV for each encryption
        sus iv []drip = cryptz.random_bytes(16)
        sus encrypted []drip = cryptz.aes_encrypt(data.bytes(), self.master_key, iv)
        
        fr fr Combine IV + encrypted data
        sus combined []drip = iv.concat(encrypted)
        damn cryptz.base64_encode(combined)
    }
    
    slay decrypt_sensitive_data(self, encrypted_data tea) yikes<tea> {
        sus combined []drip = cryptz.base64_decode(encrypted_data)
        
        ready (combined.length() < 16) {
            yikes "Invalid encrypted data"
        }
        
        sus iv []drip = combined[0..16]
        sus encrypted []drip = combined[16..]
        
        sus decrypted []drip = cryptz.aes_decrypt(encrypted, self.master_key, iv) fam {
            when e -> shook "Decryption failed"
        }
        
        damn tea.from_bytes(decrypted)
    }
}
```

### Environment Security

**Secrets Management:**
```cursed
yeet "secretz"

squad SecretsManager {
    vault_client VaultClient,
    
    slay get_secret(self, key tea) yikes<tea> {
        fr fr Try environment variable first
        sus env_value tea = env.get(key)
        ready (env_value.length() > 0) {
            damn env_value
        }
        
        fr fr Fall back to vault
        damn self.vault_client.get_secret(key)
    }
    
    slay rotate_secret(self, key tea) yikes<tea> {
        sus new_secret tea = cryptz.generate_secure_token()
        self.vault_client.set_secret(key, new_secret) fam {
            when e -> shook "Failed to store new secret: {e}"
        }
        damn new_secret
    }
}

sus secrets SecretsManager = SecretsManager{
    vault_client: VaultClient.connect(env.get("VAULT_URL"))
}

fr fr Usage
sus database_password tea = secrets.get_secret("DATABASE_PASSWORD") fam {
    when e -> {
        vibez.error("Failed to get database password: {e}")
        exit(1)
    }
}
```

**Security Headers:**
```cursed
slay security_headers_middleware(request Request, next slay(Request) Response) Response {
    sus response Response = next(request)
    
    fr fr Add security headers
    response.headers.push(("X-Frame-Options", "DENY"))
    response.headers.push(("X-Content-Type-Options", "nosniff"))
    response.headers.push(("X-XSS-Protection", "1; mode=block"))
    response.headers.push(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
    response.headers.push(("Content-Security-Policy", "default-src 'self'"))
    response.headers.push(("Referrer-Policy", "strict-origin-when-cross-origin"))
    
    damn response
}
```

---

## Conclusion

CURSED provides a complete production deployment ecosystem with:

- **🚀 Fast Deployment**: Sub-second compilation and single binary deployment
- **🌐 Universal Compatibility**: Native binaries and WebAssembly for all platforms
- **🔧 Professional Tooling**: Complete CI/CD integration and monitoring
- **📊 Production Observability**: Comprehensive metrics, logging, and tracing
- **🛡️ Enterprise Security**: Built-in authentication, encryption, and security best practices
- **⚡ High Performance**: Optimized compilation and runtime performance tuning

Whether you're deploying a simple web service or a complex distributed system, CURSED's deployment ecosystem provides the tools and practices you need for successful production operations.

**Key Advantages:**
- **Zero Downtime Deployments**: Built-in strategies for seamless updates
- **Cross-Platform**: Deploy anywhere with consistent behavior
- **Security First**: Comprehensive security practices built-in
- **Monitoring Ready**: Full observability out of the box
- **Cloud Native**: First-class support for all major cloud platforms

Start deploying CURSED applications to production with confidence! 🎉

---

**Documentation Version**: 1.0.0  
**Last Updated**: August 21, 2025  
**Support**: [Discord](https://discord.gg/cursed-lang) | [GitHub Issues](https://github.com/ghuntley/cursed/issues) | [Documentation](https://docs.cursedlang.org)
