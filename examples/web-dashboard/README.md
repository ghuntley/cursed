# CURSED Web Dashboard Application

A comprehensive real-world web application demonstrating CURSED's full-stack capabilities.

## Architecture

This application showcases:

1. **Backend API Server** - RESTful API with WebSocket support
2. **Frontend WebAssembly Module** - Interactive dashboard UI  
3. **CLI Management Tool** - System administration utilities
4. **Shared Libraries** - Common functionality across components

## Components

### Backend (`backend/`)
- HTTP API server with JSON endpoints
- User authentication and session management
- Real-time WebSocket connections
- File-based database storage
- Metrics collection and analysis

### Frontend (`frontend/`)
- WebAssembly-based dashboard UI
- Real-time data visualization
- User authentication interface
- WebSocket client for live updates

### CLI (`cli/`)
- User management commands
- Database administration
- System monitoring tools
- Deployment utilities

### Shared (`shared/`)
- Common data models
- Utility functions
- Configuration management
- Error handling

## Features

- ✅ User registration and authentication
- ✅ Real-time metrics dashboard
- ✅ File upload and download
- ✅ Chat/messaging system
- ✅ Admin panel for user management
- ✅ WebSocket real-time updates
- ✅ JSON API endpoints
- ✅ Secure session management

## Building and Running

```bash
# Build all components
zig build

# Run backend server
./zig-out/bin/cursed-zig examples/web-dashboard/backend/server.csd

# Run CLI tool
./zig-out/bin/cursed-zig examples/web-dashboard/cli/admin.csd --help

# Build frontend WASM (requires additional setup)
./zig-out/bin/cursed-zig --target=wasm32-freestanding examples/web-dashboard/frontend/app.csd
```

## API Endpoints

- `POST /api/auth/login` - User authentication
- `GET /api/users` - List users (admin)
- `POST /api/upload` - File upload
- `GET /api/metrics` - System metrics
- `WS /ws` - WebSocket connection for real-time updates

## This demonstrates CURSED's capabilities for:

- Network programming with `networkz` module
- File operations with `filez` module  
- JSON handling with `jsonz` module
- Concurrent programming with goroutines and channels
- Real-time communication with WebSockets
- Cross-compilation to WebAssembly
- Modular application architecture
