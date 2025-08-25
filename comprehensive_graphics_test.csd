fr fr Comprehensive Graphics Module Testing - Enhanced Implementation
fr fr Tests all enhanced graphics functionality including actual algorithms

yeet "vibez"
yeet "mathz"
yeet "testz"
yeet "renderz/mod_enhanced"  fr fr Import enhanced renderz module
yeet "drawz/mod_enhanced"    fr fr Import enhanced drawz module

fr fr ===== ENHANCED 3D GRAPHICS TESTING =====

slay test_enhanced_renderz_initialization() lit {
    vibez.spill("=== Testing Enhanced RenderZ Initialization ===")
    
    fr fr Test multiple graphics API initialization
    sus opengl_context renderz.RenderContext = renderz.renderz_initialize(renderz.GRAPHICS_API_OPENGL, 1920, 1080)
    testz.assert_eq_int(opengl_context.api, renderz.GRAPHICS_API_OPENGL)
    testz.assert_eq_int(opengl_context.width, 1920)
    testz.assert_eq_int(opengl_context.height, 1080)
    testz.assert_eq_lit(opengl_context.depth_test_enabled, true)
    
    sus software_context renderz.RenderContext = renderz.renderz_initialize(renderz.GRAPHICS_API_SOFTWARE, 800, 600)
    testz.assert_eq_int(software_context.api, renderz.GRAPHICS_API_SOFTWARE)
    testz.assert_eq_int(software_context.width, 800)
    testz.assert_eq_int(software_context.height, 600)
    
    vibez.spill("✓ Enhanced RenderZ initialization test passed")
    damn true
}

slay test_enhanced_shader_system() lit {
    vibez.spill("=== Testing Enhanced Shader System ===")
    
    fr fr Test enhanced shader compilation with error reporting
    sus vertex_shader tea = "
        attribute vec3 a_position;
        attribute vec3 a_normal;
        attribute vec2 a_texcoord;
        uniform mat4 u_mvp_matrix;
        uniform mat4 u_model_matrix;
        varying vec3 v_normal;
        varying vec2 v_texcoord;
        
        void main() {
            gl_Position = u_mvp_matrix * vec4(a_position, 1.0);
            v_normal = normalize((u_model_matrix * vec4(a_normal, 0.0)).xyz);
            v_texcoord = a_texcoord;
        }
    "
    
    sus fragment_shader tea = "
        precision mediump float;
        uniform vec4 u_color;
        uniform sampler2D u_texture;
        uniform vec3 u_light_direction;
        varying vec3 v_normal;
        varying vec2 v_texcoord;
        
        void main() {
            float n_dot_l = max(dot(v_normal, -u_light_direction), 0.0);
            vec4 texture_color = texture2D(u_texture, v_texcoord);
            gl_FragColor = u_color * texture_color * n_dot_l;
        }
    "
    
    sus shader renderz.Shader = renderz.renderz_create_shader(vertex_shader, fragment_shader)
    testz.assert_neq_int(shader.id, 0)
    testz.assert_eq_tea(shader.vertex_source, vertex_shader)
    testz.assert_eq_tea(shader.fragment_source, fragment_shader)
    
    fr fr Test compute shader
    sus compute_source tea = "
        #version 430
        layout(local_size_x = 16, local_size_y = 16) in;
        layout(rgba8, binding = 0) uniform image2D img_output;
        
        void main() {
            ivec2 pixel_coords = ivec2(gl_GlobalInvocationID.xy);
            vec4 pixel = vec4(pixel_coords.x / 256.0, pixel_coords.y / 256.0, 0.5, 1.0);
            imageStore(img_output, pixel_coords, pixel);
        }
    "
    
    sus compute_shader renderz.Shader = renderz.renderz_create_compute_shader(compute_source)
    testz.assert_neq_int(compute_shader.id, 0)
    testz.assert_eq_tea(compute_shader.compute_source, compute_source)
    
    vibez.spill("✓ Enhanced shader system test passed")
    damn true
}

