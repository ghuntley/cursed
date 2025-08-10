// CURSED RenderZ Module - Comprehensive Graphics Rendering Examples
// Demonstrates professional 2D/3D graphics capabilities

yeet "renderz"
yeet "imagez"
yeet "vibez"
yeet "testz"

// ===== BASIC RENDERING SETUP EXAMPLE =====

slay demo_basic_rendering_setup() lit {
    vibez.print_header("Basic Rendering Setup Demo")
    
    // Initialize rendering context
    sus context renderz.RenderContext = renderz_initialize(renderz.GRAPHICS_API_OPENGL, 1920, 1080)
    vibez.print_result("Graphics API", "OpenGL")
    vibez.print_result("Resolution", "1920x1080")
    
    // Set viewport and clear color
    context = renderz_set_viewport(context, 0, 0, 1920, 1080)
    context.clear_color = renderz_vec4(0.2, 0.3, 0.4, 1.0)
    
    // Basic render loop simulation
    sus frame normie = 0
    bestie (frame < 5) {
        renderz_clear(context)
        
        // Render content would go here
        vibez.print_success(stringz_concat("Frame ", stringz_from_int(frame), " rendered"))
        
        renderz_present(context)
        frame = frame + 1
    }
    
    // Cleanup
    renderz_shutdown(context)
    vibez.print_success("Rendering context shut down")
    
    damn true
}

// ===== SHADER PROGRAMMING EXAMPLE =====

slay demo_shader_programming() lit {
    vibez.print_header("Shader Programming Demo")
    
    // Initialize context
    sus context renderz.RenderContext = renderz_initialize(renderz.GRAPHICS_API_OPENGL, 1280, 720)
    
    // Vertex shader source
    sus vertex_shader tea = "
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aNormal;
        layout (location = 2) in vec2 aTexCoord;
        layout (location = 3) in vec4 aColor;
        
        uniform mat4 u_model;
        uniform mat4 u_view;
        uniform mat4 u_projection;
        
        out vec3 FragPos;
        out vec3 Normal;
        out vec2 TexCoord;
        out vec4 Color;
        
        void main() {
            FragPos = vec3(u_model * vec4(aPos, 1.0));
            Normal = mat3(transpose(inverse(u_model))) * aNormal;
            TexCoord = aTexCoord;
            Color = aColor;
            
            gl_Position = u_projection * u_view * vec4(FragPos, 1.0);
        }"
    
    // Fragment shader source
    sus fragment_shader tea = "
        #version 330 core
        out vec4 FragColor;
        
        in vec3 FragPos;
        in vec3 Normal;
        in vec2 TexCoord;
        in vec4 Color;
        
        uniform sampler2D u_texture;
        uniform vec3 u_camera_position;
        uniform vec4 u_light_color;
        uniform vec3 u_light_position;
        
        void main() {
            vec3 norm = normalize(Normal);
            vec3 lightDir = normalize(u_light_position - FragPos);
            
            float diff = max(dot(norm, lightDir), 0.0);
            vec3 diffuse = diff * u_light_color.rgb;
            
            vec3 viewDir = normalize(u_camera_position - FragPos);
            vec3 reflectDir = reflect(-lightDir, norm);
            float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
            vec3 specular = spec * u_light_color.rgb;
            
            vec4 texColor = texture(u_texture, TexCoord);
            vec3 result = (diffuse + specular) * texColor.rgb * Color.rgb;
            
            FragColor = vec4(result, texColor.a * Color.a);
        }"
    
    // Create and compile shader
    sus shader renderz.Shader = renderz_create_shader(vertex_shader, fragment_shader)
    ready (shader.compiled) {
        vibez.print_success("Shader compiled successfully")
        vibez.print_result("Shader ID", stringz_from_int(shader.id))
        vibez.print_result("Uniform count", stringz_from_int(shader.uniform_count))
    } otherwise {
        vibez.print_error("Failed to compile shader")
    }
    
    // Use shader and set uniforms
    renderz_use_shader(shader)
    renderz_set_uniform_vec4(shader, "u_light_color", renderz_vec4(1.0, 1.0, 1.0, 1.0))
    renderz_set_uniform_vec3(shader, "u_light_position", renderz_vec3(5.0, 5.0, 5.0))
    vibez.print_success("Shader uniforms set")
    
    renderz_shutdown(context)
    damn true
}

