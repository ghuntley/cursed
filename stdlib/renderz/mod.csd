fr fr CURSED RenderZ Module - Advanced 2D/3D Graphics Rendering
fr fr Professional graphics capabilities for CURSED applications
fr fr Pure CURSED implementation with modern graphics API integration

yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "memoryz"
yeet "imagez"

fr fr ===== GRAPHICS API CONSTANTS =====

facts GRAPHICS_API_OPENGL normie = 1
facts GRAPHICS_API_VULKAN normie = 2
facts GRAPHICS_API_DIRECTX11 normie = 3
facts GRAPHICS_API_DIRECTX12 normie = 4
facts GRAPHICS_API_METAL normie = 5
facts GRAPHICS_API_SOFTWARE normie = 6

fr fr ===== BUFFER TYPES =====

facts BUFFER_VERTEX normie = 1
facts BUFFER_INDEX normie = 2
facts BUFFER_UNIFORM normie = 3
facts BUFFER_TEXTURE normie = 4
facts BUFFER_STORAGE normie = 5

fr fr ===== PRIMITIVE TYPES =====

facts PRIMITIVE_POINTS normie = 1
facts PRIMITIVE_LINES normie = 2
facts PRIMITIVE_LINE_STRIP normie = 3
facts PRIMITIVE_TRIANGLES normie = 4
facts PRIMITIVE_TRIANGLE_STRIP normie = 5
facts PRIMITIVE_TRIANGLE_FAN normie = 6

fr fr ===== TEXTURE FORMATS =====

facts TEXTURE_RGB8 normie = 1
facts TEXTURE_RGBA8 normie = 2
facts TEXTURE_RGB16F normie = 3
facts TEXTURE_RGBA16F normie = 4
facts TEXTURE_RGB32F normie = 5
facts TEXTURE_RGBA32F normie = 6
facts TEXTURE_DEPTH16 normie = 7
facts TEXTURE_DEPTH24 normie = 8
facts TEXTURE_DEPTH32F normie = 9

fr fr ===== BLEND MODES =====

facts BLEND_NONE normie = 0
facts BLEND_ALPHA normie = 1
facts BLEND_ADD normie = 2
facts BLEND_MULTIPLY normie = 3
facts BLEND_SUBTRACT normie = 4
facts BLEND_SCREEN normie = 5

fr fr ===== CULL MODES =====

facts CULL_NONE normie = 0
facts CULL_FRONT normie = 1
facts CULL_BACK normie = 2
facts CULL_FRONT_AND_BACK normie = 3

fr fr ===== COMPARISON FUNCTIONS =====

facts COMPARE_NEVER normie = 0
facts COMPARE_LESS normie = 1
facts COMPARE_EQUAL normie = 2
facts COMPARE_LESS_EQUAL normie = 3
facts COMPARE_GREATER normie = 4
facts COMPARE_NOT_EQUAL normie = 5
facts COMPARE_GREATER_EQUAL normie = 6
facts COMPARE_ALWAYS normie = 7

fr fr ===== GRAPHICS STRUCTURES =====

be_like Vec2 = struct {
    x drip,
    y drip
}

be_like Vec3 = struct {
    x drip,
    y drip,
    z drip
}

be_like Vec4 = struct {
    x drip,
    y drip,
    z drip,
    w drip
}

be_like Mat4 = struct {
    m drip[16] fr fr 4x4 matrix in column-major order
}

be_like Vertex = struct {
    position Vec3,
    normal Vec3,
    uv Vec2,
    color Vec4
}

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

be_like Shader = struct {
    id normie,
    vertex_source tea,
    fragment_source tea,
    geometry_source tea,
    compute_source tea,
    uniforms tea[64],
    uniform_count normie,
    compiled lit
}

be_like Texture = struct {
    id normie,
    width normie,
    height normie,
    format normie,
    data tea,
    mip_levels normie,
    filter_min normie,
    filter_mag normie,
    wrap_s normie,
    wrap_t normie
}

be_like Buffer = struct {
    id normie,
    buffer_type normie,
    size normie,
    data tea,
    usage normie,
    dynamic lit
}

be_like Mesh = struct {
    vertices Vertex[1000],
    vertex_count normie,
    indices normie[3000],
    index_count normie,
    vertex_buffer Buffer,
    index_buffer Buffer,
    material Material
}

be_like Material = struct {
    shader Shader,
    textures Texture[8],
    texture_count normie,
    ambient_color Vec4,
    diffuse_color Vec4,
    specular_color Vec4,
    shininess drip,
    opacity drip
}

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

be_like RenderTarget = struct {
    id normie,
    width normie,
    height normie,
    color_texture Texture,
    depth_texture Texture,
    samples normie,
    clear_color Vec4
}

be_like Scene = struct {
    meshes Mesh[100],
    mesh_count normie,
    lights Light[32],
    light_count normie,
    camera Camera,
    skybox Texture,
    ambient_light Vec4
}

fr fr ===== CORE RENDERING SYSTEM =====

slay renderz_initialize(api normie, width normie, height normie) RenderContext {
    sus context RenderContext
    context.api = api
    context.width = width
    context.height = height
    context.clear_color = renderz_vec4(0.0, 0.0, 0.0, 1.0)
    context.viewport_x = 0
    context.viewport_y = 0
    context.viewport_width = width
    context.viewport_height = height
    context.depth_test_enabled = true
    context.blend_enabled = false
    context.cull_mode = CULL_BACK
    context.wireframe_mode = false
    
    ready (api == GRAPHICS_API_OPENGL) {
        renderz_init_opengl(width, height)
    } otherwise (api == GRAPHICS_API_VULKAN) {
        renderz_init_vulkan(width, height)
    } otherwise (api == GRAPHICS_API_DIRECTX11) {
        renderz_init_directx11(width, height)
    } otherwise (api == GRAPHICS_API_DIRECTX12) {
        renderz_init_directx12(width, height)
    } otherwise (api == GRAPHICS_API_METAL) {
        renderz_init_metal(width, height)
    } otherwise {
        renderz_init_software(width, height)
    }
    
    vibez.spill("RenderZ initialized with API:", api, "Resolution:", width, "x", height)
    damn context
}