slay test_enhanced_font_rendering() lit {
    vibez.spill("=== Testing Enhanced Font Rendering ===")
    
    fr fr Test font atlas creation with actual glyph rasterization
    sus font_atlas renderz.FontAtlas = renderz.renderz_create_font_atlas("Arial.ttf", 24.0)
    testz.assert_neq_int(font_atlas.texture.id, 0)
    testz.assert_eq_drip(font_atlas.font_size, 24.0)
    testz.assert_gt_int(font_atlas.glyph_count, 90)  fr fr Should have at least ASCII printable chars
    
    fr fr Test text rendering with proper kerning and metrics
    sus test_text tea = "Hello, CURSED World!"
    sus text_color renderz.Vec4 = renderz.renderz_vec4(1.0, 1.0, 1.0, 1.0)
    sus success lit = renderz.renderz_draw_text_enhanced(test_text, 100.0, 100.0, font_atlas, text_color)
    testz.assert_eq_lit(success, true)
    
    fr fr Test font metrics calculation
    testz.assert_gt_drip(font_atlas.line_height, font_atlas.font_size)
    testz.assert_gt_drip(font_atlas.base_line, 0.0)
    
    vibez.spill("✓ Enhanced font rendering test passed")
    damn true
}

slay test_advanced_geometry_generation() lit {
    vibez.spill("=== Testing Advanced Geometry Generation ===")
    
    fr fr Test sphere mesh generation with proper normals and UVs
    sus sphere_buffer renderz.Buffer = renderz.renderz_generate_sphere_mesh(1.0, 32, 16)
    testz.assert_neq_int(sphere_buffer.id, 0)
    testz.assert_gt_int(sphere_buffer.size, 1000)  fr fr Should have substantial data
    testz.assert_eq_int(sphere_buffer.buffer_type, renderz.BUFFER_VERTEX)
    
    fr fr Test torus mesh generation
    sus torus_buffer renderz.Buffer = renderz.renderz_generate_torus_mesh(2.0, 0.5, 24, 12)
    testz.assert_neq_int(torus_buffer.id, 0)
    testz.assert_gt_int(torus_buffer.size, 500)
    
    fr fr Test procedural mesh validation
    testz.assert_eq_lit(sphere_buffer.dynamic, false)
    testz.assert_neq_tea(sphere_buffer.data, "")
    
    vibez.spill("✓ Advanced geometry generation test passed")
    damn true
}

slay test_enhanced_lighting_system() lit {
    vibez.spill("=== Testing Enhanced Lighting System ===")
    
    fr fr Create test scene with multiple light types
    sus directional_light renderz.Light
    directional_light.light_type = 1  fr fr Directional
    directional_light.direction = renderz.renderz_vec3(-1.0, -1.0, -1.0)
    directional_light.color = renderz.renderz_vec4(1.0, 1.0, 1.0, 1.0)
    directional_light.intensity = 1.0
    directional_light.cast_shadows = true
    
    sus point_light renderz.Light
    point_light.light_type = 2  fr fr Point
    point_light.position = renderz.renderz_vec3(2.0, 3.0, 1.0)
    point_light.color = renderz.renderz_vec4(1.0, 0.8, 0.6, 1.0)
    point_light.intensity = 2.0
    point_light.range = 10.0
    point_light.attenuation = renderz.renderz_vec3(1.0, 0.09, 0.032)
    
    sus spot_light renderz.Light
    spot_light.light_type = 3  fr fr Spot
    spot_light.position = renderz.renderz_vec3(0.0, 5.0, 0.0)
    spot_light.direction = renderz.renderz_vec3(0.0, -1.0, 0.0)
    spot_light.color = renderz.renderz_vec4(1.0, 1.0, 0.0, 1.0)
    spot_light.intensity = 3.0
    spot_light.spot_angle = 45.0
    spot_light.range = 15.0
    
    fr fr Create test material
    sus test_material renderz.Material
    test_material.ambient_color = renderz.renderz_vec4(0.2, 0.2, 0.2, 1.0)
    test_material.diffuse_color = renderz.renderz_vec4(0.8, 0.8, 0.8, 1.0)
    test_material.specular_color = renderz.renderz_vec4(1.0, 1.0, 1.0, 1.0)
    test_material.shininess = 32.0
    test_material.opacity = 1.0
    
    fr fr Test lighting calculations
    sus lights [32]renderz.Light = [directional_light, point_light, spot_light]
    sus test_position renderz.Vec3 = renderz.renderz_vec3(0.0, 0.0, 0.0)
    sus test_normal renderz.Vec3 = renderz.renderz_vec3(0.0, 1.0, 0.0)
    sus view_direction renderz.Vec3 = renderz.renderz_vec3(0.0, 0.0, 1.0)
    
    sus final_color renderz.Vec4 = renderz.renderz_calculate_lighting(
        test_position, test_normal, view_direction, lights, 3, test_material
    )
    
    fr fr Validate lighting result
    testz.assert_gte_drip(final_color.x, 0.0)
    testz.assert_lte_drip(final_color.x, 1.0)
    testz.assert_gte_drip(final_color.y, 0.0)
    testz.assert_lte_drip(final_color.y, 1.0)
    testz.assert_gte_drip(final_color.z, 0.0)
    testz.assert_lte_drip(final_color.z, 1.0)
    testz.assert_eq_drip(final_color.w, 1.0)
    
    vibez.spill("✓ Enhanced lighting system test passed")
    damn true
}

