# CURSED RenderZ Module - Professional 2D/3D Graphics Rendering

## Overview

The RenderZ module provides comprehensive graphics rendering capabilities for CURSED applications, enabling professional-grade 2D and 3D graphics, shader programming, and modern graphics pipeline integration. This module supports multiple graphics APIs and offers hardware acceleration for high-performance applications.

## Features

### Supported Graphics APIs
- **OpenGL** - Cross-platform graphics API with wide compatibility
- **Vulkan** - Modern low-level API for maximum performance
- **DirectX 11** - Windows graphics API with broad hardware support
- **DirectX 12** - Modern Windows API for advanced features
- **Metal** - Apple's graphics API for macOS and iOS
- **Software** - CPU-based fallback renderer

### Core Functionality
- **Context Management** - Initialize and manage rendering contexts
- **Shader Programming** - Vertex, fragment, geometry, and compute shaders
- **Mesh Rendering** - 3D geometry with vertex and index buffers
- **Texture Management** - 2D textures with mipmaps and filtering
- **Camera System** - Perspective and orthographic projection
- **Lighting** - Point, directional, and spot lights
- **Render Targets** - Off-screen rendering and post-processing
- **2D Graphics** - Immediate mode 2D drawing functions

### Advanced Features
- **Material System** - PBR and custom material support
- **Scene Management** - Hierarchical scene graphs
- **Post-processing** - Screen-space effects and filters
- **Instanced Rendering** - Efficient rendering of multiple objects
- **Compute Shaders** - GPU compute for general-purpose computing

## Quick Start

```cursed
yeet "renderz"

# Initialize rendering context
sus context renderz.RenderContext = renderz_initialize(renderz.GRAPHICS_API_OPENGL, 1920, 1080)

# Create a simple scene
sus cube renderz.Mesh = renderz_create_cube_mesh()
sus camera renderz.Camera = renderz_create_perspective_camera(
    renderz_vec3(0.0, 0.0, 5.0),  # position
    renderz_vec3(0.0, 0.0, 0.0),  # target
    45.0, 16.0/9.0, 0.1, 100.0    # fov, aspect, near, far
)

# Render loop
bestie (true) {
    renderz_clear(context)
    
    sus model renderz.Mat4 = renderz_identity_matrix()
    renderz_set_camera_uniforms(cube.material.shader, camera)
    renderz_render_mesh(cube, model)
    
    renderz_present(context)
}
```

## API Reference

### Core Functions

#### Context Management
```cursed
slay renderz_initialize(api normie, width normie, height normie) RenderContext
slay renderz_shutdown(context RenderContext) lit
slay renderz_clear(context RenderContext) lit
slay renderz_present(context RenderContext) lit
slay renderz_set_viewport(context RenderContext, x normie, y normie, width normie, height normie) RenderContext
```

#### Shader Management
```cursed
slay renderz_create_shader(vertex_source tea, fragment_source tea) Shader
slay renderz_create_compute_shader(compute_source tea) Shader
slay renderz_use_shader(shader Shader) lit
slay renderz_set_uniform_float(shader Shader, name tea, value drip) lit
slay renderz_set_uniform_vec3(shader Shader, name tea, value Vec3) lit
slay renderz_set_uniform_mat4(shader Shader, name tea, value Mat4) lit
```

#### Texture Management
```cursed
slay renderz_create_texture(image_data imagez.ImageData) Texture
slay renderz_create_render_texture(width normie, height normie, format normie) Texture
slay renderz_bind_texture(texture Texture, slot normie) lit
slay renderz_generate_mipmaps(texture Texture) lit
```

#### Buffer Management
```cursed
slay renderz_create_vertex_buffer(vertices [1000]Vertex, vertex_count normie) Buffer
slay renderz_create_index_buffer(indices [3000]normie, index_count normie) Buffer
slay renderz_create_uniform_buffer(size normie) Buffer
slay renderz_update_buffer_data(buffer Buffer, data tea, offset normie) lit
```

#### Mesh and Geometry
```cursed
slay renderz_create_quad_mesh() Mesh
slay renderz_create_cube_mesh() Mesh
slay renderz_create_sphere_mesh(radius drip, segments normie, rings normie) Mesh
slay renderz_render_mesh(mesh Mesh, model_matrix Mat4) lit
```

#### Camera System
```cursed
slay renderz_create_perspective_camera(position Vec3, target Vec3, fov drip, aspect drip, near drip, far drip) Camera
slay renderz_create_orthographic_camera(position Vec3, target Vec3, left drip, right drip, bottom drip, top drip, near drip, far drip) Camera
slay renderz_update_camera(camera Camera) Camera
slay renderz_set_camera_uniforms(shader Shader, camera Camera) lit
```