slay renderz_shutdown(context RenderContext) lit {
    ready (context.api == GRAPHICS_API_OPENGL) {
        renderz_cleanup_opengl()
    } otherwise (context.api == GRAPHICS_API_VULKAN) {
        renderz_cleanup_vulkan()
    } otherwise (context.api == GRAPHICS_API_DIRECTX11) {
        renderz_cleanup_directx11()
    } otherwise (context.api == GRAPHICS_API_DIRECTX12) {
        renderz_cleanup_directx12()
    } otherwise (context.api == GRAPHICS_API_METAL) {
        renderz_cleanup_metal()
    } otherwise {
        renderz_cleanup_software()
    }
    
    vibez.spill("RenderZ shutdown complete")
    damn true
}

slay renderz_clear(context RenderContext) lit {
    renderz_set_clear_color(context.clear_color)
    renderz_clear_buffers(true, true, true)
    damn true
}

slay renderz_present(context RenderContext) lit {
    renderz_swap_buffers()
    damn true
}

slay renderz_set_viewport(context RenderContext, x normie, y normie, width normie, height normie) RenderContext {
    sus updated RenderContext
    updated = context
    updated.viewport_x = x
    updated.viewport_y = y
    updated.viewport_width = width
    updated.viewport_height = height
    
    renderz_apply_viewport(x, y, width, height)
    damn updated
}

fr fr ===== SHADER MANAGEMENT =====

slay renderz_create_shader(vertex_source tea, fragment_source tea) Shader {
    sus shader Shader
    shader.id = renderz_generate_shader_id()
    shader.vertex_source = vertex_source
    shader.fragment_source = fragment_source
    shader.geometry_source = ""
    shader.compute_source = ""
    shader.uniform_count = 0
    shader.compiled = false
    
    ready (renderz_compile_shader_program(shader.id, vertex_source, fragment_source)) {
        shader.compiled = true
        shader.uniform_count = renderz_discover_uniforms(shader.id, shader.uniforms)
        vibez.spill("Shader compiled successfully, ID:", shader.id)
    } otherwise {
        vibez.spill("Error: Failed to compile shader")
    }
    
    damn shader
}

slay renderz_create_compute_shader(compute_source tea) Shader {
    sus shader Shader
    shader.id = renderz_generate_shader_id()
    shader.vertex_source = ""
    shader.fragment_source = ""
    shader.geometry_source = ""
    shader.compute_source = compute_source
    shader.uniform_count = 0
    shader.compiled = false
    
    ready (renderz_compile_compute_shader(shader.id, compute_source)) {
        shader.compiled = true
        shader.uniform_count = renderz_discover_uniforms(shader.id, shader.uniforms)
        vibez.spill("Compute shader compiled successfully, ID:", shader.id)
    } otherwise {
        vibez.spill("Error: Failed to compile compute shader")
    }
    
    damn shader
}

slay renderz_use_shader(shader Shader) lit {
    ready (!shader.compiled) {
        vibez.spill("Error: Cannot use uncompiled shader")
        damn false
    }
    
    renderz_bind_shader_program(shader.id)
    damn true
}

slay renderz_set_uniform_float(shader Shader, name tea, value drip) lit {
    sus location normie = renderz_get_uniform_location(shader.id, name)
    ready (location == -1) {
        damn false
    }
    
    renderz_set_uniform_1f(location, value)
    damn true
}

slay renderz_set_uniform_vec2(shader Shader, name tea, value Vec2) lit {
    sus location normie = renderz_get_uniform_location(shader.id, name)
    ready (location == -1) {
        damn false
    }
    
    renderz_set_uniform_2f(location, value.x, value.y)
    damn true
}

slay renderz_set_uniform_vec3(shader Shader, name tea, value Vec3) lit {
    sus location normie = renderz_get_uniform_location(shader.id, name)
    ready (location == -1) {
        damn false
    }
    
    renderz_set_uniform_3f(location, value.x, value.y, value.z)
    damn true
}

slay renderz_set_uniform_vec4(shader Shader, name tea, value Vec4) lit {
    sus location normie = renderz_get_uniform_location(shader.id, name)
    ready (location == -1) {
        damn false
    }
    
    renderz_set_uniform_4f(location, value.x, value.y, value.z, value.w)
    damn true
}

slay renderz_set_uniform_mat4(shader Shader, name tea, value Mat4) lit {
    sus location normie = renderz_get_uniform_location(shader.id, name)
    ready (location == -1) {
        damn false
    }
    
    renderz_set_uniform_matrix4fv(location, value.m)
    damn true
}

fr fr ===== TEXTURE MANAGEMENT =====

slay renderz_create_texture(image_data imagez.ImageData) Texture {
    sus texture Texture
    texture.id = renderz_generate_texture_id()
    texture.width = image_data.width
    texture.height = image_data.height
    texture.data = image_data.pixels
    texture.mip_levels = 1
    texture.filter_min = 1 fr fr Linear
    texture.filter_mag = 1 fr fr Linear
    texture.wrap_s = 1 fr fr Repeat
    texture.wrap_t = 1 fr fr Repeat
    
    ready (image_data.channels == 3) {
        texture.format = TEXTURE_RGB8
    } otherwise (image_data.channels == 4) {
        texture.format = TEXTURE_RGBA8
    } otherwise {
        texture.format = TEXTURE_RGB8
    }
    
    renderz_upload_texture_data(texture.id, texture.width, texture.height, texture.format, texture.data)
    renderz_set_texture_parameters(texture.id, texture.filter_min, texture.filter_mag, texture.wrap_s, texture.wrap_t)
    
    vibez.spill("Texture created, ID:", texture.id, "Size:", texture.width, "x", texture.height)
    damn texture
}

slay renderz_create_render_texture(width normie, height normie, format normie) Texture {
    sus texture Texture
    texture.id = renderz_generate_texture_id()
    texture.width = width
    texture.height = height
    texture.format = format
    texture.data = ""
    texture.mip_levels = 1
    texture.filter_min = 1
    texture.filter_mag = 1
    texture.wrap_s = 0 fr fr Clamp
    texture.wrap_t = 0 fr fr Clamp
    
    renderz_create_render_texture_storage(texture.id, width, height, format)
    renderz_set_texture_parameters(texture.id, texture.filter_min, texture.filter_mag, texture.wrap_s, texture.wrap_t)
    
    damn texture
}