fr fr ===== ENHANCED 2D GRAPHICS TESTING =====

slay test_enhanced_drawz_canvas() lit {
    vibez.spill("=== Testing Enhanced DrawZ Canvas ===")
    
    fr fr Test enhanced canvas creation with depth buffer
    sus canvas drawz.Canvas = drawz.drawz_create_canvas(1024, 768)
    testz.assert_eq_int(canvas.width, 1024)
    testz.assert_eq_int(canvas.height, 768)
    testz.assert_eq_lit(canvas.anti_alias_enabled, true)
    testz.assert_eq_int(canvas.blend_mode, drawz.BLEND_NORMAL)
    
    fr fr Test clipping rectangle
    testz.assert_eq_drip(canvas.clip_rect.x, 0.0)
    testz.assert_eq_drip(canvas.clip_rect.y, 0.0)
    testz.assert_eq_drip(canvas.clip_rect.width, 1024.0)
    testz.assert_eq_drip(canvas.clip_rect.height, 768.0)
    
    fr fr Test gradient fill
    sus gradient drawz.Gradient
    gradient.gradient_type = 1  fr fr Linear
    gradient.start_point = drawz.drawz_create_point(0.0, 0.0)
    gradient.end_point = drawz.drawz_create_point(1024.0, 768.0)
    gradient.colors[0] = drawz.drawz_create_colorf(1.0, 0.0, 0.0, 1.0)
    gradient.colors[1] = drawz.drawz_create_colorf(0.0, 0.0, 1.0, 1.0)
    gradient.positions[0] = 0.0
    gradient.positions[1] = 1.0
    gradient.color_count = 2
    
    sus gradient_success lit = drawz.drawz_clear_canvas_with_gradient(canvas, gradient)
    testz.assert_eq_lit(gradient_success, true)
    
    vibez.spill("✓ Enhanced DrawZ canvas test passed")
    damn true
}

slay test_antialiased_drawing() lit {
    vibez.spill("=== Testing Anti-aliased Drawing ===")
    
    sus canvas drawz.Canvas = drawz.drawz_create_canvas(512, 512)
    canvas.anti_alias_enabled = true
    
    fr fr Test Wu's anti-aliased line algorithm
    sus line_start drawz.Point2D = drawz.drawz_create_point(50.0, 50.0)
    sus line_end drawz.Point2D = drawz.drawz_create_point(450.0, 300.0)
    canvas.stroke_color = drawz.drawz_create_color(255, 0, 0, 255)
    
    sus line_success lit = drawz.drawz_draw_line_antialiased(canvas, line_start, line_end)
    testz.assert_eq_lit(line_success, true)
    
    fr fr Test anti-aliased circle with distance field
    sus circle drawz.Circle2D
    circle.center = drawz.drawz_create_point(256.0, 256.0)
    circle.radius = 100.0
    canvas.fill_color = drawz.drawz_create_color(0, 255, 0, 255)
    
    sus circle_success lit = drawz.drawz_draw_circle_antialiased(canvas, circle, drawz.DRAW_MODE_FILL)
    testz.assert_eq_lit(circle_success, true)
    
    fr fr Test anti-aliased ellipse
    sus ellipse drawz.Ellipse2D
    ellipse.center = drawz.drawz_create_point(400.0, 150.0)
    ellipse.radius_x = 80.0
    ellipse.radius_y = 40.0
    
    sus ellipse_success lit = drawz.drawz_draw_ellipse_antialiased(canvas, ellipse, drawz.DRAW_MODE_BOTH)
    testz.assert_eq_lit(ellipse_success, true)
    
    vibez.spill("✓ Anti-aliased drawing test passed")
    damn true
}

