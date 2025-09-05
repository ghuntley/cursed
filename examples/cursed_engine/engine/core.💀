fr fr CursedEngine - Core Engine Module
fr fr High-performance 2D game engine written entirely in CURSED

yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "arrayz"
yeet "memoryz"
yeet "timez"
yeet "renderz"
yeet "audioz"
yeet "imagez"

fr fr ===== CORE ENGINE CONSTANTS =====

facts ENGINE_VERSION tea = "1.0.0"
facts ENGINE_NAME tea = "CursedEngine"
facts MAX_ENTITIES normie = 10000
facts MAX_COMPONENTS normie = 64
facts MAX_SYSTEMS normie = 32
facts TARGET_FPS normie = 60
facts FRAME_TIME_MS drip = 16.67 fr fr 1000/60

fr fr ===== ENGINE STATE =====

facts ENGINE_STATE_UNINITIALIZED normie = 0
facts ENGINE_STATE_INITIALIZING normie = 1
facts ENGINE_STATE_RUNNING normie = 2
facts ENGINE_STATE_PAUSED normie = 3
facts ENGINE_STATE_STOPPING normie = 4
facts ENGINE_STATE_SHUTDOWN normie = 5

fr fr ===== CORE DATA STRUCTURES =====

be_like Vector2 = struct {
    x drip,
    y drip
}

be_like Vector3 = struct {
    x drip,
    y drip,
    z drip
}

be_like Color = struct {
    r drip,
    g drip,
    b drip,
    a drip
}

be_like Rectangle = struct {
    x drip,
    y drip,
    width drip,
    height drip
}

be_like Transform = struct {
    position Vector2,
    rotation drip,
    scale Vector2
}

be_like GameTime = struct {
    total_time drip,
    delta_time drip,
    frame_count normie,
    fps drip,
    target_fps normie,
    time_scale drip
}

be_like EngineConfig = struct {
    window_title tea,
    window_width normie,
    window_height normie,
    fullscreen lit,
    vsync lit,
    target_fps normie,
    max_entities normie,
    audio_enabled lit,
    debug_mode lit,
    profiling_enabled lit
}

be_like Engine = struct {
    config EngineConfig,
    state normie,
    time GameTime,
    render_context renderz.RenderContext,
    is_running lit,
    should_exit lit,
    scene_manager SceneManager,
    entity_manager EntityManager,
    system_manager SystemManager,
    resource_manager ResourceManager,
    input_manager InputManager,
    audio_manager AudioManager,
    debug_info DebugInfo
}

be_like DebugInfo = struct {
    entity_count normie,
    active_systems normie,
    draw_calls normie,
    memory_usage_mb drip,
    frame_time_ms drip,
    cpu_usage drip,
    gpu_usage drip
}

fr fr ===== CORE ENGINE FUNCTIONS =====

slay engine_create_default_config() EngineConfig {
    sus config EngineConfig
    config.window_title = "CursedEngine Game"
    config.window_width = 1280
    config.window_height = 720
    config.fullscreen = false
    config.vsync = true
    config.target_fps = 60
    config.max_entities = 5000
    config.audio_enabled = true
    config.debug_mode = false
    config.profiling_enabled = false
    damn config
}

slay engine_initialize(config EngineConfig) Engine {
    sus engine Engine
    engine.config = config
    engine.state = ENGINE_STATE_INITIALIZING
    engine.is_running = false
    engine.should_exit = false
    
    vibez.spill("🎮 Initializing", ENGINE_NAME, "v" + ENGINE_VERSION)
    
    fr fr Initialize core systems
    engine.time = engine_create_game_time(config.target_fps)
    engine.render_context = renderz.renderz_initialize(renderz.GRAPHICS_API_OPENGL, config.window_width, config.window_height)
    engine.entity_manager = entity_manager_create(config.max_entities)
    engine.system_manager = system_manager_create()
    engine.scene_manager = scene_manager_create()
    engine.resource_manager = resource_manager_create()
    engine.input_manager = input_manager_create()
    
    ready (config.audio_enabled) {
        engine.audio_manager = audio_manager_create()
        vibez.spill("✅ Audio system initialized")
    }
    
    engine.debug_info = debug_info_create()
    
    vibez.spill("✅ Engine initialized successfully")
    engine.state = ENGINE_STATE_RUNNING
    damn engine
}

