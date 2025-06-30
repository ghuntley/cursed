// Graphics rendering system
import "../../core/component_system.csd" as Components
import "../../utils/time_manager.csd" as Time
export GraphicsRenderer, RenderPipeline, Shader, Mesh, Material, Camera

struct GraphicsRenderer {
    render_pipeline: RenderPipeline,
    active_camera: Option<Camera>,
    render_queue: Vec<RenderCommand>,
    debug_mode: bool,
    frame_count: int,
    render_stats: RenderStats
}

struct RenderPipeline {
    shaders: Map<int, Shader>,
    meshes: Map<int, Mesh>,
    materials: Map<int, Material>,
    render_targets: Vec<RenderTarget>,
    current_target: int
}

struct Shader {
    id: int,
    name: string,
    vertex_source: string,
    fragment_source: string,
    is_compiled: bool
}

struct Mesh {
    id: int,
    name: string,
    vertex_count: int,
    triangle_count: int,
    bounds: BoundingBox
}

struct Material {
    id: int,
    name: string,
    shader_id: int,
    textures: Map<string, int>,
    properties: Map<string, float>
}

struct Camera {
    position: Vec3,
    rotation: Vec3,
    fov: float,
    near_plane: float,
    far_plane: float,
    viewport_width: int,
    viewport_height: int
}

struct RenderCommand {
    mesh_id: int,
    material_id: int,
    transform: Mat4x4,
    priority: int
}

struct RenderTarget {
    id: int,
    width: int,
    height: int,
    format: TextureFormat
}

struct RenderStats {
    triangles_rendered: int,
    draw_calls: int,
    vertex_shader_invocations: int,
    fragment_shader_invocations: int,
    render_time_ms: float
}

struct BoundingBox {
    min: Vec3,
    max: Vec3
}

struct Mat4x4 {
    data: [float; 16]
}

struct Vec3 {
    x: float,
    y: float,
    z: float
}

enum TextureFormat {
    RGBA8,
    RGB8,
    R8,
    RGBA16F,
    Depth24Stencil8
}

impl GraphicsRenderer {
    func new() -> GraphicsRenderer {
        println("Initializing Graphics Renderer")
        
        let mut renderer = GraphicsRenderer {
            render_pipeline: RenderPipeline::new(),
            active_camera: None,
            render_queue: Vec.new(),
            debug_mode: false,
            frame_count: 0,
            render_stats: RenderStats::new()
        }
        
        renderer.initialize_default_assets()
        renderer.create_default_camera()
        renderer
    }
    
    func initialize_default_assets(&mut self) {
        // Create default shaders
        self.render_pipeline.create_shader(1, "basic_vertex", "basic_fragment")
        self.render_pipeline.create_shader(2, "pbr_vertex", "pbr_fragment")
        self.render_pipeline.create_shader(3, "skybox_vertex", "skybox_fragment")
        
        // Create default meshes
        self.render_pipeline.create_cube_mesh(1, "cube")
        self.render_pipeline.create_sphere_mesh(2, "sphere")
        self.render_pipeline.create_plane_mesh(3, "plane")
        
        // Create default materials
        self.render_pipeline.create_material(1, "default_material", 1)
        self.render_pipeline.create_material(2, "pbr_material", 2)
        
        println("Default graphics assets initialized")
    }
    
    func create_default_camera(&mut self) {
        self.active_camera = Some(Camera {
            position: Vec3::new(0.0, 0.0, 5.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            fov: 60.0,
            near_plane: 0.1,
            far_plane: 1000.0,
            viewport_width: 1920,
            viewport_height: 1080
        })
        
        println("Default camera created")
    }
    
    func begin_frame(&mut self) {
        self.frame_count += 1
        self.render_queue.clear()
        self.render_stats.reset()
        
        if self.frame_count % 60 == 0 {
            println("Graphics frame " + self.frame_count.to_string())
        }
    }
    
    func submit_render_command(&mut self, mesh_id: int, material_id: int, transform: Mat4x4) {
        let command = RenderCommand {
            mesh_id: mesh_id,
            material_id: material_id,
            transform: transform,
            priority: 0
        }
        
        self.render_queue.push(command)
    }
    
    func render_scene(&mut self) {
        if self.active_camera.is_none() {
            return
        }
        
        // Sort render queue by priority and material
        self.render_queue.sort_by(|a, b| {
            if a.priority != b.priority {
                return a.priority.cmp(&b.priority)
            }
            a.material_id.cmp(&b.material_id)
        })
        
        // Execute render commands
        for command in &self.render_queue {
            self.execute_render_command(command)
        }
        
        // Render debug information if enabled
        if self.debug_mode {
            self.render_debug_info()
        }
    }
    
    func execute_render_command(&mut self, command: &RenderCommand) {
        // Validate mesh and material exist
        if !self.render_pipeline.meshes.contains_key(&command.mesh_id) {
            return
        }
        
        if !self.render_pipeline.materials.contains_key(&command.material_id) {
            return
        }
        
        let mesh = &self.render_pipeline.meshes[&command.mesh_id]
        let material = &self.render_pipeline.materials[&command.material_id]
        
        // Update render stats
        self.render_stats.draw_calls += 1
        self.render_stats.triangles_rendered += mesh.triangle_count
        self.render_stats.vertex_shader_invocations += mesh.vertex_count
        self.render_stats.fragment_shader_invocations += mesh.triangle_count * 3 // Approximate
        
        // Simulate rendering
        if self.debug_mode && self.frame_count % 120 == 0 {
            println("Rendering mesh: " + mesh.name + " with material: " + material.name)
        }
    }
    
    func render_debug_info(&mut self) {
        if self.frame_count % 60 == 0 {
            println("=== Render Stats ===")
            println("Draw calls: " + self.render_stats.draw_calls.to_string())
            println("Triangles: " + self.render_stats.triangles_rendered.to_string())
            println("Vertex invocations: " + self.render_stats.vertex_shader_invocations.to_string())
        }
    }
    
    func end_frame(&mut self) {
        // Simulate frame completion time
        self.render_stats.render_time_ms = Time.mock_time() * 16.67 // ~60 FPS target
        
        // Present frame (mock)
        if self.debug_mode && self.frame_count % 180 == 0 {
            println("Frame " + self.frame_count.to_string() + " presented")
        }
    }
    
    func set_debug_mode(&mut self, enabled: bool) {
        self.debug_mode = enabled
        println("Graphics debug mode: " + enabled.to_string())
    }
    
    func get_render_stats(&self) -> &RenderStats {
        &self.render_stats
    }
    
    func shutdown(&mut self) {
        println("Shutting down Graphics Renderer")
        self.render_queue.clear()
        self.active_camera = None
    }
}

impl RenderPipeline {
    func new() -> RenderPipeline {
        RenderPipeline {
            shaders: Map.new(),
            meshes: Map.new(),
            materials: Map.new(),
            render_targets: Vec.new(),
            current_target: 0
        }
    }
    