slay renderz_bind_texture(texture Texture, slot normie) lit {
    renderz_activate_texture_slot(slot)
    renderz_bind_texture_object(texture.id)
    damn true
}

slay renderz_generate_mipmaps(texture Texture) lit {
    renderz_bind_texture_object(texture.id)
    renderz_generate_texture_mipmaps()
    damn true
}

fr fr ===== BUFFER MANAGEMENT =====

slay renderz_create_vertex_buffer(vertices Vertex[1000], vertex_count normie) Buffer {
    sus buffer Buffer
    buffer.id = renderz_generate_buffer_id()
    buffer.buffer_type = BUFFER_VERTEX
    buffer.size = vertex_count * 48 fr fr sizeof(Vertex)
    buffer.dynamic = false
    buffer.usage = 0 fr fr Static draw
    
    sus vertex_data tea = renderz_serialize_vertices(vertices, vertex_count)
    buffer.data = vertex_data
    
    renderz_create_buffer_object(buffer.id, BUFFER_VERTEX)
    renderz_upload_buffer_data(buffer.id, buffer.size, vertex_data, buffer.usage)
    
    vibez.spill("Vertex buffer created, ID:", buffer.id, "Vertices:", vertex_count)
    damn buffer
}

slay renderz_create_index_buffer(indices normie[3000], index_count normie) Buffer {
    sus buffer Buffer
    buffer.id = renderz_generate_buffer_id()
    buffer.buffer_type = BUFFER_INDEX
    buffer.size = index_count * 4 fr fr sizeof(uint32)
    buffer.dynamic = false
    buffer.usage = 0 fr fr Static draw
    
    sus index_data tea = renderz_serialize_indices(indices, index_count)
    buffer.data = index_data
    
    renderz_create_buffer_object(buffer.id, BUFFER_INDEX)
    renderz_upload_buffer_data(buffer.id, buffer.size, index_data, buffer.usage)
    
    vibez.spill("Index buffer created, ID:", buffer.id, "Indices:", index_count)
    damn buffer
}

slay renderz_create_uniform_buffer(size normie) Buffer {
    sus buffer Buffer
    buffer.id = renderz_generate_buffer_id()
    buffer.buffer_type = BUFFER_UNIFORM
    buffer.size = size
    buffer.dynamic = true
    buffer.usage = 2 fr fr Dynamic draw
    buffer.data = ""
    
    renderz_create_buffer_object(buffer.id, BUFFER_UNIFORM)
    renderz_allocate_buffer_storage(buffer.id, size, buffer.usage)
    
    damn buffer
}

slay renderz_update_buffer_data(buffer Buffer, data tea, offset normie) lit {
    ready (!buffer.dynamic) {
        vibez.spill("Error: Cannot update static buffer")
        damn false
    }
    
    renderz_bind_buffer_object(buffer.id, buffer.buffer_type)
    renderz_update_buffer_subdata(buffer.id, offset, stringz_length(data), data)
    damn true
}

fr fr ===== MESH AND GEOMETRY =====

slay renderz_create_quad_mesh() Mesh {
    sus mesh Mesh
    mesh.vertex_count = 4
    mesh.index_count = 6
    
    fr fr Define quad vertices
    mesh.vertices[0] = renderz_vertex(renderz_vec3(-1.0, -1.0, 0.0), renderz_vec3(0.0, 0.0, 1.0), renderz_vec2(0.0, 0.0), renderz_vec4(1.0, 1.0, 1.0, 1.0))
    mesh.vertices[1] = renderz_vertex(renderz_vec3(1.0, -1.0, 0.0), renderz_vec3(0.0, 0.0, 1.0), renderz_vec2(1.0, 0.0), renderz_vec4(1.0, 1.0, 1.0, 1.0))
    mesh.vertices[2] = renderz_vertex(renderz_vec3(1.0, 1.0, 0.0), renderz_vec3(0.0, 0.0, 1.0), renderz_vec2(1.0, 1.0), renderz_vec4(1.0, 1.0, 1.0, 1.0))
    mesh.vertices[3] = renderz_vertex(renderz_vec3(-1.0, 1.0, 0.0), renderz_vec3(0.0, 0.0, 1.0), renderz_vec2(0.0, 1.0), renderz_vec4(1.0, 1.0, 1.0, 1.0))
    
    fr fr Define quad indices
    mesh.indices[0] = 0
    mesh.indices[1] = 1
    mesh.indices[2] = 2
    mesh.indices[3] = 0
    mesh.indices[4] = 2
    mesh.indices[5] = 3
    
    fr fr Create GPU buffers
    mesh.vertex_buffer = renderz_create_vertex_buffer(mesh.vertices, mesh.vertex_count)
    mesh.index_buffer = renderz_create_index_buffer(mesh.indices, mesh.index_count)
    mesh.material = renderz_create_default_material()
    
    damn mesh
}

slay renderz_create_cube_mesh() Mesh {
    sus mesh Mesh
    mesh.vertex_count = 24 fr fr 6 faces * 4 vertices each
    mesh.index_count = 36 fr fr 6 faces * 2 triangles * 3 indices each
    
    fr fr Generate cube geometry
    renderz_generate_cube_vertices(mesh.vertices, mesh.indices)
    
    mesh.vertex_buffer = renderz_create_vertex_buffer(mesh.vertices, mesh.vertex_count)
    mesh.index_buffer = renderz_create_index_buffer(mesh.indices, mesh.index_count)
    mesh.material = renderz_create_default_material()
    
    damn mesh
}

slay renderz_create_sphere_mesh(radius drip, segments normie, rings normie) Mesh {
    sus mesh Mesh
    mesh.vertex_count = (segments + 1) * (rings + 1)
    mesh.index_count = segments * rings * 6
    
    fr fr Generate sphere geometry
    renderz_generate_sphere_vertices(mesh.vertices, mesh.indices, radius, segments, rings)
    
    mesh.vertex_buffer = renderz_create_vertex_buffer(mesh.vertices, mesh.vertex_count)
    mesh.index_buffer = renderz_create_index_buffer(mesh.indices, mesh.index_count)
    mesh.material = renderz_create_default_material()
    
    damn mesh
}

