// Test nested module dependencies and circular dependency detection
import "core/engine.csd" as Engine
import "modules/audio/sound_manager.csd" as Audio
import "modules/graphics/renderer.csd" as Renderer
import "modules/input/controller.csd" as Input
import "systems/physics/physics_engine.csd" as Physics
import "systems/ai/behavior_tree.csd" as AI
import logging from stdlib

func main() {
    println("🎮 Starting Nested Modules Test - Game Engine")
    
    // Initialize logging
    let logger = logging.Logger.new("GameEngine")
    logger.info("Initializing game engine systems")
    
    // Initialize core engine
    let mut engine = Engine.GameEngine.new()
    logger.info("Core engine initialized")
    
    // Initialize subsystems
    let mut audio_manager = Audio.SoundManager.new()
    let mut renderer = Renderer.GraphicsRenderer.new()
    let mut input_controller = Input.InputController.new()
    let mut physics_engine = Physics.PhysicsEngine.new()
    let mut ai_system = AI.BehaviorTreeSystem.new()
    
    logger.info("All subsystems initialized")
    
    // Register systems with engine
    engine.register_audio_system(audio_manager)
    engine.register_graphics_system(renderer)
    engine.register_input_system(input_controller)
    engine.register_physics_system(physics_engine)
    engine.register_ai_system(ai_system)
    
    logger.info("Systems registered with engine")
    
    // Start main game loop
    engine.run_game_loop()
    
    logger.info("Game engine shutdown complete")
    println("✅ Nested modules test completed successfully!")
}