    func create_shader(&mut self, id: int, vertex_name: string, fragment_name: string) {
        let shader = Shader {
            id: id,
            name: vertex_name.clone() + "_" + &fragment_name,
            vertex_source: "// Mock vertex shader for " + &vertex_name,
            fragment_source: "// Mock fragment shader for " + &fragment_name,
            is_compiled: true
        }
        
        self.shaders.insert(id, shader)
        println("Created shader: " + vertex_name + " + " + fragment_name)
    }
    
    func create_cube_mesh(&mut self, id: int, name: string) {
        let mesh = Mesh {
            id: id,
            name: name.clone(),
            vertex_count: 24,  // 6 faces * 4 vertices
            triangle_count: 12, // 6 faces * 2 triangles
            bounds: BoundingBox {
                min: Vec3::new(-1.0, -1.0, -1.0),
                max: Vec3::new(1.0, 1.0, 1.0)
            }
        }
        
        self.meshes.insert(id, mesh)
        println("Created cube mesh: " + name)
    }
    
    func create_sphere_mesh(&mut self, id: int, name: string) {
        let mesh = Mesh {
            id: id,
            name: name.clone(),
            vertex_count: 1024,  // More complex sphere
            triangle_count: 2048,
            bounds: BoundingBox {
                min: Vec3::new(-1.0, -1.0, -1.0),
                max: Vec3::new(1.0, 1.0, 1.0)
            }
        }
        
        self.meshes.insert(id, mesh)
        println("Created sphere mesh: " + name)
    }
    
    func create_plane_mesh(&mut self, id: int, name: string) {
        let mesh = Mesh {
            id: id,
            name: name.clone(),
            vertex_count: 4,
            triangle_count: 2,
            bounds: BoundingBox {
                min: Vec3::new(-1.0, 0.0, -1.0),
                max: Vec3::new(1.0, 0.0, 1.0)
            }
        }
        
        self.meshes.insert(id, mesh)
        println("Created plane mesh: " + name)
    }
    
    func create_material(&mut self, id: int, name: string, shader_id: int) {
        let mut properties = Map.new()
        properties.insert("roughness", 0.5)
        properties.insert("metallic", 0.0)
        properties.insert("emission", 0.0)
        
        let material = Material {
            id: id,
            name: name.clone(),
            shader_id: shader_id,
            textures: Map.new(),
            properties: properties
        }
        
        self.materials.insert(id, material)
        println("Created material: " + name + " with shader " + shader_id.to_string())
    }
}

impl RenderStats {
    func new() -> RenderStats {
        RenderStats {
            triangles_rendered: 0,
            draw_calls: 0,
            vertex_shader_invocations: 0,
            fragment_shader_invocations: 0,
            render_time_ms: 0.0
        }
    }
    
    func reset(&mut self) {
        self.triangles_rendered = 0
        self.draw_calls = 0
        self.vertex_shader_invocations = 0
        self.fragment_shader_invocations = 0
        self.render_time_ms = 0.0
    }
}

impl Vec3 {
    func new(x: float, y: float, z: float) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
}

impl Mat4x4 {
    func identity() -> Mat4x4 {
        let mut data = [0.0; 16]
        data[0] = 1.0   // [0,0]
        data[5] = 1.0   // [1,1] 
        data[10] = 1.0  // [2,2]
        data[15] = 1.0  // [3,3]
        
        Mat4x4 { data: data }
    }
}