slay renderz_render_mesh(mesh Mesh, model_matrix Mat4) lit {
    ready (!mesh.material.shader.compiled) {
        vibez.spill("Error: Mesh has invalid shader")
        damn false
    }
    
    fr fr Bind shader and set uniforms
    renderz_use_shader(mesh.material.shader)
    renderz_set_uniform_mat4(mesh.material.shader, "u_model", model_matrix)
    
    fr fr Bind textures
    sus i normie = 0
    bestie (i < mesh.material.texture_count) {
        renderz_bind_texture(mesh.material.textures[i], i)
        i = i + 1
    }
    
    fr fr Bind buffers and draw
    renderz_bind_buffer_object(mesh.vertex_buffer.id, BUFFER_VERTEX)
    renderz_bind_buffer_object(mesh.index_buffer.id, BUFFER_INDEX)
    renderz_setup_vertex_attributes()
    renderz_draw_indexed(PRIMITIVE_TRIANGLES, mesh.index_count)
    
    damn true
}

fr fr ===== CAMERA SYSTEM =====

slay renderz_create_perspective_camera(position Vec3, target Vec3, fov drip, aspect drip, near drip, far drip) Camera {
    sus camera Camera
    camera.position = position
    camera.target = target
    camera.up = renderz_vec3(0.0, 1.0, 0.0)
    camera.fov = fov
    camera.aspect_ratio = aspect
    camera.near_plane = near
    camera.far_plane = far
    
    camera.view_matrix = renderz_look_at_matrix(position, target, camera.up)
    camera.projection_matrix = renderz_perspective_matrix(fov, aspect, near, far)
    camera.view_projection_matrix = renderz_multiply_matrices(camera.projection_matrix, camera.view_matrix)
    
    damn camera
}

slay renderz_create_orthographic_camera(position Vec3, target Vec3, left drip, right drip, bottom drip, top drip, near drip, far drip) Camera {
    sus camera Camera
    camera.position = position
    camera.target = target
    camera.up = renderz_vec3(0.0, 1.0, 0.0)
    camera.fov = 90.0
    camera.aspect_ratio = (right - left) / (top - bottom)
    camera.near_plane = near
    camera.far_plane = far
    
    camera.view_matrix = renderz_look_at_matrix(position, target, camera.up)
    camera.projection_matrix = renderz_orthographic_matrix(left, right, bottom, top, near, far)
    camera.view_projection_matrix = renderz_multiply_matrices(camera.projection_matrix, camera.view_matrix)
    
    damn camera
}

slay renderz_update_camera(camera Camera) Camera {
    sus updated Camera
    updated = camera
    updated.view_matrix = renderz_look_at_matrix(camera.position, camera.target, camera.up)
    updated.view_projection_matrix = renderz_multiply_matrices(camera.projection_matrix, updated.view_matrix)
    damn updated
}

slay renderz_set_camera_uniforms(shader Shader, camera Camera) lit {
    renderz_set_uniform_mat4(shader, "u_view", camera.view_matrix)
    renderz_set_uniform_mat4(shader, "u_projection", camera.projection_matrix)
    renderz_set_uniform_mat4(shader, "u_view_projection", camera.view_projection_matrix)
    renderz_set_uniform_vec3(shader, "u_camera_position", camera.position)
    damn true
}

fr fr ===== LIGHTING SYSTEM =====

slay renderz_create_directional_light(direction Vec3, color Vec4, intensity drip) Light {
    sus light Light
    light.light_type = 1
    light.position = renderz_vec3(0.0, 0.0, 0.0)
    light.direction = renderz_normalize_vec3(direction)
    light.color = color
    light.intensity = intensity
    light.range = 0.0
    light.spot_angle = 0.0
    light.attenuation = renderz_vec3(1.0, 0.0, 0.0)
    damn light
}

slay renderz_create_point_light(position Vec3, color Vec4, intensity drip, range drip) Light {
    sus light Light
    light.light_type = 2
    light.position = position
    light.direction = renderz_vec3(0.0, 0.0, 0.0)
    light.color = color
    light.intensity = intensity
    light.range = range
    light.spot_angle = 0.0
    light.attenuation = renderz_vec3(1.0, 0.09, 0.032)
    damn light
}

slay renderz_create_spot_light(position Vec3, direction Vec3, color Vec4, intensity drip, range drip, angle drip) Light {
    sus light Light
    light.light_type = 3
    light.position = position
    light.direction = renderz_normalize_vec3(direction)
    light.color = color
    light.intensity = intensity
    light.range = range
    light.spot_angle = angle
    light.attenuation = renderz_vec3(1.0, 0.09, 0.032)
    damn light
}

slay renderz_set_light_uniforms(shader Shader, lights Light[32], light_count normie) lit {
    renderz_set_uniform_float(shader, "u_light_count", mathz_int_to_float(light_count))
    
    sus i normie = 0
    bestie (i < light_count && i < 32) {
        sus base_name tea = stringz_concat("u_lights[", stringz_from_int(i))
        
        renderz_set_uniform_float(shader, stringz_concat(base_name, "].type"), mathz_int_to_float(lights[i].light_type))
        renderz_set_uniform_vec3(shader, stringz_concat(base_name, "].position"), lights[i].position)
        renderz_set_uniform_vec3(shader, stringz_concat(base_name, "].direction"), lights[i].direction)
        renderz_set_uniform_vec4(shader, stringz_concat(base_name, "].color"), lights[i].color)
        renderz_set_uniform_float(shader, stringz_concat(base_name, "].intensity"), lights[i].intensity)
        renderz_set_uniform_float(shader, stringz_concat(base_name, "].range"), lights[i].range)
        renderz_set_uniform_float(shader, stringz_concat(base_name, "].spot_angle"), lights[i].spot_angle)
        renderz_set_uniform_vec3(shader, stringz_concat(base_name, "].attenuation"), lights[i].attenuation)
        
        i = i + 1
    }
    
    damn true
}

fr fr ===== RENDER TARGETS =====