#### Lighting System
```cursed
slay renderz_create_directional_light(direction Vec3, color Vec4, intensity drip) Light
slay renderz_create_point_light(position Vec3, color Vec4, intensity drip, range drip) Light
slay renderz_create_spot_light(position Vec3, direction Vec3, color Vec4, intensity drip, range drip, angle drip) Light
slay renderz_set_light_uniforms(shader Shader, lights [32]Light, light_count normie) lit
```

#### Render Targets
```cursed
slay renderz_create_render_target(width normie, height normie, samples normie) RenderTarget
slay renderz_bind_render_target(target RenderTarget) lit
slay renderz_unbind_render_target() lit
```

#### 2D Graphics
```cursed
slay renderz_draw_rectangle(x drip, y drip, width drip, height drip, color Vec4) lit
slay renderz_draw_circle(center_x drip, center_y drip, radius drip, segments normie, color Vec4) lit
slay renderz_draw_line(start_x drip, start_y drip, end_x drip, end_y drip, thickness drip, color Vec4) lit
slay renderz_draw_text(text tea, x drip, y drip, font_size drip, color Vec4) lit
```

### Data Structures

#### RenderContext
```cursed
be_like RenderContext = struct {
    api normie,
    width normie,
    height normie,
    clear_color Vec4,
    viewport_x normie,
    viewport_y normie,
    viewport_width normie,
    viewport_height normie,
    depth_test_enabled lit,
    blend_enabled lit,
    cull_mode normie,
    wireframe_mode lit
}
```

#### Vertex
```cursed
be_like Vertex = struct {
    position Vec3,
    normal Vec3,
    uv Vec2,
    color Vec4
}
```

#### Mesh
```cursed
be_like Mesh = struct {
    vertices [1000]Vertex,
    vertex_count normie,
    indices [3000]normie,
    index_count normie,
    vertex_buffer Buffer,
    index_buffer Buffer,
    material Material
}
```

#### Camera
```cursed
be_like Camera = struct {
    position Vec3,
    target Vec3,
    up Vec3,
    fov drip,
    aspect_ratio drip,
    near_plane drip,
    far_plane drip,
    view_matrix Mat4,
    projection_matrix Mat4,
    view_projection_matrix Mat4
}
```

#### Light
```cursed
be_like Light = struct {
    light_type normie,
    position Vec3,
    direction Vec3,
    color Vec4,
    intensity drip,
    range drip,
    spot_angle drip,
    attenuation Vec3
}
```

### Constants

#### Graphics APIs
```cursed
facts GRAPHICS_API_OPENGL normie = 1
facts GRAPHICS_API_VULKAN normie = 2
facts GRAPHICS_API_DIRECTX11 normie = 3
facts GRAPHICS_API_DIRECTX12 normie = 4
facts GRAPHICS_API_METAL normie = 5
facts GRAPHICS_API_SOFTWARE normie = 6
```

#### Primitive Types
```cursed
facts PRIMITIVE_POINTS normie = 1
facts PRIMITIVE_LINES normie = 2
facts PRIMITIVE_TRIANGLES normie = 4
facts PRIMITIVE_TRIANGLE_STRIP normie = 5
```

#### Texture Formats
```cursed
facts TEXTURE_RGB8 normie = 1
facts TEXTURE_RGBA8 normie = 2
facts TEXTURE_RGB32F normie = 5
facts TEXTURE_DEPTH24 normie = 8
```

#### Blend Modes
```cursed
facts BLEND_NONE normie = 0
facts BLEND_ALPHA normie = 1
facts BLEND_ADD normie = 2
facts BLEND_MULTIPLY normie = 3
```

## Usage Examples