slay engine_shutdown(engine Engine) lit {
    vibez.spill("🛑 Shutting down", ENGINE_NAME)
    engine.state = ENGINE_STATE_STOPPING
    
    fr fr Cleanup systems in reverse order
    audio_manager_shutdown(engine.audio_manager)
    input_manager_shutdown(engine.input_manager)
    resource_manager_shutdown(engine.resource_manager)
    scene_manager_shutdown(engine.scene_manager)
    system_manager_shutdown(engine.system_manager)
    entity_manager_shutdown(engine.entity_manager)
    renderz.renderz_shutdown(engine.render_context)
    
    engine.state = ENGINE_STATE_SHUTDOWN
    vibez.spill("✅ Engine shutdown complete")
    damn true
}

slay engine_run(engine Engine) lit {
    engine.is_running = true
    sus last_frame_time drip = timez.timez_current_time_seconds()
    
    vibez.spill("🚀 Starting game loop...")
    
    bestie (engine.is_running && !engine.should_exit) {
        sus current_time drip = timez.timez_current_time_seconds()
        sus delta_time drip = current_time - last_frame_time
        last_frame_time = current_time
        
        fr fr Update game time
        engine.time = engine_update_game_time(engine.time, delta_time)
        
        fr fr Main game loop
        engine_update(engine, engine.time)
        engine_render(engine)
        
        fr fr Frame rate limiting
        engine_limit_frame_rate(engine.time)
        
        fr fr Update debug info
        ready (engine.config.debug_mode) {
            engine.debug_info = debug_info_update(engine.debug_info, engine)
        }
        
        fr fr Check for exit condition
        ready (input_manager_is_key_pressed(engine.input_manager, "ESCAPE")) {
            engine.should_exit = true
        }
    }
    
    damn true
}

slay engine_update(engine Engine, time GameTime) lit {
    fr fr Update input
    input_manager_update(engine.input_manager)
    
    fr fr Update audio
    ready (engine.config.audio_enabled) {
        audio_manager_update(engine.audio_manager, time)
    }
    
    fr fr Update current scene
    scene_manager_update(engine.scene_manager, time)
    
    fr fr Update all systems
    system_manager_update(engine.system_manager, engine.entity_manager, time)
    
    damn true
}

slay engine_render(engine Engine) lit {
    fr fr Begin frame
    renderz.renderz_clear(engine.render_context)
    
    fr fr Render current scene
    scene_manager_render(engine.scene_manager, engine.render_context)
    
    fr fr Render systems
    system_manager_render(engine.system_manager, engine.entity_manager, engine.render_context)
    
    fr fr Render debug info
    ready (engine.config.debug_mode) {
        debug_info_render(engine.debug_info, engine.render_context)
    }
    
    fr fr Present frame
    renderz.renderz_present(engine.render_context)
    damn true
}

fr fr ===== GAME TIME MANAGEMENT =====

slay engine_create_game_time(target_fps normie) GameTime {
    sus time GameTime
    time.total_time = 0.0
    time.delta_time = 0.0
    time.frame_count = 0
    time.fps = 0.0
    time.target_fps = target_fps
    time.time_scale = 1.0
    damn time
}

slay engine_update_game_time(time GameTime, delta_time drip) GameTime {
    sus updated GameTime
    updated = time
    updated.delta_time = delta_time * time.time_scale
    updated.total_time = updated.total_time + updated.delta_time
    updated.frame_count = updated.frame_count + 1
    
    fr fr Calculate FPS (smoothed over multiple frames)
    ready (delta_time > 0.0) {
        sus current_fps drip = 1.0 / delta_time
        updated.fps = (updated.fps * 0.9) + (current_fps * 0.1)
    }
    
    damn updated
}

slay engine_limit_frame_rate(time GameTime) lit {
    sus target_frame_time drip = 1.0 / mathz.mathz_int_to_float(time.target_fps)
    sus current_frame_time drip = timez.timez_current_time_seconds()
    sus sleep_time drip = target_frame_time - current_frame_time
    
    ready (sleep_time > 0.0) {
        timez.timez_sleep_milliseconds(mathz.mathz_float_to_int(sleep_time * 1000.0))
    }
    
    damn true
}

fr fr ===== VECTOR MATH UTILITIES =====