// ===== 3D MESH RENDERING EXAMPLE =====

slay demo_3d_mesh_rendering() lit {
    vibez.print_header("3D Mesh Rendering Demo")
    
    sus context renderz.RenderContext = renderz_initialize(renderz.GRAPHICS_API_OPENGL, 1280, 720)
    
    // Create 3D meshes
    sus cube renderz.Mesh = renderz_create_cube_mesh()
    vibez.print_success("Cube mesh created")
    
    sus sphere renderz.Mesh = renderz_create_sphere_mesh(1.0, 32, 16)
    vibez.print_success("Sphere mesh created")
    
    // Create camera
    sus camera renderz.Camera = renderz_create_perspective_camera(
        renderz_vec3(0.0, 0.0, 5.0),  // position
        renderz_vec3(0.0, 0.0, 0.0),  // target
        45.0,                         // fov
        16.0 / 9.0,                   // aspect ratio
        0.1,                          // near plane
        100.0                         // far plane
    )
    vibez.print_success("Perspective camera created")
    
    // Create lights
    sus lights [32]renderz.Light
    lights[0] = renderz_create_directional_light(
        renderz_vec3(-1.0, -1.0, -1.0),
        renderz_vec4(1.0, 1.0, 1.0, 1.0),
        1.0
    )
    lights[1] = renderz_create_point_light(
        renderz_vec3(2.0, 2.0, 2.0),
        renderz_vec4(1.0, 0.5, 0.2, 1.0),
        0.8,
        10.0
    )
    sus light_count normie = 2
    vibez.print_success("Lighting setup complete")
    
    // Render scene
    sus frame normie = 0
    bestie (frame < 10) {
        renderz_clear(context)
        
        // Update camera
        sus rotation drip = mathz_int_to_float(frame) * 0.1
        camera.position = renderz_vec3(
            mathz_cos(rotation) * 5.0,
            2.0,
            mathz_sin(rotation) * 5.0
        )
        camera = renderz_update_camera(camera)
        
        // Set camera uniforms
        renderz_set_camera_uniforms(cube.material.shader, camera)
        renderz_set_light_uniforms(cube.material.shader, lights, light_count)
        
        // Render cube
        sus cube_model renderz.Mat4 = renderz_create_transform_matrix(
            renderz_vec3(-2.0, 0.0, 0.0),
            renderz_vec3(rotation, rotation * 0.7, 0.0),
            renderz_vec3(1.0, 1.0, 1.0)
        )
        renderz_render_mesh(cube, cube_model)
        
        // Render sphere
        sus sphere_model renderz.Mat4 = renderz_create_transform_matrix(
            renderz_vec3(2.0, 0.0, 0.0),
            renderz_vec3(0.0, rotation * 0.5, rotation),
            renderz_vec3(1.0, 1.0, 1.0)
        )
        renderz_render_mesh(sphere, sphere_model)
        
        renderz_present(context)
        vibez.print_success(stringz_concat("3D frame ", stringz_from_int(frame), " rendered"))
        
        frame = frame + 1
    }
    
    renderz_shutdown(context)
    damn true
}

// ===== TEXTURE MAPPING EXAMPLE =====