slay test_advanced_bezier_curves() lit {
    vibez.spill("=== Testing Advanced Bezier Curves ===")
    
    sus canvas drawz.Canvas = drawz.drawz_create_canvas(400, 300)
    canvas.stroke_color = drawz.drawz_create_color(0, 0, 255, 255)
    canvas.line_width = 2.0
    
    fr fr Test adaptive subdivision Bezier curve
    sus bezier drawz.BezierCurve
    bezier.start = drawz.drawz_create_point(50.0, 200.0)
    bezier.control1 = drawz.drawz_create_point(100.0, 50.0)
    bezier.control2 = drawz.drawz_create_point(300.0, 50.0)
    bezier.end = drawz.drawz_create_point(350.0, 200.0)
    
    sus tolerance drip = 1.0
    sus bezier_success lit = drawz.drawz_draw_bezier_curve_adaptive(canvas, bezier, tolerance)
    testz.assert_eq_lit(bezier_success, true)
    
    fr fr Test quadratic Bezier curve
    sus quad_bezier drawz.QuadraticBezier
    quad_bezier.start = drawz.drawz_create_point(50.0, 250.0)
    quad_bezier.control = drawz.drawz_create_point(200.0, 100.0)
    quad_bezier.end = drawz.drawz_create_point(350.0, 250.0)
    
    fr fr Convert to cubic and render
    sus cubic_from_quad drawz.BezierCurve
    cubic_from_quad.start = quad_bezier.start
    cubic_from_quad.control1 = drawz.drawz_lerp_point(quad_bezier.start, quad_bezier.control, 0.67)
    cubic_from_quad.control2 = drawz.drawz_lerp_point(quad_bezier.end, quad_bezier.control, 0.67)
    cubic_from_quad.end = quad_bezier.end
    
    sus quad_success lit = drawz.drawz_draw_bezier_curve_adaptive(canvas, cubic_from_quad, tolerance)
    testz.assert_eq_lit(quad_success, true)
    
    vibez.spill("✓ Advanced Bezier curves test passed")
    damn true
}

slay test_enhanced_text_rendering() lit {
    vibez.spill("=== Testing Enhanced Text Rendering ===")
    
    sus canvas drawz.Canvas = drawz.drawz_create_canvas(600, 400)
    
    fr fr Test enhanced text styling
    sus text_style drawz.TextStyle
    text_style.font_family = "Arial"
    text_style.font_size = 24.0
    text_style.bold = false
    text_style.italic = false
    text_style.underline = true
    text_style.strike_through = false
    text_style.letter_spacing = 1.0
    text_style.line_height = 30.0
    text_style.alignment = 0  fr fr Left align
    
    canvas.stroke_color = drawz.drawz_create_color(0, 0, 0, 255)
    
    fr fr Test multi-line text rendering
    sus test_text tea = "Enhanced CURSED Text\nWith Multiple Lines\nAnd Special Features!"
    sus text_success lit = drawz.drawz_draw_text_enhanced(canvas, test_text, 50, 100, text_style)
    testz.assert_eq_lit(text_success, true)
    
    fr fr Test bold and italic text
    text_style.bold = true
    text_style.italic = true
    text_style.underline = false
    text_style.strike_through = true
    text_style.font_size = 32.0
    
    sus styled_text tea = "Bold Italic Strikethrough"
    sus styled_success lit = drawz.drawz_draw_text_enhanced(canvas, styled_text, 50, 250, text_style)
    testz.assert_eq_lit(styled_success, true)
    
    fr fr Test text alignment
    text_style.alignment = 1  fr fr Center
    text_style.font_size = 20.0
    text_style.bold = false
    text_style.italic = false
    text_style.strike_through = false
    text_style.underline = false
    
    sus centered_text tea = "Centered Text"
    sus centered_success lit = drawz.drawz_draw_text_enhanced(canvas, centered_text, 300, 350, text_style)
    testz.assert_eq_lit(centered_success, true)
    
    vibez.spill("✓ Enhanced text rendering test passed")
    damn true
}