slay vector2_create(x drip, y drip) Vector2 {
    sus v Vector2
    v.x = x
    v.y = y
    damn v
}

slay vector2_add(a Vector2, b Vector2) Vector2 {
    damn vector2_create(a.x + b.x, a.y + b.y)
}

slay vector2_subtract(a Vector2, b Vector2) Vector2 {
    damn vector2_create(a.x - b.x, a.y - b.y)
}

slay vector2_multiply(v Vector2, scalar drip) Vector2 {
    damn vector2_create(v.x * scalar, v.y * scalar)
}

slay vector2_length(v Vector2) drip {
    damn mathz.mathz_sqrt(v.x * v.x + v.y * v.y)
}

slay vector2_normalize(v Vector2) Vector2 {
    sus length drip = vector2_length(v)
    ready (length == 0.0) {
        damn vector2_create(0.0, 0.0)
    }
    damn vector2_create(v.x / length, v.y / length)
}

slay vector2_distance(a Vector2, b Vector2) drip {
    damn vector2_length(vector2_subtract(b, a))
}

slay vector2_dot(a Vector2, b Vector2) drip {
    damn a.x * b.x + a.y * b.y
}

fr fr ===== COLOR UTILITIES =====

slay color_create(r drip, g drip, b drip, a drip) Color {
    sus c Color
    c.r = r
    c.g = g
    c.b = b
    c.a = a
    damn c
}

slay color_white() Color {
    damn color_create(1.0, 1.0, 1.0, 1.0)
}

slay color_black() Color {
    damn color_create(0.0, 0.0, 0.0, 1.0)
}

slay color_red() Color {
    damn color_create(1.0, 0.0, 0.0, 1.0)
}

slay color_green() Color {
    damn color_create(0.0, 1.0, 0.0, 1.0)
}

slay color_blue() Color {
    damn color_create(0.0, 0.0, 1.0, 1.0)
}

slay color_yellow() Color {
    damn color_create(1.0, 1.0, 0.0, 1.0)
}

slay color_magenta() Color {
    damn color_create(1.0, 0.0, 1.0, 1.0)
}

slay color_cyan() Color {
    damn color_create(0.0, 1.0, 1.0, 1.0)
}

fr fr ===== RECTANGLE UTILITIES =====

slay rectangle_create(x drip, y drip, width drip, height drip) Rectangle {
    sus r Rectangle
    r.x = x
    r.y = y
    r.width = width
    r.height = height
    damn r
}

slay rectangle_contains_point(rect Rectangle, point Vector2) lit {
    damn point.x >= rect.x && point.x <= (rect.x + rect.width) &&
         point.y >= rect.y && point.y <= (rect.y + rect.height)
}

slay rectangle_intersects(a Rectangle, b Rectangle) lit {
    damn a.x < (b.x + b.width) && (a.x + a.width) > b.x &&
         a.y < (b.y + b.height) && (a.y + a.height) > b.y
}

fr fr ===== TRANSFORM UTILITIES =====

slay transform_create(position Vector2, rotation drip, scale Vector2) Transform {
    sus t Transform
    t.position = position
    t.rotation = rotation
    t.scale = scale
    damn t
}

slay transform_identity() Transform {
    damn transform_create(vector2_create(0.0, 0.0), 0.0, vector2_create(1.0, 1.0))
}

fr fr ===== DEBUG INFO SYSTEM =====

slay debug_info_create() DebugInfo {
    sus debug DebugInfo
    debug.entity_count = 0
    debug.active_systems = 0
    debug.draw_calls = 0
    debug.memory_usage_mb = 0.0
    debug.frame_time_ms = 0.0
    debug.cpu_usage = 0.0
    debug.gpu_usage = 0.0
    damn debug
}

slay debug_info_update(debug DebugInfo, engine Engine) DebugInfo {
    sus updated DebugInfo
    updated = debug
    updated.entity_count = entity_manager_count(engine.entity_manager)
    updated.active_systems = system_manager_active_count(engine.system_manager)
    updated.frame_time_ms = engine.time.delta_time * 1000.0
    updated.memory_usage_mb = memoryz.memoryz_get_usage_mb()
    damn updated
}