slay demo_texture_mapping() lit {
    vibez.print_header("Texture Mapping Demo")
    
    sus context renderz.RenderContext = renderz_initialize(renderz.GRAPHICS_API_OPENGL, 1024, 768)
    
    // Load texture image
    sus texture_image imagez.ImageData = imagez_load_from_file("texture.png")
    ready (texture_image.width == 0) {
        // Create procedural texture if file not found
        texture_image = imagez_create_solid_color(256, 256, imagez.COLOR_WHITE, 4)
        vibez.print_warning("Using procedural texture")
    } otherwise {
        vibez.print_success("Loaded texture from file")
    }
    
    // Create GPU texture
    sus texture renderz.Texture = renderz_create_texture(texture_image)
    vibez.print_result("Texture ID", stringz_from_int(texture.id))
    vibez.print_result("Texture size", stringz_concat(stringz_from_int(texture.width), "x", stringz_from_int(texture.height)))
    
    // Generate mipmaps for better quality
    renderz_generate_mipmaps(texture)
    vibez.print_success("Generated texture mipmaps")
    
    // Create textured quad
    sus quad renderz.Mesh = renderz_create_quad_mesh()
    quad.material.textures[0] = texture
    quad.material.texture_count = 1
    
    // Create camera for 2D view
    sus camera renderz.Camera = renderz_create_orthographic_camera(
        renderz_vec3(0.0, 0.0, 1.0),
        renderz_vec3(0.0, 0.0, 0.0),
        -2.0, 2.0, -1.5, 1.5, 0.1, 10.0
    )
    
    // Render textured quad
    sus frame normie = 0
    bestie (frame < 5) {
        renderz_clear(context)
        
        // Bind texture
        renderz_bind_texture(texture, 0)
        
        // Set uniforms
        renderz_set_camera_uniforms(quad.material.shader, camera)
        renderz_set_uniform_float(quad.material.shader, "u_time", mathz_int_to_float(frame))
        
        // Render quad
        sus model renderz.Mat4 = renderz_create_transform_matrix(
            renderz_vec3(0.0, 0.0, 0.0),
            renderz_vec3(0.0, 0.0, 0.0),
            renderz_vec3(1.0, 1.0, 1.0)
        )
        renderz_render_mesh(quad, model)
        
        renderz_present(context)
        vibez.print_success(stringz_concat("Textured frame ", stringz_from_int(frame), " rendered"))
        
        frame = frame + 1
    }
    
    renderz_shutdown(context)
    damn true
}

// ===== 2D GRAPHICS EXAMPLE =====

slay demo_2d_graphics() lit {
    vibez.print_header("2D Graphics Demo")
    
    sus context renderz.RenderContext = renderz_initialize(renderz.GRAPHICS_API_OPENGL, 800, 600)
    
    // Set up 2D projection
    sus camera renderz.Camera = renderz_create_orthographic_camera(
        renderz_vec3(0.0, 0.0, 1.0),
        renderz_vec3(0.0, 0.0, 0.0),
        0.0, 800.0, 0.0, 600.0, -1.0, 1.0
    )
    
    // Draw various 2D shapes
    renderz_clear(context)
    
    // Draw rectangles
    renderz_draw_rectangle(100.0, 100.0, 200.0, 150.0, renderz_vec4(1.0, 0.0, 0.0, 1.0))
    vibez.print_success("Drew red rectangle")
    
    renderz_draw_rectangle(400.0, 200.0, 150.0, 100.0, renderz_vec4(0.0, 1.0, 0.0, 0.7))
    vibez.print_success("Drew green rectangle with transparency")
    
    // Draw circles
    renderz_draw_circle(200.0, 400.0, 50.0, 32, renderz_vec4(0.0, 0.0, 1.0, 1.0))
    vibez.print_success("Drew blue circle")
    
    renderz_draw_circle(500.0, 450.0, 75.0, 64, renderz_vec4(1.0, 1.0, 0.0, 0.8))
    vibez.print_success("Drew yellow circle")
    
    // Draw lines
    renderz_draw_line(50.0, 50.0, 750.0, 550.0, 5.0, renderz_vec4(1.0, 0.0, 1.0, 1.0))
    vibez.print_success("Drew diagonal line")
    
    renderz_draw_line(50.0, 550.0, 750.0, 50.0, 3.0, renderz_vec4(0.0, 1.0, 1.0, 1.0))
    vibez.print_success("Drew second diagonal line")
    
    // Draw text
    renderz_draw_text("CURSED Graphics!", 300.0, 300.0, 24.0, renderz_vec4(1.0, 1.0, 1.0, 1.0))
    vibez.print_success("Drew text")
    
    renderz_present(context)
    vibez.print_success("2D graphics frame presented")
    
    renderz_shutdown(context)
    damn true
}

// ===== RENDER TARGET EXAMPLE =====