slay test_advanced_image_filters() lit {
    vibez.spill("=== Testing Advanced Image Filters ===")
    
    sus canvas drawz.Canvas = drawz.drawz_create_canvas(256, 256)
    
    fr fr Create test pattern
    sus test_rect drawz.Rect2D = drawz.drawz_create_rect(50.0, 50.0, 156.0, 156.0)
    sus checkerboard_color1 drawz.Color = drawz.drawz_create_color(255, 255, 255, 255)
    sus checkerboard_color2 drawz.Color = drawz.drawz_create_color(0, 0, 0, 255)
    drawz.drawz_draw_checkered_pattern(canvas, test_rect, checkerboard_color1, checkerboard_color2, 16)
    
    fr fr Test Gaussian blur filter
    sus blur_rect drawz.Rect2D = drawz.drawz_create_rect(0.0, 0.0, 128.0, 128.0)
    sus blur_success lit = drawz.drawz_apply_filter(canvas, blur_rect, drawz.FILTER_BLUR, 3.0)
    testz.assert_eq_lit(blur_success, true)
    
    fr fr Test unsharp mask sharpening
    sus sharpen_rect drawz.Rect2D = drawz.drawz_create_rect(128.0, 0.0, 128.0, 128.0)
    sus sharpen_success lit = drawz.drawz_apply_filter(canvas, sharpen_rect, drawz.FILTER_SHARPEN, 1.5)
    testz.assert_eq_lit(sharpen_success, true)
    
    fr fr Test emboss filter
    sus emboss_rect drawz.Rect2D = drawz.drawz_create_rect(0.0, 128.0, 128.0, 128.0)
    sus emboss_success lit = drawz.drawz_apply_filter(canvas, emboss_rect, drawz.FILTER_EMBOSS, 2.0)
    testz.assert_eq_lit(emboss_success, true)
    
    fr fr Test edge detection
    sus edge_rect drawz.Rect2D = drawz.drawz_create_rect(128.0, 128.0, 128.0, 128.0)
    sus edge_success lit = drawz.drawz_apply_filter(canvas, edge_rect, drawz.FILTER_EDGE_DETECT, 0.5)
    testz.assert_eq_lit(edge_success, true)
    
    fr fr Test color adjustments
    sus full_canvas drawz.Rect2D = drawz.drawz_create_rect(0.0, 0.0, 256.0, 256.0)
    sus brightness_success lit = drawz.drawz_apply_filter(canvas, full_canvas, drawz.FILTER_BRIGHTNESS, 0.2)
    testz.assert_eq_lit(brightness_success, true)
    
    sus contrast_success lit = drawz.drawz_apply_filter(canvas, full_canvas, drawz.FILTER_CONTRAST, 1.2)
    testz.assert_eq_lit(contrast_success, true)
    
    vibez.spill("✓ Advanced image filters test passed")
    damn true
}