slay debug_info_render(debug DebugInfo, context renderz.RenderContext) lit {
    sus y_offset drip = 10.0
    sus line_height drip = 20.0
    
    sus fps_text tea = "FPS: " + stringz.stringz_from_float(debug.frame_time_ms, 1)
    sus entity_text tea = "Entities: " + stringz.stringz_from_int(debug.entity_count)
    sus memory_text tea = "Memory: " + stringz.stringz_from_float(debug.memory_usage_mb, 1) + " MB"
    sus systems_text tea = "Systems: " + stringz.stringz_from_int(debug.active_systems)
    
    renderz.renderz_draw_text(fps_text, 10.0, y_offset, 14.0, color_green())
    renderz.renderz_draw_text(entity_text, 10.0, y_offset + line_height, 14.0, color_white())
    renderz.renderz_draw_text(memory_text, 10.0, y_offset + line_height * 2.0, 14.0, color_white())
    renderz.renderz_draw_text(systems_text, 10.0, y_offset + line_height * 3.0, 14.0, color_white())
    
    damn true
}

fr fr ===== ENGINE UTILITIES =====

slay engine_is_running(engine Engine) lit {
    damn engine.is_running && !engine.should_exit
}

slay engine_request_exit(engine Engine) Engine {
    sus updated Engine
    updated = engine
    updated.should_exit = true
    damn updated
}

slay engine_pause(engine Engine) Engine {
    sus updated Engine
    updated = engine
    updated.state = ENGINE_STATE_PAUSED
    damn updated
}

slay engine_resume(engine Engine) Engine {
    sus updated Engine
    updated = engine
    updated.state = ENGINE_STATE_RUNNING
    damn updated
}

slay engine_set_time_scale(engine Engine, scale drip) Engine {
    sus updated Engine
    updated = engine
    updated.time.time_scale = scale
    damn updated
}

fr fr ===== PLACEHOLDER FUNCTIONS =====
fr fr These would be implemented by the respective managers

slay entity_manager_create(max_entities normie) EntityManager {
    sus manager EntityManager
    vibez.spill("✅ Entity manager created")
    damn manager
}

slay system_manager_create() SystemManager {
    sus manager SystemManager
    vibez.spill("✅ System manager created")
    damn manager
}

slay scene_manager_create() SceneManager {
    sus manager SceneManager
    vibez.spill("✅ Scene manager created")
    damn manager
}

slay resource_manager_create() ResourceManager {
    sus manager ResourceManager
    vibez.spill("✅ Resource manager created")
    damn manager
}

slay input_manager_create() InputManager {
    sus manager InputManager
    vibez.spill("✅ Input manager created")
    damn manager
}

slay audio_manager_create() AudioManager {
    sus manager AudioManager
    vibez.spill("✅ Audio manager created")
    damn manager
}

fr fr Placeholder types for managers (to be defined in separate modules)
be_like EntityManager = struct { placeholder lit }
be_like SystemManager = struct { placeholder lit }
be_like SceneManager = struct { placeholder lit }
be_like ResourceManager = struct { placeholder lit }
be_like InputManager = struct { placeholder lit }
be_like AudioManager = struct { placeholder lit }

fr fr Placeholder functions
slay entity_manager_shutdown(manager EntityManager) lit { damn true }
slay system_manager_shutdown(manager SystemManager) lit { damn true }
slay scene_manager_shutdown(manager SceneManager) lit { damn true }
slay resource_manager_shutdown(manager ResourceManager) lit { damn true }
slay input_manager_shutdown(manager InputManager) lit { damn true }
slay audio_manager_shutdown(manager AudioManager) lit { damn true }
slay input_manager_update(manager InputManager) lit { damn true }
slay audio_manager_update(manager AudioManager, time GameTime) lit { damn true }
slay scene_manager_update(manager SceneManager, time GameTime) lit { damn true }
slay system_manager_update(manager SystemManager, entity_manager EntityManager, time GameTime) lit { damn true }
slay scene_manager_render(manager SceneManager, context renderz.RenderContext) lit { damn true }
slay system_manager_render(manager SystemManager, entity_manager EntityManager, context renderz.RenderContext) lit { damn true }
slay input_manager_is_key_pressed(manager InputManager, key tea) lit { damn false }
slay entity_manager_count(manager EntityManager) normie { damn 0 }
slay system_manager_active_count(manager SystemManager) normie { damn 0 }