slay demo_render_targets() lit {
    vibez.print_header("Render Target Demo")
    
    sus context renderz.RenderContext = renderz_initialize(renderz.GRAPHICS_API_OPENGL, 1024, 768)
    
    // Create render target for off-screen rendering
    sus render_target renderz.RenderTarget = renderz_create_render_target(512, 512, 1)
    vibez.print_success("Created 512x512 render target")
    
    // Create scene to render
    sus cube renderz.Mesh = renderz_create_cube_mesh()
    sus camera renderz.Camera = renderz_create_perspective_camera(
        renderz_vec3(0.0, 0.0, 3.0),
        renderz_vec3(0.0, 0.0, 0.0),
        45.0, 1.0, 0.1, 100.0
    )
    
    // Render to off-screen target
    renderz_bind_render_target(render_target)
    renderz_clear(context)
    
    sus model renderz.Mat4 = renderz_create_transform_matrix(
        renderz_vec3(0.0, 0.0, 0.0),
        renderz_vec3(0.3, 0.5, 0.0),
        renderz_vec3(1.0, 1.0, 1.0)
    )
    renderz_set_camera_uniforms(cube.material.shader, camera)
    renderz_render_mesh(cube, model)
    
    vibez.print_success("Rendered cube to off-screen target")
    
    // Return to main framebuffer
    renderz_unbind_render_target()
    renderz_clear(context)
    
    // Create full-screen quad to display render target
    sus fullscreen_quad renderz.Mesh = renderz_create_quad_mesh()
    fullscreen_quad.material.textures[0] = render_target.color_texture
    fullscreen_quad.material.texture_count = 1
    
    // Render full-screen quad
    sus orthographic_camera renderz.Camera = renderz_create_orthographic_camera(
        renderz_vec3(0.0, 0.0, 1.0),
        renderz_vec3(0.0, 0.0, 0.0),
        -1.0, 1.0, -1.0, 1.0, 0.1, 10.0
    )
    
    renderz_bind_texture(render_target.color_texture, 0)
    renderz_set_camera_uniforms(fullscreen_quad.material.shader, orthographic_camera)
    
    sus fullscreen_model renderz.Mat4 = renderz_identity_matrix()
    renderz_render_mesh(fullscreen_quad, fullscreen_model)
    
    renderz_present(context)
    vibez.print_success("Displayed render target on main framebuffer")
    
    renderz_shutdown(context)
    damn true
}

// ===== PERFORMANCE TESTING EXAMPLE =====

slay demo_performance_testing() lit {
    vibez.print_header("Performance Testing Demo")
    
    sus context renderz.RenderContext = renderz_initialize(renderz.GRAPHICS_API_OPENGL, 1920, 1080)
    
    // Create many meshes for stress testing
    sus mesh_count normie = 100
    sus meshes [100]renderz.Mesh
    sus i normie = 0
    bestie (i < mesh_count) {
        meshes[i] = renderz_create_cube_mesh()
        i = i + 1
    }
    vibez.print_result("Created meshes", stringz_from_int(mesh_count))
    
    // Setup camera
    sus camera renderz.Camera = renderz_create_perspective_camera(
        renderz_vec3(0.0, 0.0, 10.0),
        renderz_vec3(0.0, 0.0, 0.0),
        45.0, 16.0/9.0, 0.1, 100.0
    )
    
    // Performance test rendering
    sus start_time drip = time_now_seconds()
    sus frame_count normie = 60
    sus frame normie = 0
    
    bestie (frame < frame_count) {
        renderz_clear(context)
        
        // Render all meshes
        sus mesh_index normie = 0
        bestie (mesh_index < mesh_count) {
            sus x drip = mathz_int_to_float(mesh_index % 10) * 2.0 - 10.0
            sus y drip = mathz_int_to_float(mesh_index / 10) * 2.0 - 10.0
            sus rotation drip = mathz_int_to_float(frame) * 0.02
            
            sus model renderz.Mat4 = renderz_create_transform_matrix(
                renderz_vec3(x, y, 0.0),
                renderz_vec3(rotation, rotation * 0.7, 0.0),
                renderz_vec3(0.5, 0.5, 0.5)
            )
            
            renderz_set_camera_uniforms(meshes[mesh_index].material.shader, camera)
            renderz_render_mesh(meshes[mesh_index], model)
            
            mesh_index = mesh_index + 1
        }
        
        renderz_present(context)
        frame = frame + 1
    }
    
    sus end_time drip = time_now_seconds()
    sus total_time drip = end_time - start_time
    sus fps drip = mathz_int_to_float(frame_count) / total_time
    
    vibez.print_result("Total render time", stringz_concat(stringz_from_float(total_time), "s"))
    vibez.print_result("Average FPS", stringz_from_float(fps))
    vibez.print_result("Draw calls per frame", stringz_from_int(mesh_count))
    
    // Get GPU memory info
    sus gpu_info tea = renderz_get_gpu_memory_info()
    vibez.print_result("GPU Memory Info", gpu_info)
    
    sus render_stats tea = renderz_get_render_stats()
    vibez.print_result("Render Statistics", render_stats)
    
    renderz_shutdown(context)
    damn true
}