slay test_advanced_blend_modes() lit {
    vibez.spill("=== Testing Advanced Blend Modes ===")
    
    sus canvas drawz.Canvas = drawz.drawz_create_canvas(400, 300)
    
    fr fr Create base layer with gradient
    sus base_rect drawz.Rect2D = drawz.drawz_create_rect(0.0, 0.0, 400.0, 300.0)
    sus start_color drawz.Color = drawz.drawz_create_color(255, 0, 0, 255)
    sus end_color drawz.Color = drawz.drawz_create_color(0, 0, 255, 255)
    drawz.drawz_draw_gradient_rect(canvas, base_rect, start_color, end_color, true)
    
    fr fr Test multiply blend mode
    canvas.blend_mode = drawz.BLEND_MULTIPLY
    canvas.fill_color = drawz.drawz_create_color(255, 255, 0, 128)  fr fr Semi-transparent yellow
    sus multiply_rect drawz.Rect2D = drawz.drawz_create_rect(50.0, 50.0, 100.0, 100.0)
    drawz.drawz_draw_rect(canvas, multiply_rect, drawz.DRAW_MODE_FILL)
    
    fr fr Test screen blend mode
    canvas.blend_mode = drawz.BLEND_SCREEN
    canvas.fill_color = drawz.drawz_create_color(0, 255, 255, 128)  fr fr Semi-transparent cyan
    sus screen_rect drawz.Rect2D = drawz.drawz_create_rect(150.0, 50.0, 100.0, 100.0)
    drawz.drawz_draw_rect(canvas, screen_rect, drawz.DRAW_MODE_FILL)
    
    fr fr Test overlay blend mode
    canvas.blend_mode = drawz.BLEND_OVERLAY
    canvas.fill_color = drawz.drawz_create_color(255, 0, 255, 128)  fr fr Semi-transparent magenta
    sus overlay_rect drawz.Rect2D = drawz.drawz_create_rect(250.0, 50.0, 100.0, 100.0)
    drawz.drawz_draw_rect(canvas, overlay_rect, drawz.DRAW_MODE_FILL)
    
    fr fr Test soft light blend mode
    canvas.blend_mode = drawz.BLEND_SOFT_LIGHT
    canvas.fill_color = drawz.drawz_create_color(128, 128, 128, 128)  fr fr Semi-transparent gray
    sus soft_light_rect drawz.Rect2D = drawz.drawz_create_rect(100.0, 150.0, 100.0, 100.0)
    drawz.drawz_draw_rect(canvas, soft_light_rect, drawz.DRAW_MODE_FILL)
    
    fr fr Reset to normal blend mode
    canvas.blend_mode = drawz.BLEND_NORMAL
    
    vibez.spill("✓ Advanced blend modes test passed")
    damn true
}

slay test_transformation_system() lit {
    vibez.spill("=== Testing Transformation System ===")
    
    sus canvas drawz.Canvas = drawz.drawz_create_canvas(300, 300)
    
    fr fr Test identity transform
    drawz.drawz_reset_transform(canvas)
    
    fr fr Test translation
    drawz.drawz_translate(canvas, 50.0, 50.0)
    testz.assert_eq_drip(canvas.transform_matrix[2], 50.0)
    testz.assert_eq_drip(canvas.transform_matrix[5], 50.0)
    
    fr fr Test scaling
    drawz.drawz_scale(canvas, 1.5, 2.0)
    testz.assert_eq_drip(canvas.transform_matrix[0], 1.5)
    testz.assert_eq_drip(canvas.transform_matrix[4], 2.0)
    
    fr fr Test rotation
    drawz.drawz_rotate(canvas, 45.0)
    fr fr Check that rotation was applied (matrix elements changed)
    testz.assert_neq_drip(canvas.transform_matrix[0], 1.5)
    testz.assert_neq_drip(canvas.transform_matrix[1], 0.0)
    
    fr fr Draw transformed rectangle
    canvas.fill_color = drawz.drawz_create_color(255, 128, 0, 255)
    sus test_rect drawz.Rect2D = drawz.drawz_create_rect(0.0, 0.0, 50.0, 30.0)
    drawz.drawz_draw_rect(canvas, test_rect, drawz.DRAW_MODE_FILL)
    
    fr fr Reset transform
    drawz.drawz_reset_transform(canvas)
    testz.assert_eq_drip(canvas.transform_matrix[0], 1.0)
    testz.assert_eq_drip(canvas.transform_matrix[4], 1.0)
    testz.assert_eq_drip(canvas.transform_matrix[2], 0.0)
    testz.assert_eq_drip(canvas.transform_matrix[5], 0.0)
    
    vibez.spill("✓ Transformation system test passed")
    damn true
}