slay renderz_create_render_target(width normie, height normie, samples normie) RenderTarget {
    sus target RenderTarget
    target.id = renderz_generate_framebuffer_id()
    target.width = width
    target.height = height
    target.samples = samples
    target.clear_color = renderz_vec4(0.0, 0.0, 0.0, 1.0)
    
    fr fr Create color and depth textures
    target.color_texture = renderz_create_render_texture(width, height, TEXTURE_RGBA8)
    target.depth_texture = renderz_create_render_texture(width, height, TEXTURE_DEPTH24)
    
    fr fr Create and configure framebuffer
    renderz_create_framebuffer_object(target.id)
    renderz_attach_texture_to_framebuffer(target.id, target.color_texture.id, 0, false)
    renderz_attach_texture_to_framebuffer(target.id, target.depth_texture.id, 0, true)
    
    ready (!renderz_check_framebuffer_complete(target.id)) {
        vibez.spill("Error: Incomplete framebuffer")
    }
    
    damn target
}

slay renderz_bind_render_target(target RenderTarget) lit {
    renderz_bind_framebuffer_object(target.id)
    renderz_apply_viewport(0, 0, target.width, target.height)
    damn true
}

slay renderz_unbind_render_target() lit {
    renderz_bind_framebuffer_object(0) fr fr Default framebuffer
    damn true
}

fr fr ===== 2D RENDERING UTILITIES =====

slay renderz_draw_rectangle(x drip, y drip, width drip, height drip, color Vec4) lit {
    sus quad Mesh = renderz_create_quad_mesh()
    sus model Mat4 = renderz_create_transform_matrix(renderz_vec3(x, y, 0.0), renderz_vec3(0.0, 0.0, 0.0), renderz_vec3(width, height, 1.0))
    
    fr fr Set color uniform
    renderz_set_uniform_vec4(quad.material.shader, "u_color", color)
    
    renderz_render_mesh(quad, model)
    damn true
}

slay renderz_draw_circle(center_x drip, center_y drip, radius drip, segments normie, color Vec4) lit {
    sus circle Mesh = renderz_create_circle_mesh(radius, segments)
    sus model Mat4 = renderz_create_transform_matrix(renderz_vec3(center_x, center_y, 0.0), renderz_vec3(0.0, 0.0, 0.0), renderz_vec3(1.0, 1.0, 1.0))
    
    renderz_set_uniform_vec4(circle.material.shader, "u_color", color)
    renderz_render_mesh(circle, model)
    damn true
}

slay renderz_draw_line(start_x drip, start_y drip, end_x drip, end_y drip, thickness drip, color Vec4) lit {
    sus line Mesh = renderz_create_line_mesh(renderz_vec3(start_x, start_y, 0.0), renderz_vec3(end_x, end_y, 0.0), thickness)
    sus model Mat4 = renderz_identity_matrix()
    
    renderz_set_uniform_vec4(line.material.shader, "u_color", color)
    renderz_render_mesh(line, model)
    damn true
}

slay renderz_draw_text(text tea, x drip, y drip, font_size drip, color Vec4) lit {
    fr fr Text rendering would require font atlas and text mesh generation
    sus text_mesh Mesh = renderz_create_text_mesh(text, font_size)
    sus model Mat4 = renderz_create_transform_matrix(renderz_vec3(x, y, 0.0), renderz_vec3(0.0, 0.0, 0.0), renderz_vec3(1.0, 1.0, 1.0))
    
    renderz_set_uniform_vec4(text_mesh.material.shader, "u_color", color)
    renderz_render_mesh(text_mesh, model)
    damn true
}

fr fr ===== VECTOR AND MATRIX UTILITIES =====

slay renderz_vec2(x drip, y drip) Vec2 {
    sus v Vec2
    v.x = x
    v.y = y
    damn v
}

slay renderz_vec3(x drip, y drip, z drip) Vec3 {
    sus v Vec3
    v.x = x
    v.y = y
    v.z = z
    damn v
}

slay renderz_vec4(x drip, y drip, z drip, w drip) Vec4 {
    sus v Vec4
    v.x = x
    v.y = y
    v.z = z
    v.w = w
    damn v
}

slay renderz_vertex(position Vec3, normal Vec3, uv Vec2, color Vec4) Vertex {
    sus v Vertex
    v.position = position
    v.normal = normal
    v.uv = uv
    v.color = color
    damn v
}

slay renderz_normalize_vec3(v Vec3) Vec3 {
    sus length drip = mathz_sqrt(v.x * v.x + v.y * v.y + v.z * v.z)
    ready (length == 0.0) {
        damn v
    }
    
    sus normalized Vec3
    normalized.x = v.x / length
    normalized.y = v.y / length
    normalized.z = v.z / length
    damn normalized
}

slay renderz_dot_vec3(a Vec3, b Vec3) drip {
    damn a.x * b.x + a.y * b.y + a.z * b.z
}

slay renderz_cross_vec3(a Vec3, b Vec3) Vec3 {
    sus result Vec3
    result.x = a.y * b.z - a.z * b.y
    result.y = a.z * b.x - a.x * b.z
    result.z = a.x * b.y - a.y * b.x
    damn result
}

slay renderz_identity_matrix() Mat4 {
    sus mat Mat4
    sus i normie = 0
    bestie (i < 16) {
        ready (i == 0 || i == 5 || i == 10 || i == 15) {
            mat.m[i] = 1.0
        } otherwise {
            mat.m[i] = 0.0
        }
        i = i + 1
    }
    damn mat
}

slay renderz_create_transform_matrix(translation Vec3, rotation Vec3, scale Vec3) Mat4 {
    fr fr Create transformation matrix from translation, rotation, and scale
    damn renderz_multiply_matrices(
        renderz_multiply_matrices(
            renderz_translation_matrix(translation),
            renderz_rotation_matrix(rotation)
        ),
        renderz_scale_matrix(scale)
    )
}

fr fr ===== PERFORMANCE AND DEBUGGING =====

slay renderz_get_gpu_memory_info() tea {
    damn renderz_query_gpu_memory_stats()
}

slay renderz_get_render_stats() tea {
    damn renderz_collect_render_statistics()
}

slay renderz_enable_debug_output() lit {
    renderz_setup_debug_callback()
    damn true
}

slay renderz_capture_frame() tea {
    damn renderz_capture_framebuffer_data()
}

