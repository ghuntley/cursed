// Core game engine with system management
import "../modules/audio/sound_manager.csd" as Audio
import "../modules/graphics/renderer.csd" as Renderer  
import "../modules/input/controller.csd" as Input
import "../systems/physics/physics_engine.csd" as Physics
import "../systems/ai/behavior_tree.csd" as AI
import "../utils/time_manager.csd" as Time
import "component_system.csd" as Components

export GameEngine, EngineState, SystemManager

enum EngineState {
    Initializing,
    Running,
    Paused,
    Shutting_Down,
    Error(string)
}

struct GameEngine {
    state: EngineState,
    systems: SystemManager,
    time_manager: Time.TimeManager,
    component_registry: Components.ComponentRegistry,
    is_running: bool,
    target_fps: int
}

struct SystemManager {
    audio: Option<Audio.SoundManager>,
    graphics: Option<Renderer.GraphicsRenderer>,
    input: Option<Input.InputController>,
    physics: Option<Physics.PhysicsEngine>,
    ai: Option<AI.BehaviorTreeSystem>
}

impl GameEngine {
    func new() -> GameEngine {
        println("Initializing GameEngine core")
        
        return GameEngine {
            state: EngineState::Initializing,
            systems: SystemManager::new(),
            time_manager: Time.TimeManager.new(),
            component_registry: Components.ComponentRegistry.new(),
            is_running: false,
            target_fps: 60
        }
    }
    
    func register_audio_system(&mut self, audio_system: Audio.SoundManager) {
        println("Registering audio system")
        self.systems.audio = Some(audio_system)
    }
    
    func register_graphics_system(&mut self, graphics_system: Renderer.GraphicsRenderer) {
        println("Registering graphics system") 
        self.systems.graphics = Some(graphics_system)
    }
    
    func register_input_system(&mut self, input_system: Input.InputController) {
        println("Registering input system")
        self.systems.input = Some(input_system)
    }
    
    func register_physics_system(&mut self, physics_system: Physics.PhysicsEngine) {
        println("Registering physics system")
        self.systems.physics = Some(physics_system)
    }
    
    func register_ai_system(&mut self, ai_system: AI.BehaviorTreeSystem) {
        println("Registering AI system")
        self.systems.ai = Some(ai_system)
    }
    
    func run_game_loop(&mut self) {
        println("Starting main game loop")
        self.state = EngineState::Running
        self.is_running = true
        
        let mut frame_count = 0
        let max_frames = 100  // Limit for testing
        
        while self.is_running && frame_count < max_frames {
            let frame_start = self.time_manager.get_current_time()
            
            // Update all systems
            self.update_systems()
            
            // Render frame
            self.render_frame()
            
            // Calculate frame time and enforce FPS limit
            let frame_time = self.time_manager.get_current_time() - frame_start
            self.time_manager.update_frame_time(frame_time)
            
            frame_count += 1
            
            if frame_count % 20 == 0 {
                println("Frame " + frame_count.to_string() + " - FPS: " + self.get_fps().to_string())
            }
        }
        
        println("Game loop finished after " + frame_count.to_string() + " frames")
        self.shutdown()
    }
    
    func update_systems(&mut self) {
        // Update input first
        if let Some(ref mut input) = self.systems.input {
            input.update()
        }
        
        // Update physics
        if let Some(ref mut physics) = self.systems.physics {
            physics.update(self.time_manager.get_delta_time())
        }
        
        // Update AI
        if let Some(ref mut ai) = self.systems.ai {
            ai.update(self.time_manager.get_delta_time())
        }
        
        // Update audio
        if let Some(ref mut audio) = self.systems.audio {
            audio.update()
        }
    }
    
    func render_frame(&mut self) {
        if let Some(ref mut renderer) = self.systems.graphics {
            renderer.begin_frame()
            renderer.render_scene()
            renderer.end_frame()
        }
    }
    
    func get_fps(&self) -> int {
        self.time_manager.get_fps()
    }
    
    func shutdown(&mut self) {
        println("Shutting down game engine")
        self.state = EngineState::Shutting_Down
        self.is_running = false
        
        // Shutdown systems in reverse order
        if let Some(ref mut ai) = self.systems.ai {
            ai.shutdown()
        }
        
        if let Some(ref mut physics) = self.systems.physics {
            physics.shutdown()
        }
        
        if let Some(ref mut input) = self.systems.input {
            input.shutdown()
        }
        
        if let Some(ref mut renderer) = self.systems.graphics {
            renderer.shutdown()
        }
        
        if let Some(ref mut audio) = self.systems.audio {
            audio.shutdown()
        }
        
        println("Game engine shutdown complete")
    }
}

impl SystemManager {
    func new() -> SystemManager {
        return SystemManager {
            audio: None,
            graphics: None,
            input: None,
            physics: None,
            ai: None
        }
    }
    
    func all_systems_initialized(&self) -> bool {
        return self.audio.is_some() && 
               self.graphics.is_some() && 
               self.input.is_some() && 
               self.physics.is_some() && 
               self.ai.is_some()
    }
}
