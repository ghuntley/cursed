# CursedEngine - 2D Game Engine & Demo Game

A comprehensive 2D game engine written entirely in CURSED to showcase the language's capabilities for real-time, performance-critical applications.

## 🎮 Features

### Core Engine Systems
- **Entity-Component-System (ECS)** architecture for high-performance game object management
- **Advanced Sprite Rendering** with batching and texture atlasing
- **Physics Simulation** with collision detection and resolution
- **Audio System** with positional audio and dynamic music
- **Input Handling** for keyboard, mouse, and gamepad
- **Scene Management** with seamless transitions
- **Particle Systems** for visual effects
- **UI Framework** for game interfaces

### Graphics Pipeline
- **2D Sprite Rendering** with hardware acceleration
- **Animation Framework** with state machines
- **Lighting System** with dynamic shadows
- **Post-Processing Effects** pipeline
- **Debug Visualization** tools

### Demo Game: "Cursed Quest"
- **2D Platformer/Adventure** gameplay
- **Character System** with RPG elements
- **Enemy AI** with behavior trees
- **Level Progression** with save/load system
- **Achievement System** 
- **Sound Effects & Music** with dynamic audio

### Development Tools
- **Level Editor** (web-based)
- **Asset Pipeline** for textures, audio, and data
- **Performance Profiler** with real-time metrics
- **Debug Console** with runtime commands

### Multi-Platform Deployment
- **Web Version** (WebAssembly + Canvas)
- **Native Desktop** (Windows, macOS, Linux)
- **Mobile-Friendly** web interface

## 🚀 Quick Start

```bash
# Build the engine and demo
zig build

# Run the demo game
./zig-out/bin/cursed-quest

# Run level editor (web)
./zig-out/bin/cursed-editor --web

# Run performance tests
./zig-out/bin/cursed-engine-benchmark
```

## 📁 Project Structure

```
cursed_engine/
├── engine/           # Core engine systems
│   ├── ecs/         # Entity-Component-System
│   ├── graphics/    # Rendering pipeline
│   ├── physics/     # Physics simulation
│   ├── audio/       # Audio management
│   ├── input/       # Input handling
│   └── scenes/      # Scene management
├── demo_game/       # Cursed Quest demo
│   ├── entities/    # Game entities
│   ├── systems/     # Game systems
│   ├── levels/      # Level data
│   └── assets/      # Game assets
├── tools/           # Development tools
│   ├── editor/      # Level editor
│   ├── profiler/    # Performance profiler
│   └── pipeline/    # Asset pipeline
└── examples/        # Usage examples
```

## 🎯 Performance Targets

- **60 FPS** at 1080p with 1000+ entities
- **<10ms** input latency
- **<100MB** memory usage
- **<1s** level loading times
- **Cross-platform** compatibility

## 🛠️ Technical Highlights

- **Memory-efficient** ECS with data-oriented design
- **Batched rendering** for optimal GPU utilization
- **Spatial partitioning** for efficient collision detection
- **Asset streaming** for large worlds
- **Hot-reloading** for rapid development
- **Comprehensive profiling** tools

## 📚 Documentation

- [Engine Architecture](docs/architecture.md)
- [API Reference](docs/api.md)
- [Game Development Guide](docs/game_dev.md)
- [Performance Guide](docs/performance.md)
- [Deployment Guide](docs/deployment.md)

---

**Built with ❤️ in CURSED**
*Showcasing real-time performance in a fun, expressive language*