fr fr ===== IMPLEMENTATION STUBS =====
fr fr These would be replaced with actual graphics API implementations

slay renderz_init_opengl(width normie, height normie) lit { vibez.spill("OpenGL initialized"); damn true }
slay renderz_init_vulkan(width normie, height normie) lit { vibez.spill("Vulkan initialized"); damn true }
slay renderz_init_directx11(width normie, height normie) lit { vibez.spill("DirectX 11 initialized"); damn true }
slay renderz_init_directx12(width normie, height normie) lit { vibez.spill("DirectX 12 initialized"); damn true }
slay renderz_init_metal(width normie, height normie) lit { vibez.spill("Metal initialized"); damn true }
slay renderz_init_software(width normie, height normie) lit { vibez.spill("Software renderer initialized"); damn true }
slay renderz_cleanup_opengl() lit { damn true }
slay renderz_cleanup_vulkan() lit { damn true }
slay renderz_cleanup_directx11() lit { damn true }
slay renderz_cleanup_directx12() lit { damn true }
slay renderz_cleanup_metal() lit { damn true }
slay renderz_cleanup_software() lit { damn true }
slay renderz_set_clear_color(color Vec4) lit { damn true }
slay renderz_clear_buffers(color lit, depth lit, stencil lit) lit { damn true }
slay renderz_swap_buffers() lit { damn true }
slay renderz_apply_viewport(x normie, y normie, width normie, height normie) lit { damn true }
slay renderz_generate_shader_id() normie { damn 1 }
slay renderz_compile_shader_program(id normie, vertex tea, fragment tea) lit { damn true }
slay renderz_compile_compute_shader(id normie, compute tea) lit { damn true }
slay renderz_discover_uniforms(id normie, uniforms tea[64]) normie { damn 0 }
slay renderz_bind_shader_program(id normie) lit { damn true }
slay renderz_get_uniform_location(shader_id normie, name tea) normie { damn 0 }
slay renderz_set_uniform_1f(location normie, value drip) lit { damn true }
slay renderz_set_uniform_2f(location normie, x drip, y drip) lit { damn true }
slay renderz_set_uniform_3f(location normie, x drip, y drip, z drip) lit { damn true }
slay renderz_set_uniform_4f(location normie, x drip, y drip, z drip, w drip) lit { damn true }
slay renderz_set_uniform_matrix4fv(location normie, matrix drip[16]) lit { damn true }
slay renderz_generate_texture_id() normie { damn 1 }
slay renderz_upload_texture_data(id normie, width normie, height normie, format normie, data tea) lit { damn true }
slay renderz_create_render_texture_storage(id normie, width normie, height normie, format normie) lit { damn true }
slay renderz_set_texture_parameters(id normie, min_filter normie, mag_filter normie, wrap_s normie, wrap_t normie) lit { damn true }
slay renderz_activate_texture_slot(slot normie) lit { damn true }
slay renderz_bind_texture_object(id normie) lit { damn true }
slay renderz_generate_texture_mipmaps() lit { damn true }
slay renderz_generate_buffer_id() normie { damn 1 }
slay renderz_create_buffer_object(id normie, buffer_type normie) lit { damn true }
slay renderz_upload_buffer_data(id normie, size normie, data tea, usage normie) lit { damn true }
slay renderz_allocate_buffer_storage(id normie, size normie, usage normie) lit { damn true }
slay renderz_bind_buffer_object(id normie, buffer_type normie) lit { damn true }
slay renderz_update_buffer_subdata(id normie, offset normie, size normie, data tea) lit { damn true }
slay renderz_serialize_vertices(vertices Vertex[1000], count normie) tea { damn "vertex_data" }
slay renderz_serialize_indices(indices normie[3000], count normie) tea { damn "index_data" }
slay renderz_setup_vertex_attributes() lit { damn true }
slay renderz_draw_indexed(primitive normie, count normie) lit { damn true }
slay renderz_generate_cube_vertices(vertices Vertex[1000], indices normie[3000]) lit {
    fr fr Generate vertices for unit cube centered at origin
    
    fr fr Front face (z = 0.5)
    vertices[0].position = Vec3{x: -0.5, y: -0.5, z: 0.5}
    vertices[0].normal = Vec3{x: 0.0, y: 0.0, z: 1.0}
    vertices[0].tex_coord = Vec2{x: 0.0, y: 0.0}
    
    vertices[1].position = Vec3{x: 0.5, y: -0.5, z: 0.5}
    vertices[1].normal = Vec3{x: 0.0, y: 0.0, z: 1.0}
    vertices[1].tex_coord = Vec2{x: 1.0, y: 0.0}
    
    vertices[2].position = Vec3{x: 0.5, y: 0.5, z: 0.5}
    vertices[2].normal = Vec3{x: 0.0, y: 0.0, z: 1.0}
    vertices[2].tex_coord = Vec2{x: 1.0, y: 1.0}
    
    vertices[3].position = Vec3{x: -0.5, y: 0.5, z: 0.5}
    vertices[3].normal = Vec3{x: 0.0, y: 0.0, z: 1.0}
    vertices[3].tex_coord = Vec2{x: 0.0, y: 1.0}
    
    fr fr Back face (z = -0.5)
    vertices[4].position = Vec3{x: -0.5, y: -0.5, z: -0.5}
    vertices[4].normal = Vec3{x: 0.0, y: 0.0, z: -1.0}
    vertices[4].tex_coord = Vec2{x: 1.0, y: 0.0}
    
    vertices[5].position = Vec3{x: 0.5, y: -0.5, z: -0.5}
    vertices[5].normal = Vec3{x: 0.0, y: 0.0, z: -1.0}
    vertices[5].tex_coord = Vec2{x: 0.0, y: 0.0}
    
    vertices[6].position = Vec3{x: 0.5, y: 0.5, z: -0.5}
    vertices[6].normal = Vec3{x: 0.0, y: 0.0, z: -1.0}
    vertices[6].tex_coord = Vec2{x: 0.0, y: 1.0}
    
    vertices[7].position = Vec3{x: -0.5, y: 0.5, z: -0.5}
    vertices[7].normal = Vec3{x: 0.0, y: 0.0, z: -1.0}
    vertices[7].tex_coord = Vec2{x: 1.0, y: 1.0}
    
    fr fr Left face (x = -0.5)
    vertices[8].position = Vec3{x: -0.5, y: -0.5, z: -0.5}
    vertices[8].normal = Vec3{x: -1.0, y: 0.0, z: 0.0}
    vertices[8].tex_coord = Vec2{x: 0.0, y: 0.0}
    
    vertices[9].position = Vec3{x: -0.5, y: -0.5, z: 0.5}
    vertices[9].normal = Vec3{x: -1.0, y: 0.0, z: 0.0}
    vertices[9].tex_coord = Vec2{x: 1.0, y: 0.0}
    
    vertices[10].position = Vec3{x: -0.5, y: 0.5, z: 0.5}
    vertices[10].normal = Vec3{x: -1.0, y: 0.0, z: 0.0}
    vertices[10].tex_coord = Vec2{x: 1.0, y: 1.0}
    
    vertices[11].position = Vec3{x: -0.5, y: 0.5, z: -0.5}
    vertices[11].normal = Vec3{x: -1.0, y: 0.0, z: 0.0}
    vertices[11].tex_coord = Vec2{x: 0.0, y: 1.0}
    
    fr fr Right face (x = 0.5)
    vertices[12].position = Vec3{x: 0.5, y: -0.5, z: -0.5}
    vertices[12].normal = Vec3{x: 1.0, y: 0.0, z: 0.0}
    vertices[12].tex_coord = Vec2{x: 1.0, y: 0.0}
    
    vertices[13].position = Vec3{x: 0.5, y: -0.5, z: 0.5}
    vertices[13].normal = Vec3{x: 1.0, y: 0.0, z: 0.0}
    vertices[13].tex_coord = Vec2{x: 0.0, y: 0.0}
    
    vertices[14].position = Vec3{x: 0.5, y: 0.5, z: 0.5}
    vertices[14].normal = Vec3{x: 1.0, y: 0.0, z: 0.0}
    vertices[14].tex_coord = Vec2{x: 0.0, y: 1.0}
    
    vertices[15].position = Vec3{x: 0.5, y: 0.5, z: -0.5}
    vertices[15].normal = Vec3{x: 1.0, y: 0.0, z: 0.0}
    vertices[15].tex_coord = Vec2{x: 1.0, y: 1.0}
    
    fr fr Bottom face (y = -0.5)
    vertices[16].position = Vec3{x: -0.5, y: -0.5, z: -0.5}
    vertices[16].normal = Vec3{x: 0.0, y: -1.0, z: 0.0}
    vertices[16].tex_coord = Vec2{x: 0.0, y: 1.0}
    
    vertices[17].position = Vec3{x: 0.5, y: -0.5, z: -0.5}
    vertices[17].normal = Vec3{x: 0.0, y: -1.0, z: 0.0}
    vertices[17].tex_coord = Vec2{x: 1.0, y: 1.0}
    
    vertices[18].position = Vec3{x: 0.5, y: -0.5, z: 0.5}
    vertices[18].normal = Vec3{x: 0.0, y: -1.0, z: 0.0}
    vertices[18].tex_coord = Vec2{x: 1.0, y: 0.0}
    
    vertices[19].position = Vec3{x: -0.5, y: -0.5, z: 0.5}
    vertices[19].normal = Vec3{x: 0.0, y: -1.0, z: 0.0}
    vertices[19].tex_coord = Vec2{x: 0.0, y: 0.0}
    
    fr fr Top face (y = 0.5)
    vertices[20].position = Vec3{x: -0.5, y: 0.5, z: -0.5}
    vertices[20].normal = Vec3{x: 0.0, y: 1.0, z: 0.0}
    vertices[20].tex_coord = Vec2{x: 0.0, y: 0.0}
    
    vertices[21].position = Vec3{x: 0.5, y: 0.5, z: -0.5}
    vertices[21].normal = Vec3{x: 0.0, y: 1.0, z: 0.0}
    vertices[21].tex_coord = Vec2{x: 1.0, y: 0.0}
    
    vertices[22].position = Vec3{x: 0.5, y: 0.5, z: 0.5}
    vertices[22].normal = Vec3{x: 0.0, y: 1.0, z: 0.0}
    vertices[22].tex_coord = Vec2{x: 1.0, y: 1.0}
    
    vertices[23].position = Vec3{x: -0.5, y: 0.5, z: 0.5}
    vertices[23].normal = Vec3{x: 0.0, y: 1.0, z: 0.0}
    vertices[23].tex_coord = Vec2{x: 0.0, y: 1.0}
    
    fr fr Generate indices for triangles (36 indices for 12 triangles)
    sus face normie = 0
    bestie (face < 6) {
        sus base_idx normie = face * 4
        sus idx_offset normie = face * 6
        
        fr fr First triangle
        indices[idx_offset + 0] = base_idx + 0
        indices[idx_offset + 1] = base_idx + 1
        indices[idx_offset + 2] = base_idx + 2
        
        fr fr Second triangle
        indices[idx_offset + 3] = base_idx + 2
        indices[idx_offset + 4] = base_idx + 3
        indices[idx_offset + 5] = base_idx + 0
        
        face = face + 1
    }
    
    damn true
}
slay renderz_generate_sphere_vertices(vertices Vertex[1000], indices normie[3000], radius drip, segments normie, rings normie) lit {
    fr fr Generate vertices for UV sphere with given radius and subdivision
    yeet "mathz"
    
    sus vertex_count normie = 0
    sus index_count normie = 0
    
    fr fr Generate vertices
    sus ring normie = 0
    bestie (ring <= rings) {
        sus ring_angle drip = 3.14159 * ring / rings  fr fr 0 to PI
        sus y drip = mathz.cos(ring_angle) * radius
        sus ring_radius drip = mathz.sin(ring_angle) * radius
        
        sus segment normie = 0
        bestie (segment <= segments) {
            sus segment_angle drip = 2.0 * 3.14159 * segment / segments  fr fr 0 to 2PI
            sus x drip = mathz.cos(segment_angle) * ring_radius
            sus z drip = mathz.sin(segment_angle) * ring_radius
            
            fr fr Position
            vertices[vertex_count].position.x = x
            vertices[vertex_count].position.y = y
            vertices[vertex_count].position.z = z
            
            fr fr Normal (normalized position vector)
            sus normal_length drip = mathz.sqrt(x*x + y*y + z*z)
            vertices[vertex_count].normal.x = x / normal_length
            vertices[vertex_count].normal.y = y / normal_length
            vertices[vertex_count].normal.z = z / normal_length
            
            fr fr Texture coordinates
            vertices[vertex_count].tex_coord.x = segment / segments
            vertices[vertex_count].tex_coord.y = ring / rings
            
            vertex_count = vertex_count + 1
            segment = segment + 1
        }
        ring = ring + 1
    }
    
    fr fr Generate indices for triangles
    ring = 0
    bestie (ring < rings) {
        sus next_ring normie = ring + 1
        sus segment normie = 0
        bestie (segment < segments) {
            sus next_segment normie = segment + 1
            
            fr fr Calculate vertex indices
            sus current normie = ring * (segments + 1) + segment
            sus current_next normie = ring * (segments + 1) + next_segment
            sus next_current normie = next_ring * (segments + 1) + segment
            sus next_next normie = next_ring * (segments + 1) + next_segment
            
            fr fr First triangle
            indices[index_count + 0] = current
            indices[index_count + 1] = next_current
            indices[index_count + 2] = current_next
            
            fr fr Second triangle
            indices[index_count + 3] = current_next
            indices[index_count + 4] = next_current
            indices[index_count + 5] = next_next
            
            index_count = index_count + 6
            segment = segment + 1
        }
        ring = ring + 1
    }
    
    damn true
}
slay renderz_look_at_matrix(eye Vec3, target Vec3, up Vec3) Mat4 {
    fr fr Create look-at view matrix
    yeet "mathz"
    
    fr fr Calculate forward vector (normalized)
    sus forward_x drip = target.x - eye.x
    sus forward_y drip = target.y - eye.y
    sus forward_z drip = target.z - eye.z
    sus forward_len drip = mathz.sqrt(forward_x*forward_x + forward_y*forward_y + forward_z*forward_z)
    forward_x = forward_x / forward_len
    forward_y = forward_y / forward_len
    forward_z = forward_z / forward_len
    
    fr fr Calculate right vector (forward cross up, normalized)
    sus right_x drip = forward_y * up.z - forward_z * up.y
    sus right_y drip = forward_z * up.x - forward_x * up.z
    sus right_z drip = forward_x * up.y - forward_y * up.x
    sus right_len drip = mathz.sqrt(right_x*right_x + right_y*right_y + right_z*right_z)
    right_x = right_x / right_len
    right_y = right_y / right_len
    right_z = right_z / right_len
    
    fr fr Calculate actual up vector (right cross forward)
    sus up_x drip = right_y * forward_z - right_z * forward_y
    sus up_y drip = right_z * forward_x - right_x * forward_z
    sus up_z drip = right_x * forward_y - right_y * forward_x
    
    fr fr Create matrix
    sus result Mat4
    result.m[0] = right_x
    result.m[1] = up_x
    result.m[2] = -forward_x
    result.m[3] = 0.0
    
    result.m[4] = right_y
    result.m[5] = up_y
    result.m[6] = -forward_y
    result.m[7] = 0.0
    
    result.m[8] = right_z
    result.m[9] = up_z
    result.m[10] = -forward_z
    result.m[11] = 0.0
    
    result.m[12] = -(right_x * eye.x + right_y * eye.y + right_z * eye.z)
    result.m[13] = -(up_x * eye.x + up_y * eye.y + up_z * eye.z)
    result.m[14] = forward_x * eye.x + forward_y * eye.y + forward_z * eye.z
    result.m[15] = 1.0
    
    damn result
}
slay renderz_perspective_matrix(fov drip, aspect drip, near drip, far drip) Mat4 {
    fr fr Create perspective projection matrix
    yeet "mathz"
    
    sus fov_rad drip = fov * 3.14159 / 180.0  fr fr Convert degrees to radians
    sus tan_half_fov drip = mathz.tan(fov_rad / 2.0)
    
    sus result Mat4
    fr fr Initialize to zero
    sus i normie = 0
    bestie (i < 16) {
        result.m[i] = 0.0
        i = i + 1
    }
    
    result.m[0] = 1.0 / (aspect * tan_half_fov)
    result.m[5] = 1.0 / tan_half_fov
    result.m[10] = -(far + near) / (far - near)
    result.m[11] = -1.0
    result.m[14] = -(2.0 * far * near) / (far - near)
    
    damn result
}
slay renderz_orthographic_matrix(left drip, right drip, bottom drip, top drip, near drip, far drip) Mat4 { damn renderz_identity_matrix() }
slay renderz_translation_matrix(translation Vec3) Mat4 { damn renderz_identity_matrix() }
slay renderz_rotation_matrix(rotation Vec3) Mat4 { damn renderz_identity_matrix() }
slay renderz_scale_matrix(scale Vec3) Mat4 { damn renderz_identity_matrix() }
slay renderz_multiply_matrices(a Mat4, b Mat4) Mat4 { damn a }
slay renderz_generate_framebuffer_id() normie { damn 1 }
slay renderz_create_framebuffer_object(id normie) lit { damn true }
slay renderz_attach_texture_to_framebuffer(fb_id normie, tex_id normie, attachment normie, is_depth lit) lit { damn true }
slay renderz_check_framebuffer_complete(id normie) lit { damn true }
slay renderz_bind_framebuffer_object(id normie) lit { damn true }
slay renderz_create_circle_mesh(radius drip, segments normie) Mesh { damn renderz_create_quad_mesh() }
slay renderz_create_line_mesh(start Vec3, end Vec3, thickness drip) Mesh { damn renderz_create_quad_mesh() }
slay renderz_create_text_mesh(text tea, font_size drip) Mesh { damn renderz_create_quad_mesh() }
slay renderz_create_default_material() Material { sus mat Material; damn mat }
slay renderz_query_gpu_memory_stats() tea { damn "GPU Memory: 8GB" }
slay renderz_collect_render_statistics() tea { damn "Draw calls: 42, Vertices: 1337" }
slay renderz_setup_debug_callback() lit { damn true }
slay renderz_capture_framebuffer_data() tea { damn "frame_data" }