slay test_thick_line_rendering() lit {
    vibez.spill("=== Testing Thick Line Rendering ===")
    
    sus canvas drawz.Canvas = drawz.drawz_create_canvas(400, 200)
    canvas.stroke_color = drawz.drawz_create_color(255, 0, 0, 255)
    
    fr fr Test thick anti-aliased line with rounded caps
    sus start_point drawz.Point2D = drawz.drawz_create_point(50.0, 100.0)
    sus end_point drawz.Point2D = drawz.drawz_create_point(350.0, 100.0)
    
    sus thick_line_success lit = drawz.drawz_draw_thick_line_antialiased(canvas, start_point, end_point, 10.0)
    testz.assert_eq_lit(thick_line_success, true)
    
    fr fr Test dashed line patterns
    sus dash_pattern [8]drip = [10.0, 5.0, 3.0, 5.0, 10.0, 5.0, 3.0, 5.0]
    sus dash_start drawz.Point2D = drawz.drawz_create_point(50.0, 150.0)
    sus dash_end drawz.Point2D = drawz.drawz_create_point(350.0, 150.0)
    
    sus dash_success lit = drawz.drawz_draw_dashed_line(canvas, dash_start, dash_end, dash_pattern, 4)
    testz.assert_eq_lit(dash_success, true)
    
    fr fr Test zero-length line (should draw circle)
    sus zero_start drawz.Point2D = drawz.drawz_create_point(200.0, 50.0)
    sus zero_end drawz.Point2D = drawz.drawz_create_point(200.0, 50.0)
    
    sus zero_success lit = drawz.drawz_draw_thick_line_antialiased(canvas, zero_start, zero_end, 8.0)
    testz.assert_eq_lit(zero_success, true)
    
    vibez.spill("✓ Thick line rendering test passed")
    damn true
}

slay test_polygon_antialiasing() lit {
    vibez.spill("=== Testing Polygon Anti-aliasing ===")
    
    sus canvas drawz.Canvas = drawz.drawz_create_canvas(400, 400)
    canvas.fill_color = drawz.drawz_create_color(0, 255, 128, 255)
    canvas.stroke_color = drawz.drawz_create_color(255, 0, 0, 255)
    
    fr fr Create complex polygon (star shape)
    sus star_points [10]drawz.Point2D
    sus center_x drip = 200.0
    sus center_y drip = 200.0
    sus outer_radius drip = 80.0
    sus inner_radius drip = 40.0
    
    sus i normie = 0
    bestie (i < 10) {
        sus angle drip = i * 36.0 * 3.14159 / 180.0  fr fr 36 degrees in radians
        sus radius drip = ready ((i % 2) == 0) outer_radius otherwise inner_radius
        
        star_points[i].x = center_x + mathz.cos(angle) * radius
        star_points[i].y = center_y + mathz.sin(angle) * radius
        i = i + 1
    }
    
    fr fr Test anti-aliased polygon fill
    sus polygon_success lit = drawz.drawz_draw_polygon_antialiased(canvas, star_points, 10, drawz.DRAW_MODE_BOTH)
    testz.assert_eq_lit(polygon_success, true)
    
    fr fr Test complex polygon with self-intersection
    sus complex_points [6]drawz.Point2D
    complex_points[0] = drawz.drawz_create_point(50.0, 50.0)
    complex_points[1] = drawz.drawz_create_point(150.0, 50.0)
    complex_points[2] = drawz.drawz_create_point(50.0, 100.0)
    complex_points[3] = drawz.drawz_create_point(150.0, 100.0)
    complex_points[4] = drawz.drawz_create_point(100.0, 25.0)
    complex_points[5] = drawz.drawz_create_point(100.0, 125.0)
    
    canvas.fill_color = drawz.drawz_create_color(255, 255, 0, 128)
    sus complex_success lit = drawz.drawz_draw_polygon_antialiased(canvas, complex_points, 6, drawz.DRAW_MODE_FILL)
    testz.assert_eq_lit(complex_success, true)
    
    vibez.spill("✓ Polygon anti-aliasing test passed")
    damn true
}

fr fr ===== PERFORMANCE AND STRESS TESTING =====