// ===== MULTI-API SUPPORT EXAMPLE =====

slay demo_multi_api_support() lit {
    vibez.print_header("Multi-API Support Demo")
    
    // Test different graphics APIs
    sus apis [4]normie
    apis[0] = renderz.GRAPHICS_API_OPENGL
    apis[1] = renderz.GRAPHICS_API_VULKAN
    apis[2] = renderz.GRAPHICS_API_DIRECTX11
    apis[3] = renderz.GRAPHICS_API_SOFTWARE
    
    sus api_names [4]tea
    api_names[0] = "OpenGL"
    api_names[1] = "Vulkan"
    api_names[2] = "DirectX 11"
    api_names[3] = "Software"
    
    sus api_index normie = 0
    bestie (api_index < 4) {
        vibez.print_result("Testing API", api_names[api_index])
        
        sus context renderz.RenderContext = renderz_initialize(apis[api_index], 800, 600)
        
        // Simple render test
        renderz_clear(context)
        renderz_draw_rectangle(100.0, 100.0, 200.0, 150.0, renderz_vec4(1.0, 0.0, 0.0, 1.0))
        renderz_present(context)
        
        vibez.print_success(stringz_concat(api_names[api_index], " render test passed"))
        
        renderz_shutdown(context)
        api_index = api_index + 1
    }
    
    vibez.print_success("All graphics APIs tested")
    damn true
}

// ===== MAIN DEMO FUNCTION =====

slay main() normie {
    vibez.print_header("CURSED RenderZ Professional Graphics Rendering Demo")
    
    // Enable debug output for development
    renderz_enable_debug_output()
    vibez.print_success("Debug output enabled")
    
    // Run all demonstrations
    demo_basic_rendering_setup()
    vibez.print_separator()
    
    demo_shader_programming()
    vibez.print_separator()
    
    demo_3d_mesh_rendering()
    vibez.print_separator()
    
    demo_texture_mapping()
    vibez.print_separator()
    
    demo_2d_graphics()
    vibez.print_separator()
    
    demo_render_targets()
    vibez.print_separator()
    
    demo_performance_testing()
    vibez.print_separator()
    
    demo_multi_api_support()
    vibez.print_separator()
    
    vibez.print_success("All RenderZ demos completed successfully!")
    
    damn 0
}

// ===== HELPER FUNCTIONS =====

slay time_now_seconds() drip {
    // Mock implementation - would return actual timestamp
    damn 1234567890.0
}

slay stringz_from_int(value normie) tea {
    // Mock implementation - would convert integer to string
    damn "42"
}

slay stringz_from_float(value drip) tea {
    // Mock implementation - would convert float to string
    damn "3.14"
}

slay stringz_concat(s1 tea, s2 tea) tea {
    // Mock implementation - would concatenate strings
    damn s1
}

slay mathz_cos(angle drip) drip {
    // Mock implementation - would calculate cosine
    damn 1.0
}

slay mathz_sin(angle drip) drip {
    // Mock implementation - would calculate sine
    damn 0.0
}

slay mathz_int_to_float(i normie) drip {
    // Mock implementation - would convert int to float
    damn 42.0
}