### Basic 3D Rendering
```cursed
yeet "renderz"
yeet "imagez"

# Initialize rendering
sus context renderz.RenderContext = renderz_initialize(renderz.GRAPHICS_API_OPENGL, 1280, 720)

# Create shader
sus vertex_shader tea = "
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aNormal;
    layout (location = 2) in vec2 aTexCoord;
    
    uniform mat4 u_model;
    uniform mat4 u_view;
    uniform mat4 u_projection;
    
    out vec3 FragPos;
    out vec3 Normal;
    out vec2 TexCoord;
    
    void main() {
        FragPos = vec3(u_model * vec4(aPos, 1.0));
        Normal = mat3(transpose(inverse(u_model))) * aNormal;
        TexCoord = aTexCoord;
        gl_Position = u_projection * u_view * vec4(FragPos, 1.0);
    }"

sus fragment_shader tea = "
    #version 330 core
    out vec4 FragColor;
    
    in vec3 FragPos;
    in vec3 Normal;
    in vec2 TexCoord;
    
    uniform sampler2D u_texture;
    uniform vec3 u_light_position;
    uniform vec4 u_light_color;
    
    void main() {
        vec3 norm = normalize(Normal);
        vec3 lightDir = normalize(u_light_position - FragPos);
        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * u_light_color.rgb;
        
        vec4 texColor = texture(u_texture, TexCoord);
        FragColor = vec4(diffuse * texColor.rgb, texColor.a);
    }"

sus shader renderz.Shader = renderz_create_shader(vertex_shader, fragment_shader)

# Create 3D objects
sus cube renderz.Mesh = renderz_create_cube_mesh()
sus sphere renderz.Mesh = renderz_create_sphere_mesh(1.0, 32, 16)

# Setup camera
sus camera renderz.Camera = renderz_create_perspective_camera(
    renderz_vec3(0.0, 0.0, 5.0),
    renderz_vec3(0.0, 0.0, 0.0),
    45.0, 16.0/9.0, 0.1, 100.0
)

# Create lighting
sus light renderz.Light = renderz_create_point_light(
    renderz_vec3(2.0, 2.0, 2.0),
    renderz_vec4(1.0, 1.0, 1.0, 1.0),
    1.0, 10.0
)

# Render loop
sus frame normie = 0
bestie (frame < 60) {
    renderz_clear(context)
    
    # Update camera position
    sus angle drip = mathz_int_to_float(frame) * 0.02
    camera.position = renderz_vec3(mathz_cos(angle) * 5.0, 2.0, mathz_sin(angle) * 5.0)
    camera = renderz_update_camera(camera)
    
    # Set uniforms
    renderz_use_shader(shader)
    renderz_set_camera_uniforms(shader, camera)
    renderz_set_uniform_vec3(shader, "u_light_position", light.position)
    renderz_set_uniform_vec4(shader, "u_light_color", light.color)
    
    # Render objects
    sus cube_model renderz.Mat4 = renderz_create_transform_matrix(
        renderz_vec3(-2.0, 0.0, 0.0),
        renderz_vec3(angle, angle * 0.7, 0.0),
        renderz_vec3(1.0, 1.0, 1.0)
    )
    renderz_render_mesh(cube, cube_model)
    
    sus sphere_model renderz.Mat4 = renderz_create_transform_matrix(
        renderz_vec3(2.0, 0.0, 0.0),
        renderz_vec3(0.0, angle * 0.5, angle),
        renderz_vec3(1.0, 1.0, 1.0)
    )
    renderz_render_mesh(sphere, sphere_model)
    
    renderz_present(context)
    frame = frame + 1
}

renderz_shutdown(context)
```

### Texture Mapping
```cursed
# Load texture
sus texture_image imagez.ImageData = imagez_load_from_file("brick.jpg")
sus texture renderz.Texture = renderz_create_texture(texture_image)
renderz_generate_mipmaps(texture)

# Create textured mesh
sus textured_quad renderz.Mesh = renderz_create_quad_mesh()
textured_quad.material.textures[0] = texture
textured_quad.material.texture_count = 1

# Render with texture
renderz_clear(context)
renderz_bind_texture(texture, 0)
renderz_set_uniform_float(shader, "u_texture", 0.0)

sus model renderz.Mat4 = renderz_identity_matrix()
renderz_render_mesh(textured_quad, model)
renderz_present(context)
```

### 2D Graphics
```cursed
# Initialize for 2D rendering
sus context renderz.RenderContext = renderz_initialize(renderz.GRAPHICS_API_OPENGL, 800, 600)

# Setup orthographic camera
sus camera renderz.Camera = renderz_create_orthographic_camera(
    renderz_vec3(0.0, 0.0, 1.0),
    renderz_vec3(0.0, 0.0, 0.0),
    0.0, 800.0, 0.0, 600.0, -1.0, 1.0
)

# Draw 2D shapes
renderz_clear(context)

# Draw background
renderz_draw_rectangle(0.0, 0.0, 800.0, 600.0, renderz_vec4(0.1, 0.1, 0.2, 1.0))

# Draw UI elements
renderz_draw_rectangle(50.0, 50.0, 200.0, 100.0, renderz_vec4(0.8, 0.2, 0.2, 1.0))
renderz_draw_circle(400.0, 300.0, 75.0, 32, renderz_vec4(0.2, 0.8, 0.2, 0.8))
renderz_draw_line(100.0, 200.0, 700.0, 400.0, 5.0, renderz_vec4(1.0, 1.0, 0.0, 1.0))

# Draw text
renderz_draw_text("Score: 1337", 50.0, 500.0, 24.0, renderz_vec4(1.0, 1.0, 1.0, 1.0))

renderz_present(context)
```