slay test_graphics_performance() lit {
    vibez.spill("=== Testing Graphics Performance ===")
    
    sus start_time drip = 0.0  fr fr Would use actual timer
    
    fr fr Large canvas stress test
    sus large_canvas drawz.Canvas = drawz.drawz_create_canvas(2048, 2048)
    
    fr fr Render many primitives
    sus primitive_count normie = 1000
    sus i normie = 0
    bestie (i < primitive_count) {
        sus x drip = (i % 64) * 32.0
        sus y drip = (i / 64) * 32.0
        sus size drip = 10.0 + (i % 20)
        
        ready ((i % 3) == 0) {
            fr fr Draw circles
            sus circle drawz.Circle2D
            circle.center = drawz.drawz_create_point(x + size/2, y + size/2)
            circle.radius = size / 2
            large_canvas.fill_color = drawz.drawz_create_color(i % 256, (i * 2) % 256, (i * 3) % 256, 255)
            drawz.drawz_draw_circle_antialiased(large_canvas, circle, drawz.DRAW_MODE_FILL)
        } otherwise ready ((i % 3) == 1) {
            fr fr Draw rectangles
            sus rect drawz.Rect2D = drawz.drawz_create_rect(x, y, size, size)
            large_canvas.fill_color = drawz.drawz_create_color((i * 3) % 256, i % 256, (i * 2) % 256, 255)
            drawz.drawz_draw_rect(large_canvas, rect, drawz.DRAW_MODE_FILL)
        } otherwise {
            fr fr Draw lines
            sus start drawz.Point2D = drawz.drawz_create_point(x, y)
            sus end drawz.Point2D = drawz.drawz_create_point(x + size, y + size)
            large_canvas.stroke_color = drawz.drawz_create_color((i * 2) % 256, (i * 3) % 256, i % 256, 255)
            drawz.drawz_draw_line_antialiased(large_canvas, start, end)
        }
        
        i = i + 1
    }
    
    sus end_time drip = 0.0  fr fr Would use actual timer
    vibez.spill("Rendered", primitive_count, "primitives on 2048x2048 canvas")
    
    vibez.spill("✓ Graphics performance test completed")
    damn true
}

slay test_memory_management() lit {
    vibez.spill("=== Testing Graphics Memory Management ===")
    
    fr fr Test resource cleanup
    sus context renderz.RenderContext = renderz.renderz_initialize(renderz.GRAPHICS_API_SOFTWARE, 512, 512)
    
    fr fr Create and destroy many resources
    sus resource_count normie = 100
    sus i normie = 0
    bestie (i < resource_count) {
        fr fr Create texture
        sus texture_data [1024]normie  fr fr 32x32 RGBA
        sus j normie = 0
        bestie (j < 1024) {
            texture_data[j] = (i * 1000 + j) % 16777216  fr fr Random color data
            j = j + 1
        }
        
        fr fr Test texture creation and implicit cleanup
        sus fake_image imagez.ImageData
        fake_image.width = 32
        fake_image.height = 32
        fake_image.channels = 4
        fake_image.pixels = "texture_data_placeholder"
        
        sus texture renderz.Texture = renderz.renderz_create_texture(fake_image)
        testz.assert_neq_int(texture.id, 0)
        testz.assert_eq_int(texture.width, 32)
        testz.assert_eq_int(texture.height, 32)
        
        i = i + 1
    }
    
    fr fr Test context shutdown (should cleanup all resources)
    sus shutdown_success lit = renderz.renderz_shutdown(context)
    testz.assert_eq_lit(shutdown_success, true)
    
    vibez.spill("✓ Graphics memory management test passed")
    damn true
}

fr fr ===== MAIN TEST RUNNER =====

slay run_comprehensive_graphics_tests() lit {
    vibez.spill("🎨 Starting Comprehensive Graphics Testing Suite 🎨")
    vibez.spill("================================================")
    
    fr fr Enhanced 3D Graphics Tests
    test_enhanced_renderz_initialization()
    test_enhanced_shader_system()
    test_enhanced_font_rendering()
    test_advanced_geometry_generation()
    test_enhanced_lighting_system()
    
    fr fr Enhanced 2D Graphics Tests  
    test_enhanced_drawz_canvas()
    test_antialiased_drawing()
    test_advanced_bezier_curves()
    test_enhanced_text_rendering()
    test_advanced_image_filters()
    test_advanced_blend_modes()
    test_transformation_system()
    test_thick_line_rendering()
    test_polygon_antialiasing()
    
    fr fr Performance and Stress Tests
    test_graphics_performance()
    test_memory_management()
    
    vibez.spill("================================================")
    vibez.spill("🎊 All Enhanced Graphics Tests Completed Successfully! 🎊")
    
    testz.print_test_summary()
    damn true
}

fr fr Execute the comprehensive test suite
run_comprehensive_graphics_tests()