### Render to Texture
```cursed
# Create render target
sus render_target renderz.RenderTarget = renderz_create_render_target(512, 512, 1)

# Render scene to off-screen target
renderz_bind_render_target(render_target)
renderz_clear(context)

# Render 3D scene
sus model renderz.Mat4 = renderz_create_transform_matrix(
    renderz_vec3(0.0, 0.0, 0.0),
    renderz_vec3(0.3, 0.5, 0.0),
    renderz_vec3(1.0, 1.0, 1.0)
)
renderz_render_mesh(cube, model)

# Return to main framebuffer
renderz_unbind_render_target()
renderz_clear(context)

# Use render target as texture
sus fullscreen_quad renderz.Mesh = renderz_create_quad_mesh()
fullscreen_quad.material.textures[0] = render_target.color_texture

renderz_bind_texture(render_target.color_texture, 0)
renderz_render_mesh(fullscreen_quad, renderz_identity_matrix())
renderz_present(context)
```

### Multi-API Support
```cursed
# Try different graphics APIs
sus apis [3]normie = [renderz.GRAPHICS_API_VULKAN, renderz.GRAPHICS_API_OPENGL, renderz.GRAPHICS_API_SOFTWARE]
sus api_names [3]tea = ["Vulkan", "OpenGL", "Software"]

sus api_index normie = 0
bestie (api_index < 3) {
    sus context renderz.RenderContext = renderz_initialize(apis[api_index], 800, 600)
    ready (context.api == apis[api_index]) {
        vibez.spill("Successfully initialized", api_names[api_index])
        
        # Perform basic rendering test
        renderz_clear(context)
        renderz_draw_rectangle(100.0, 100.0, 200.0, 150.0, renderz_vec4(1.0, 0.0, 0.0, 1.0))
        renderz_present(context)
        
        renderz_shutdown(context)
        break
    }
    api_index = api_index + 1
}
```

### Performance Profiling
```cursed
# Enable debug output
renderz_enable_debug_output()

# Measure rendering performance
sus start_time drip = time_now_seconds()
sus frame_count normie = 1000

sus frame normie = 0
bestie (frame < frame_count) {
    renderz_clear(context)
    
    # Render many objects
    sus object_count normie = 100
    sus i normie = 0
    bestie (i < object_count) {
        sus x drip = mathz_random() * 10.0 - 5.0
        sus y drip = mathz_random() * 10.0 - 5.0
        sus z drip = mathz_random() * 10.0 - 5.0
        
        sus model renderz.Mat4 = renderz_create_transform_matrix(
            renderz_vec3(x, y, z),
            renderz_vec3(0.0, 0.0, 0.0),
            renderz_vec3(0.1, 0.1, 0.1)
        )
        renderz_render_mesh(cube, model)
        i = i + 1
    }
    
    renderz_present(context)
    frame = frame + 1
}

sus end_time drip = time_now_seconds()
sus total_time drip = end_time - start_time
sus fps drip = mathz_int_to_float(frame_count) / total_time

vibez.spill("Average FPS:", fps)
vibez.spill("Total draw calls:", frame_count * 100)

# Get detailed statistics
sus gpu_info tea = renderz_get_gpu_memory_info()
sus render_stats tea = renderz_get_render_stats()
vibez.spill("GPU Info:", gpu_info)
vibez.spill("Render Stats:", render_stats)
```

## Performance Considerations

### Memory Management
- Reuse meshes and textures when possible
- Use appropriate texture formats and sizes
- Implement level-of-detail (LOD) systems for complex scenes

### Rendering Optimization
- Minimize state changes and draw calls
- Use instanced rendering for repeated objects
- Implement frustum culling for large scenes
- Consider GPU occlusion culling for complex environments

### Graphics API Selection
- **Vulkan/DirectX 12** - Best performance for complex applications
- **OpenGL/DirectX 11** - Good balance of performance and compatibility
- **Software** - Fallback for systems without hardware acceleration

## Dependencies

The RenderZ module depends on:
- `vibez` - For output and logging
- `mathz` - For mathematical operations and matrix calculations
- `stringz` - For string manipulation
- `memoryz` - For memory management
- `imagez` - For texture loading and image processing

## Error Handling

The module follows CURSED error handling conventions:
- Graphics API initialization failures are handled gracefully
- Shader compilation errors are reported with detailed messages
- Resource creation failures return invalid handles
- Use validation functions to check operation success

## Platform Support

RenderZ is designed to work across all CURSED-supported platforms:
- **Linux** - OpenGL, Vulkan support
- **macOS** - OpenGL, Metal support
- **Windows** - OpenGL, DirectX 11/12, Vulkan support
- **WebAssembly** - WebGL support with software fallback

## Thread Safety

RenderZ provides thread-safe operations for:
- Resource creation and management
- Command buffer recording (Vulkan/DirectX 12)
- Multi-threaded rendering scenarios

Note: OpenGL contexts are typically single-threaded per context.

## License

This module is part of the CURSED standard library and follows the same licensing terms as the core language.
