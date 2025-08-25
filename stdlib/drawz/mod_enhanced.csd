fr fr CURSED DrawZ Module - Enhanced 2D Graphics Primitives and Drawing
fr fr Professional 2D graphics operations with advanced algorithms
fr fr Pure CURSED implementation with optimized rendering

yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "memoryz"

fr fr ===== ENHANCED COLOR CONSTANTS =====

facts COLOR_TRANSPARENT normie = 0x00000000
facts COLOR_BLACK normie = 0xFF000000
facts COLOR_WHITE normie = 0xFFFFFFFF
facts COLOR_RED normie = 0xFFFF0000
facts COLOR_GREEN normie = 0xFF00FF00
facts COLOR_BLUE normie = 0xFF0000FF
facts COLOR_YELLOW normie = 0xFFFFFF00
facts COLOR_MAGENTA normie = 0xFFFF00FF
facts COLOR_CYAN normie = 0xFF00FFFF
facts COLOR_ORANGE normie = 0xFFFFA500
facts COLOR_PURPLE normie = 0xFF800080
facts COLOR_GRAY normie = 0xFF808080
facts COLOR_BROWN normie = 0xFFA52A2A
facts COLOR_PINK normie = 0xFFFFC0CB
facts COLOR_LIME normie = 0xFF00FF00
facts COLOR_NAVY normie = 0xFF000080
facts COLOR_TEAL normie = 0xFF008080
facts COLOR_SILVER normie = 0xFFC0C0C0
facts COLOR_GOLD normie = 0xFFFFD700
facts COLOR_CRIMSON normie = 0xFFDC143C

fr fr ===== DRAWING MODES =====

facts DRAW_MODE_FILL normie = 0
facts DRAW_MODE_STROKE normie = 1
facts DRAW_MODE_BOTH normie = 2

fr fr ===== LINE STYLES =====

facts LINE_SOLID normie = 0
facts LINE_DASHED normie = 1
facts LINE_DOTTED normie = 2
facts LINE_DASH_DOT normie = 3
facts LINE_CUSTOM normie = 4

fr fr ===== BLEND MODES =====

facts BLEND_NORMAL normie = 0
facts BLEND_MULTIPLY normie = 1
facts BLEND_SCREEN normie = 2
facts BLEND_OVERLAY normie = 3
facts BLEND_SOFT_LIGHT normie = 4
facts BLEND_HARD_LIGHT normie = 5
facts BLEND_COLOR_DODGE normie = 6
facts BLEND_COLOR_BURN normie = 7
facts BLEND_DARKEN normie = 8
facts BLEND_LIGHTEN normie = 9
facts BLEND_DIFFERENCE normie = 10
facts BLEND_EXCLUSION normie = 11

fr fr ===== FILTER TYPES =====

facts FILTER_NONE normie = 0
facts FILTER_BLUR normie = 1
facts FILTER_SHARPEN normie = 2
facts FILTER_EMBOSS normie = 3
facts FILTER_EDGE_DETECT normie = 4
facts FILTER_NOISE normie = 5
facts FILTER_SEPIA normie = 6
facts FILTER_GRAYSCALE normie = 7
facts FILTER_INVERT normie = 8
facts FILTER_BRIGHTNESS normie = 9
facts FILTER_CONTRAST normie = 10

fr fr ===== ENHANCED 2D GRAPHICS STRUCTURES =====

be_like Point2D = struct {
    x drip,
    y drip
}

be_like Rect2D = struct {
    x drip,
    y drip,
    width drip,
    height drip
}

be_like Circle2D = struct {
    center Point2D,
    radius drip
}

be_like Ellipse2D = struct {
    center Point2D,
    radius_x drip,
    radius_y drip
}

be_like Color = struct {
    r normie,  fr fr 0-255
    g normie,  fr fr 0-255 
    b normie,  fr fr 0-255
    a normie   fr fr 0-255 alpha
}

be_like ColorF = struct {
    r drip,    fr fr 0.0-1.0
    g drip,    fr fr 0.0-1.0
    b drip,    fr fr 0.0-1.0
    a drip     fr fr 0.0-1.0 alpha
}

be_like Canvas = struct {
    width normie,
    height normie,
    pixels [4194304]normie,  fr fr RGBA pixels (2048x2048 max)
    depth_buffer [4194304]drip,  fr fr Z-buffer for 3D effects
    stroke_color Color,
    fill_color Color,
    line_width drip,
    line_style normie,
    blend_mode normie,
    clip_rect Rect2D,
    transform_matrix [9]drip,  fr fr 3x3 2D transformation matrix
    anti_alias_enabled lit,
    mip_map_enabled lit
}

be_like BezierCurve = struct {
    start Point2D,
    control1 Point2D,
    control2 Point2D,
    end Point2D
}

be_like QuadraticBezier = struct {
    start Point2D,
    control Point2D,
    end Point2D
}

be_like Path2D = struct {
    points [1000]Point2D,
    commands [1000]normie,  fr fr Move, Line, Curve, etc.
    point_count normie,
    command_count normie,
    closed lit
}

be_like Gradient = struct {
    gradient_type normie,  fr fr Linear, Radial, Conic
    start_point Point2D,
    end_point Point2D,
    colors [16]ColorF,
    positions [16]drip,    fr fr 0.0 to 1.0
    color_count normie
}

be_like Pattern = struct {
    pattern_type normie,   fr fr Checkerboard, Stripes, etc.
    color1 Color,
    color2 Color,
    size normie,
    rotation drip
}

be_like FontMetrics = struct {
    ascent drip,
    descent drip,
    line_gap drip,
    x_height drip,
    cap_height drip
}

be_like TextStyle = struct {
    font_family tea,
    font_size drip,
    bold lit,
    italic lit,
    underline lit,
    strike_through lit,
    letter_spacing drip,
    line_height drip,
    alignment normie  fr fr Left, Center, Right, Justify
}

fr fr ===== ENHANCED CANVAS OPERATIONS =====

slay drawz_create_canvas(width normie, height normie) Canvas {
    sus canvas Canvas
    canvas.width = width
    canvas.height = height
    
    fr fr Initialize with transparent pixels
    sus i normie = 0
    bestie (i < width * height) {
        canvas.pixels[i] = COLOR_TRANSPARENT
        canvas.depth_buffer[i] = 1.0  fr fr Far plane
        i = i + 1
    }
    
    fr fr Set default drawing properties
    canvas.stroke_color = drawz_create_color(0, 0, 0, 255)
    canvas.fill_color = drawz_create_color(255, 255, 255, 255)
    canvas.line_width = 1.0
    canvas.line_style = LINE_SOLID
    canvas.blend_mode = BLEND_NORMAL
    canvas.clip_rect = drawz_create_rect(0.0, 0.0, width, height)
    canvas.anti_alias_enabled = true
    canvas.mip_map_enabled = false
    
    fr fr Initialize identity transform matrix
    drawz_reset_transform(canvas)
    
    vibez.spill("Enhanced canvas created:", width, "x", height, "with", width * height, "pixels")
    damn canvas
}

slay drawz_clear_canvas(canvas Canvas, color Color) lit {
    sus rgba normie = drawz_color_to_rgba(color)
    
    sus i normie = 0
    bestie (i < canvas.width * canvas.height) {
        canvas.pixels[i] = rgba
        canvas.depth_buffer[i] = 1.0
        i = i + 1
    }
    
    damn true
}

slay drawz_clear_canvas_with_gradient(canvas Canvas, gradient Gradient) lit {
    sus y normie = 0
    bestie (y < canvas.height) {
        sus x normie = 0
        bestie (x < canvas.width) {
            sus gradient_color ColorF = drawz_sample_gradient(gradient, x, y, canvas.width, canvas.height)
            sus color Color = drawz_colorf_to_color(gradient_color)
            drawz_set_pixel_safe(canvas, x, y, color)
            x = x + 1
        }
        y = y + 1
    }
    
    damn true
}

slay drawz_set_pixel_safe(canvas Canvas, x normie, y normie, color Color) lit {
    ready (x >= canvas.clip_rect.x && x < canvas.clip_rect.x + canvas.clip_rect.width &&
           y >= canvas.clip_rect.y && y < canvas.clip_rect.y + canvas.clip_rect.height &&
           x >= 0 && x < canvas.width && y >= 0 && y < canvas.height) {
        
        sus index normie = y * canvas.width + x
        sus new_rgba normie = drawz_color_to_rgba(color)
        
        ready (canvas.blend_mode == BLEND_NORMAL) {
            canvas.pixels[index] = new_rgba
        } otherwise {
            sus existing Color = drawz_rgba_to_color(canvas.pixels[index])
            sus blended Color = drawz_blend_colors_advanced(existing, color, canvas.blend_mode)
            canvas.pixels[index] = drawz_color_to_rgba(blended)
        }
        
        damn true
    }
    damn false
}

slay drawz_set_pixel_with_depth(canvas Canvas, x normie, y normie, color Color, depth drip) lit {
    ready (drawz_is_pixel_in_bounds(canvas, x, y)) {
        sus index normie = y * canvas.width + x
        
        fr fr Depth test
        ready (depth < canvas.depth_buffer[index]) {
            canvas.depth_buffer[index] = depth
            drawz_set_pixel_safe(canvas, x, y, color)
            damn true
        }
    }
    damn false
}

fr fr ===== ENHANCED LINE DRAWING WITH ANTI-ALIASING =====

slay drawz_draw_line_antialiased(canvas Canvas, start Point2D, end Point2D) lit {
    fr fr Wu's anti-aliased line algorithm
    sus x0 drip = start.x
    sus y0 drip = start.y
    sus x1 drip = end.x
    sus y1 drip = end.y
    
    sus steep lit = mathz.abs(y1 - y0) > mathz.abs(x1 - x0)
    
    ready (steep) {
        fr fr Swap x and y coordinates
        sus temp drip = x0
        x0 = y0
        y0 = temp
        temp = x1
        x1 = y1
        y1 = temp
    }
    
    ready (x0 > x1) {
        fr fr Swap start and end
        sus temp drip = x0
        x0 = x1
        x1 = temp
        temp = y0
        y0 = y1
        y1 = temp
    }
    
    sus dx drip = x1 - x0
    sus dy drip = y1 - y0
    sus gradient drip = ready (dx == 0.0) 1.0 otherwise dy / dx
    
    fr fr Handle first endpoint
    sus xend normie = mathz.round(x0)
    sus yend drip = y0 + gradient * (xend - x0)
    sus xgap drip = 1.0 - (x0 + 0.5 - mathz.floor(x0 + 0.5))
    sus xpxl1 normie = xend
    sus ypxl1 normie = mathz.floor(yend)
    
    ready (steep) {
        drawz_plot_pixel_alpha(canvas, ypxl1, xpxl1, canvas.stroke_color, (1.0 - (yend - mathz.floor(yend))) * xgap)
        drawz_plot_pixel_alpha(canvas, ypxl1 + 1, xpxl1, canvas.stroke_color, (yend - mathz.floor(yend)) * xgap)
    } otherwise {
        drawz_plot_pixel_alpha(canvas, xpxl1, ypxl1, canvas.stroke_color, (1.0 - (yend - mathz.floor(yend))) * xgap)
        drawz_plot_pixel_alpha(canvas, xpxl1, ypxl1 + 1, canvas.stroke_color, (yend - mathz.floor(yend)) * xgap)
    }
    
    sus intery drip = yend + gradient
    
    fr fr Handle second endpoint
    xend = mathz.round(x1)
    yend = y1 + gradient * (xend - x1)
    xgap = x1 + 0.5 - mathz.floor(x1 + 0.5)
    sus xpxl2 normie = xend
    sus ypxl2 normie = mathz.floor(yend)
    
    ready (steep) {
        drawz_plot_pixel_alpha(canvas, ypxl2, xpxl2, canvas.stroke_color, (1.0 - (yend - mathz.floor(yend))) * xgap)
        drawz_plot_pixel_alpha(canvas, ypxl2 + 1, xpxl2, canvas.stroke_color, (yend - mathz.floor(yend)) * xgap)
    } otherwise {
        drawz_plot_pixel_alpha(canvas, xpxl2, ypxl2, canvas.stroke_color, (1.0 - (yend - mathz.floor(yend))) * xgap)
        drawz_plot_pixel_alpha(canvas, xpxl2, ypxl2 + 1, canvas.stroke_color, (yend - mathz.floor(yend)) * xgap)
    }
    
    fr fr Main loop
    ready (steep) {
        sus x normie = xpxl1 + 1
        bestie (x < xpxl2) {
            drawz_plot_pixel_alpha(canvas, mathz.floor(intery), x, canvas.stroke_color, 1.0 - (intery - mathz.floor(intery)))
            drawz_plot_pixel_alpha(canvas, mathz.floor(intery) + 1, x, canvas.stroke_color, intery - mathz.floor(intery))
            intery = intery + gradient
            x = x + 1
        }
    } otherwise {
        sus x normie = xpxl1 + 1
        bestie (x < xpxl2) {
            drawz_plot_pixel_alpha(canvas, x, mathz.floor(intery), canvas.stroke_color, 1.0 - (intery - mathz.floor(intery)))
            drawz_plot_pixel_alpha(canvas, x, mathz.floor(intery) + 1, canvas.stroke_color, intery - mathz.floor(intery))
            intery = intery + gradient
            x = x + 1
        }
    }
    
    damn true
}

slay drawz_draw_thick_line_antialiased(canvas Canvas, start Point2D, end Point2D, thickness drip) lit {
    fr fr Anti-aliased thick line using capsule approach
    sus dx drip = end.x - start.x
    sus dy drip = end.y - start.y
    sus length drip = mathz.sqrt(dx * dx + dy * dy)
    
    ready (length < 0.001) {
        fr fr Draw circle for zero-length line
        sus circle Circle2D = {center: start, radius: thickness / 2.0}
        damn drawz_draw_circle_antialiased(canvas, circle, DRAW_MODE_FILL)
    }
    
    fr fr Calculate perpendicular vector
    sus perp_x drip = -dy / length * thickness / 2.0
    sus perp_y drip = dx / length * thickness / 2.0
    
    fr fr Create polygon for thick line
    sus points [4]Point2D
    points[0] = drawz_create_point(start.x + perp_x, start.y + perp_y)
    points[1] = drawz_create_point(start.x - perp_x, start.y - perp_y)
    points[2] = drawz_create_point(end.x - perp_x, end.y - perp_y)
    points[3] = drawz_create_point(end.x + perp_x, end.y + perp_y)
    
    drawz_draw_polygon_antialiased(canvas, points, 4, DRAW_MODE_FILL)
    
    fr fr Draw rounded end caps
    sus start_circle Circle2D = {center: start, radius: thickness / 2.0}
    sus end_circle Circle2D = {center: end, radius: thickness / 2.0}
    drawz_draw_circle_antialiased(canvas, start_circle, DRAW_MODE_FILL)
    drawz_draw_circle_antialiased(canvas, end_circle, DRAW_MODE_FILL)
    
    damn true
}

slay drawz_draw_dashed_line(canvas Canvas, start Point2D, end Point2D, dash_pattern [8]drip, pattern_length normie) lit {
    sus total_length drip = mathz.sqrt((end.x - start.x) * (end.x - start.x) + (end.y - start.y) * (end.y - start.y))
    sus dx drip = (end.x - start.x) / total_length
    sus dy drip = (end.y - start.y) / total_length
    
    sus current_distance drip = 0.0
    sus pattern_index normie = 0
    sus drawing lit = true
    
    bestie (current_distance < total_length) {
        sus dash_length drip = dash_pattern[pattern_index % pattern_length]
        sus next_distance drip = mathz.min(current_distance + dash_length, total_length)
        
        ready (drawing) {
            sus segment_start Point2D = drawz_create_point(
                start.x + dx * current_distance,
                start.y + dy * current_distance
            )
            sus segment_end Point2D = drawz_create_point(
                start.x + dx * next_distance,
                start.y + dy * next_distance
            )
            
            ready (canvas.anti_alias_enabled) {
                drawz_draw_line_antialiased(canvas, segment_start, segment_end)
            } otherwise {
                drawz_draw_line(canvas, segment_start, segment_end)
            }
        }
        
        current_distance = next_distance
        pattern_index = pattern_index + 1
        drawing = !drawing
    }
    
    damn true
}

fr fr ===== ENHANCED CIRCLE AND ELLIPSE DRAWING =====

slay drawz_draw_circle_antialiased(canvas Canvas, circle Circle2D, mode normie) lit {
    fr fr Anti-aliased circle using distance field approach
    sus cx drip = circle.center.x
    sus cy drip = circle.center.y
    sus radius drip = circle.radius
    
    fr fr Calculate bounding box
    sus min_x normie = mathz.max(0, mathz.floor(cx - radius - 1.0))
    sus max_x normie = mathz.min(canvas.width - 1, mathz.ceil(cx + radius + 1.0))
    sus min_y normie = mathz.max(0, mathz.floor(cy - radius - 1.0))
    sus max_y normie = mathz.min(canvas.height - 1, mathz.ceil(cy + radius + 1.0))
    
    sus y normie = min_y
    bestie (y <= max_y) {
        sus x normie = min_x
        bestie (x <= max_x) {
            sus dx drip = x - cx
            sus dy drip = y - cy
            sus distance drip = mathz.sqrt(dx * dx + dy * dy)
            
            ready (mode == DRAW_MODE_FILL || mode == DRAW_MODE_BOTH) {
                ready (distance <= radius) {
                    sus alpha drip = ready (distance > radius - 1.0) 1.0 - (distance - (radius - 1.0)) otherwise 1.0
                    drawz_plot_pixel_alpha(canvas, x, y, canvas.fill_color, alpha)
                }
            }
            
            ready (mode == DRAW_MODE_STROKE || mode == DRAW_MODE_BOTH) {
                sus edge_distance drip = mathz.abs(distance - radius)
                ready (edge_distance < canvas.line_width / 2.0 + 1.0) {
                    sus alpha drip = 1.0 - mathz.clamp(edge_distance - canvas.line_width / 2.0, 0.0, 1.0)
                    drawz_plot_pixel_alpha(canvas, x, y, canvas.stroke_color, alpha)
                }
            }
            
            x = x + 1
        }
        y = y + 1
    }
    
    damn true
}

slay drawz_draw_ellipse_antialiased(canvas Canvas, ellipse Ellipse2D, mode normie) lit {
    fr fr Anti-aliased ellipse rendering
    sus cx drip = ellipse.center.x
    sus cy drip = ellipse.center.y
    sus rx drip = ellipse.radius_x
    sus ry drip = ellipse.radius_y
    
    sus max_radius drip = mathz.max(rx, ry)
    sus min_x normie = mathz.max(0, mathz.floor(cx - max_radius - 1.0))
    sus max_x normie = mathz.min(canvas.width - 1, mathz.ceil(cx + max_radius + 1.0))
    sus min_y normie = mathz.max(0, mathz.floor(cy - max_radius - 1.0))
    sus max_y normie = mathz.min(canvas.height - 1, mathz.ceil(cy + max_radius + 1.0))
    
    sus y normie = min_y
    bestie (y <= max_y) {
        sus x normie = min_x
        bestie (x <= max_x) {
            sus dx drip = x - cx
            sus dy drip = y - cy
            sus ellipse_value drip = (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry)
            
            ready (mode == DRAW_MODE_FILL || mode == DRAW_MODE_BOTH) {
                ready (ellipse_value <= 1.0) {
                    sus distance_to_edge drip = 1.0 - ellipse_value
                    sus alpha drip = mathz.clamp(distance_to_edge, 0.0, 1.0)
                    drawz_plot_pixel_alpha(canvas, x, y, canvas.fill_color, alpha)
                }
            }
            
            ready (mode == DRAW_MODE_STROKE || mode == DRAW_MODE_BOTH) {
                sus edge_distance drip = mathz.abs(1.0 - ellipse_value)
                ready (edge_distance < 0.1) {
                    sus alpha drip = 1.0 - edge_distance * 10.0
                    drawz_plot_pixel_alpha(canvas, x, y, canvas.stroke_color, alpha)
                }
            }
            
            x = x + 1
        }
        y = y + 1
    }
    
    damn true
}

fr fr ===== ENHANCED BEZIER CURVE RENDERING =====

slay drawz_draw_bezier_curve_adaptive(canvas Canvas, curve BezierCurve, tolerance drip) lit {
    fr fr Adaptive subdivision Bezier curve rendering for smooth curves
    drawz_subdivide_bezier_curve(canvas, curve, tolerance, 0)
    damn true
}

slay drawz_subdivide_bezier_curve(canvas Canvas, curve BezierCurve, tolerance drip, depth normie) lit {
    ready (depth > 10) {  fr fr Prevent infinite recursion
        damn false
    }
    
    fr fr Calculate curve flatness
    sus dx1 drip = 3.0 * curve.control1.x - 2.0 * curve.start.x - curve.end.x
    sus dy1 drip = 3.0 * curve.control1.y - 2.0 * curve.start.y - curve.end.y
    sus dx2 drip = 3.0 * curve.control2.x - curve.start.x - 2.0 * curve.end.x
    sus dy2 drip = 3.0 * curve.control2.y - curve.start.y - 2.0 * curve.end.y
    
    sus flatness drip = mathz.max(dx1 * dx1 + dy1 * dy1, dx2 * dx2 + dy2 * dy2)
    
    ready (flatness < tolerance * tolerance) {
        fr fr Curve is flat enough, draw line segment
        ready (canvas.anti_alias_enabled) {
            drawz_draw_line_antialiased(canvas, curve.start, curve.end)
        } otherwise {
            drawz_draw_line(canvas, curve.start, curve.end)
        }
        damn true
    }
    
    fr fr Subdivide curve
    sus left_curve BezierCurve
    sus right_curve BezierCurve
    drawz_split_bezier_curve(curve, left_curve, right_curve)
    
    drawz_subdivide_bezier_curve(canvas, left_curve, tolerance, depth + 1)
    drawz_subdivide_bezier_curve(canvas, right_curve, tolerance, depth + 1)
    damn true
}

slay drawz_split_bezier_curve(original BezierCurve, left BezierCurve, right BezierCurve) lit {
    fr fr De Casteljau's algorithm for curve subdivision
    sus p01 Point2D = drawz_lerp_point(original.start, original.control1, 0.5)
    sus p12 Point2D = drawz_lerp_point(original.control1, original.control2, 0.5)
    sus p23 Point2D = drawz_lerp_point(original.control2, original.end, 0.5)
    
    sus p012 Point2D = drawz_lerp_point(p01, p12, 0.5)
    sus p123 Point2D = drawz_lerp_point(p12, p23, 0.5)
    
    sus p0123 Point2D = drawz_lerp_point(p012, p123, 0.5)
    
    fr fr Left curve
    left.start = original.start
    left.control1 = p01
    left.control2 = p012
    left.end = p0123
    
    fr fr Right curve
    right.start = p0123
    right.control1 = p123
    right.control2 = p23
    right.end = original.end
    
    damn true
}

fr fr ===== ENHANCED POLYGON RENDERING =====

slay drawz_draw_polygon_antialiased(canvas Canvas, points [100]Point2D, num_points normie, mode normie) lit {
    ready (num_points < 3) {
        damn false
    }
    
    ready (mode == DRAW_MODE_STROKE || mode == DRAW_MODE_BOTH) {
        fr fr Draw anti-aliased polygon outline
        sus i normie = 0
        bestie (i < num_points) {
            sus next_i normie = (i + 1) % num_points
            ready (canvas.anti_alias_enabled) {
                drawz_draw_line_antialiased(canvas, points[i], points[next_i])
            } otherwise {
                drawz_draw_line(canvas, points[i], points[next_i])
            }
            i = i + 1
        }
    }
    
    ready (mode == DRAW_MODE_FILL || mode == DRAW_MODE_BOTH) {
        fr fr Anti-aliased polygon fill using edge sampling
        drawz_fill_polygon_antialiased(canvas, points, num_points)
    }
    
    damn true
}

slay drawz_fill_polygon_antialiased(canvas Canvas, points [100]Point2D, num_points normie) lit {
    fr fr Find bounding box
    sus min_x drip = points[0].x
    sus max_x drip = points[0].x
    sus min_y drip = points[0].y
    sus max_y drip = points[0].y
    
    sus i normie = 1
    bestie (i < num_points) {
        ready (points[i].x < min_x) min_x = points[i].x
        ready (points[i].x > max_x) max_x = points[i].x
        ready (points[i].y < min_y) min_y = points[i].y
        ready (points[i].y > max_y) max_y = points[i].y
        i = i + 1
    }
    
    fr fr Super-sampling for anti-aliasing
    sus sample_step drip = 0.25
    sus y_pixel normie = mathz.floor(min_y)
    bestie (y_pixel <= mathz.ceil(max_y)) {
        sus x_pixel normie = mathz.floor(min_x)
        bestie (x_pixel <= mathz.ceil(max_x)) {
            sus coverage drip = 0.0
            sus sample_count normie = 0
            
            fr fr 4x4 super-sampling
            sus sy drip = y_pixel
            bestie (sy < y_pixel + 1.0) {
                sus sx drip = x_pixel
                bestie (sx < x_pixel + 1.0) {
                    ready (drawz_point_in_polygon(drawz_create_point(sx, sy), points, num_points)) {
                        coverage = coverage + 1.0
                    }
                    sample_count = sample_count + 1
                    sx = sx + sample_step
                }
                sy = sy + sample_step
            }
            
            sus alpha drip = coverage / sample_count
            ready (alpha > 0.0) {
                drawz_plot_pixel_alpha(canvas, x_pixel, y_pixel, canvas.fill_color, alpha)
            }
            
            x_pixel = x_pixel + 1
        }
        y_pixel = y_pixel + 1
    }
    
    damn true
}

fr fr ===== ENHANCED TEXT RENDERING SYSTEM =====

slay drawz_draw_text_enhanced(canvas Canvas, text tea, x normie, y normie, style TextStyle) lit {
    yeet "stringz"
    
    fr fr Load font metrics (simplified)
    sus metrics FontMetrics = drawz_get_font_metrics(style.font_family, style.font_size)
    
    sus current_x drip = x
    sus current_y drip = y + metrics.ascent
    sus line_start_x drip = x
    
    fr fr Apply text alignment
    ready (style.alignment == 1 || style.alignment == 2) {  fr fr Center or Right
        sus text_width drip = drawz_measure_text_width(text, style)
        ready (style.alignment == 1) {  fr fr Center
            current_x = x - text_width / 2.0
            line_start_x = current_x
        } otherwise {  fr fr Right
            current_x = x - text_width
            line_start_x = current_x
        }
    }
    
    sus i normie = 0
    bestie (i < stringz.len(text)) {
        sus char normie = stringz.char_at(text, i)
        
        fr fr Handle line breaks
        ready (char == 10) {  fr fr '\n'
            current_x = line_start_x
            current_y = current_y + style.line_height
            i = i + 1
            continue
        }
        
        fr fr Render character
        sus char_width drip = drawz_render_character_enhanced(canvas, char, current_x, current_y, style)
        
        fr fr Apply effects
        ready (style.underline) {
            sus underline_y drip = current_y + metrics.descent * 0.5
            sus underline_start Point2D = drawz_create_point(current_x, underline_y)
            sus underline_end Point2D = drawz_create_point(current_x + char_width, underline_y)
            drawz_draw_line_antialiased(canvas, underline_start, underline_end)
        }
        
        ready (style.strike_through) {
            sus strike_y drip = current_y - metrics.x_height * 0.5
            sus strike_start Point2D = drawz_create_point(current_x, strike_y)
            sus strike_end Point2D = drawz_create_point(current_x + char_width, strike_y)
            drawz_draw_line_antialiased(canvas, strike_start, strike_end)
        }
        
        current_x = current_x + char_width + style.letter_spacing
        i = i + 1
    }
    
    damn true
}

slay drawz_render_character_enhanced(canvas Canvas, character normie, x drip, y drip, style TextStyle) drip {
    fr fr Enhanced character rendering with font hinting and sub-pixel positioning
    
    fr fr For now, render simple character shapes (would use real font rendering in production)
    sus char_width drip = style.font_size * 0.6
    sus char_height drip = style.font_size
    
    ready (style.bold) {
        fr fr Render character with extra thickness
        drawz_render_character_bold(canvas, character, x, y, char_width, char_height)
    } otherwise {
        drawz_render_character_regular(canvas, character, x, y, char_width, char_height)
    }
    
    ready (style.italic) {
        fr fr Apply italic slant effect
        drawz_apply_italic_transform(canvas, x, y, char_width, char_height)
    }
    
    damn char_width
}

fr fr ===== ENHANCED IMAGE PROCESSING FILTERS =====

slay drawz_apply_filter(canvas Canvas, rect Rect2D, filter_type normie, strength drip) lit {
    ready (filter_type == FILTER_BLUR) {
        damn drawz_apply_gaussian_blur(canvas, rect, strength)
    } otherwise ready (filter_type == FILTER_SHARPEN) {
        damn drawz_apply_unsharp_mask(canvas, rect, strength)
    } otherwise ready (filter_type == FILTER_EMBOSS) {
        damn drawz_apply_emboss_filter(canvas, rect, strength)
    } otherwise ready (filter_type == FILTER_EDGE_DETECT) {
        damn drawz_apply_edge_detection(canvas, rect, strength)
    } otherwise ready (filter_type == FILTER_NOISE) {
        damn drawz_add_noise(canvas, rect, strength)
    } otherwise ready (filter_type == FILTER_SEPIA) {
        damn drawz_apply_sepia_tone(canvas, rect, strength)
    } otherwise ready (filter_type == FILTER_GRAYSCALE) {
        damn drawz_convert_to_grayscale(canvas, rect)
    } otherwise ready (filter_type == FILTER_INVERT) {
        damn drawz_invert_colors(canvas, rect)
    } otherwise ready (filter_type == FILTER_BRIGHTNESS) {
        damn drawz_adjust_brightness(canvas, rect, strength)
    } otherwise ready (filter_type == FILTER_CONTRAST) {
        damn drawz_adjust_contrast(canvas, rect, strength)
    }
    
    damn false
}

slay drawz_apply_gaussian_blur(canvas Canvas, rect Rect2D, radius drip) lit {
    fr fr 2D Gaussian blur implementation
    sus kernel_size normie = mathz.min(31, mathz.max(3, radius * 6.0 + 1.0))  fr fr Ensure odd kernel size
    ready (kernel_size % 2 == 0) kernel_size = kernel_size + 1
    
    sus kernel [31]drip
    sus kernel_sum drip = 0.0
    sus sigma drip = radius / 3.0
    
    fr fr Generate Gaussian kernel
    sus i normie = 0
    bestie (i < kernel_size) {
        sus x drip = i - kernel_size / 2
        kernel[i] = mathz.exp(-(x * x) / (2.0 * sigma * sigma))
        kernel_sum = kernel_sum + kernel[i]
        i = i + 1
    }
    
    fr fr Normalize kernel
    i = 0
    bestie (i < kernel_size) {
        kernel[i] = kernel[i] / kernel_sum
        i = i + 1
    }
    
    fr fr Create temporary buffer
    sus temp_buffer [4194304]normie
    
    fr fr Horizontal pass
    sus y normie = rect.y
    bestie (y < rect.y + rect.height) {
        sus x normie = rect.x
        bestie (x < rect.x + rect.width) {
            sus r drip = 0.0
            sus g drip = 0.0
            sus b drip = 0.0
            sus a drip = 0.0
            
            sus k normie = 0
            bestie (k < kernel_size) {
                sus sample_x normie = x + k - kernel_size / 2
                sample_x = mathz.clamp(sample_x, 0, canvas.width - 1)
                
                sus sample_color Color = drawz_get_pixel(canvas, sample_x, y)
                sus weight drip = kernel[k]
                
                r = r + sample_color.r * weight
                g = g + sample_color.g * weight
                b = b + sample_color.b * weight
                a = a + sample_color.a * weight
                
                k = k + 1
            }
            
            sus blurred_color Color = drawz_create_color(r, g, b, a)
            temp_buffer[y * canvas.width + x] = drawz_color_to_rgba(blurred_color)
            
            x = x + 1
        }
        y = y + 1
    }
    
    fr fr Vertical pass (copy from temp buffer back to canvas)
    y = rect.y
    bestie (y < rect.y + rect.height) {
        sus x normie = rect.x
        bestie (x < rect.x + rect.width) {
            sus r drip = 0.0
            sus g drip = 0.0
            sus b drip = 0.0
            sus a drip = 0.0
            
            sus k normie = 0
            bestie (k < kernel_size) {
                sus sample_y normie = y + k - kernel_size / 2
                sample_y = mathz.clamp(sample_y, 0, canvas.height - 1)
                
                sus sample_rgba normie = temp_buffer[sample_y * canvas.width + x]
                sus sample_color Color = drawz_rgba_to_color(sample_rgba)
                sus weight drip = kernel[k]
                
                r = r + sample_color.r * weight
                g = g + sample_color.g * weight
                b = b + sample_color.b * weight
                a = a + sample_color.a * weight
                
                k = k + 1
            }
            
            sus final_color Color = drawz_create_color(r, g, b, a)
            drawz_set_pixel_safe(canvas, x, y, final_color)
            
            x = x + 1
        }
        y = y + 1
    }
    
    damn true
}

slay drawz_apply_unsharp_mask(canvas Canvas, rect Rect2D, strength drip) lit {
    fr fr Unsharp mask filter for sharpening
    fr fr Create blurred version
    sus blurred_canvas Canvas = drawz_copy_canvas_region(canvas, rect)
    drawz_apply_gaussian_blur(blurred_canvas, drawz_create_rect(0.0, 0.0, rect.width, rect.height), 2.0)
    
    fr fr Apply unsharp mask formula: sharpened = original + (original - blurred) * strength
    sus y normie = rect.y
    bestie (y < rect.y + rect.height) {
        sus x normie = rect.x
        bestie (x < rect.x + rect.width) {
            sus original Color = drawz_get_pixel(canvas, x, y)
            sus blurred Color = drawz_get_pixel(blurred_canvas, x - rect.x, y - rect.y)
            
            fr fr Calculate difference
            sus diff_r drip = (original.r - blurred.r) * strength
            sus diff_g drip = (original.g - blurred.g) * strength
            sus diff_b drip = (original.b - blurred.b) * strength
            
            fr fr Apply sharpening
            sus sharp_r normie = mathz.clamp(original.r + diff_r, 0, 255)
            sus sharp_g normie = mathz.clamp(original.g + diff_g, 0, 255)
            sus sharp_b normie = mathz.clamp(original.b + diff_b, 0, 255)
            
            sus sharpened Color = drawz_create_color(sharp_r, sharp_g, sharp_b, original.a)
            drawz_set_pixel_safe(canvas, x, y, sharpened)
            
            x = x + 1
        }
        y = y + 1
    }
    
    damn true
}

fr fr ===== ENHANCED UTILITY FUNCTIONS =====

slay drawz_create_point(x drip, y drip) Point2D {
    sus point Point2D
    point.x = x
    point.y = y
    damn point
}

slay drawz_create_rect(x drip, y drip, width drip, height drip) Rect2D {
    sus rect Rect2D
    rect.x = x
    rect.y = y
    rect.width = width
    rect.height = height
    damn rect
}

slay drawz_create_color(r normie, g normie, b normie, a normie) Color {
    sus color Color
    color.r = mathz.clamp(r, 0, 255)
    color.g = mathz.clamp(g, 0, 255)
    color.b = mathz.clamp(b, 0, 255)
    color.a = mathz.clamp(a, 0, 255)
    damn color
}

slay drawz_create_colorf(r drip, g drip, b drip, a drip) ColorF {
    sus color ColorF
    color.r = mathz.clamp(r, 0.0, 1.0)
    color.g = mathz.clamp(g, 0.0, 1.0)
    color.b = mathz.clamp(b, 0.0, 1.0)
    color.a = mathz.clamp(a, 0.0, 1.0)
    damn color
}

slay drawz_color_to_rgba(color Color) normie {
    damn (color.a << 24) | (color.r << 16) | (color.g << 8) | color.b
}

slay drawz_rgba_to_color(rgba normie) Color {
    sus color Color
    color.a = (rgba >> 24) & 0xFF
    color.r = (rgba >> 16) & 0xFF
    color.g = (rgba >> 8) & 0xFF
    color.b = rgba & 0xFF
    damn color
}

slay drawz_colorf_to_color(colorf ColorF) Color {
    damn drawz_create_color(
        colorf.r * 255.0,
        colorf.g * 255.0,
        colorf.b * 255.0,
        colorf.a * 255.0
    )
}

slay drawz_color_to_colorf(color Color) ColorF {
    damn drawz_create_colorf(
        color.r / 255.0,
        color.g / 255.0,
        color.b / 255.0,
        color.a / 255.0
    )
}

slay drawz_blend_colors_advanced(base Color, overlay Color, blend_mode normie) Color {
    ready (overlay.a == 0) {
        damn base
    }
    
    sus base_f ColorF = drawz_color_to_colorf(base)
    sus overlay_f ColorF = drawz_color_to_colorf(overlay)
    sus result_f ColorF
    
    ready (blend_mode == BLEND_MULTIPLY) {
        result_f.r = base_f.r * overlay_f.r
        result_f.g = base_f.g * overlay_f.g
        result_f.b = base_f.b * overlay_f.b
    } otherwise ready (blend_mode == BLEND_SCREEN) {
        result_f.r = 1.0 - (1.0 - base_f.r) * (1.0 - overlay_f.r)
        result_f.g = 1.0 - (1.0 - base_f.g) * (1.0 - overlay_f.g)
        result_f.b = 1.0 - (1.0 - base_f.b) * (1.0 - overlay_f.b)
    } otherwise ready (blend_mode == BLEND_OVERLAY) {
        result_f.r = ready (base_f.r < 0.5) 2.0 * base_f.r * overlay_f.r otherwise 1.0 - 2.0 * (1.0 - base_f.r) * (1.0 - overlay_f.r)
        result_f.g = ready (base_f.g < 0.5) 2.0 * base_f.g * overlay_f.g otherwise 1.0 - 2.0 * (1.0 - base_f.g) * (1.0 - overlay_f.g)
        result_f.b = ready (base_f.b < 0.5) 2.0 * base_f.b * overlay_f.b otherwise 1.0 - 2.0 * (1.0 - base_f.b) * (1.0 - overlay_f.b)
    } otherwise {
        fr fr Default to normal blending
        sus alpha drip = overlay_f.a
        result_f.r = base_f.r * (1.0 - alpha) + overlay_f.r * alpha
        result_f.g = base_f.g * (1.0 - alpha) + overlay_f.g * alpha
        result_f.b = base_f.b * (1.0 - alpha) + overlay_f.b * alpha
    }
    
    result_f.a = mathz.max(base_f.a, overlay_f.a)
    damn drawz_colorf_to_color(result_f)
}

slay drawz_lerp_point(a Point2D, b Point2D, t drip) Point2D {
    damn drawz_create_point(
        a.x + (b.x - a.x) * t,
        a.y + (b.y - a.y) * t
    )
}

slay drawz_point_in_polygon(point Point2D, polygon [100]Point2D, num_points normie) lit {
    fr fr Ray casting algorithm for point-in-polygon test
    sus inside lit = false
    sus j normie = num_points - 1
    
    sus i normie = 0
    bestie (i < num_points) {
        ready ((polygon[i].y > point.y) != (polygon[j].y > point.y) &&
               (point.x < (polygon[j].x - polygon[i].x) * (point.y - polygon[i].y) / (polygon[j].y - polygon[i].y) + polygon[i].x)) {
            inside = !inside
        }
        j = i
        i = i + 1
    }
    
    damn inside
}

slay drawz_plot_pixel_alpha(canvas Canvas, x normie, y normie, color Color, alpha drip) lit {
    ready (drawz_is_pixel_in_bounds(canvas, x, y)) {
        sus alpha_color Color = drawz_create_color(color.r, color.g, color.b, color.a * alpha)
        drawz_set_pixel_safe(canvas, x, y, alpha_color)
        damn true
    }
    damn false
}

slay drawz_is_pixel_in_bounds(canvas Canvas, x normie, y normie) lit {
    damn x >= 0 && x < canvas.width && y >= 0 && y < canvas.height
}

fr fr ===== TRANSFORMATION SYSTEM =====

slay drawz_reset_transform(canvas Canvas) lit {
    fr fr Reset to identity matrix
    canvas.transform_matrix[0] = 1.0   fr fr Scale X
    canvas.transform_matrix[1] = 0.0   fr fr Shear X
    canvas.transform_matrix[2] = 0.0   fr fr Translate X
    canvas.transform_matrix[3] = 0.0   fr fr Shear Y
    canvas.transform_matrix[4] = 1.0   fr fr Scale Y
    canvas.transform_matrix[5] = 0.0   fr fr Translate Y
    canvas.transform_matrix[6] = 0.0   fr fr Perspective X
    canvas.transform_matrix[7] = 0.0   fr fr Perspective Y
    canvas.transform_matrix[8] = 1.0   fr fr Perspective W
    damn true
}

slay drawz_translate(canvas Canvas, dx drip, dy drip) lit {
    fr fr Apply translation to current transform
    canvas.transform_matrix[2] = canvas.transform_matrix[2] + dx
    canvas.transform_matrix[5] = canvas.transform_matrix[5] + dy
    damn true
}

slay drawz_scale(canvas Canvas, sx drip, sy drip) lit {
    fr fr Apply scaling to current transform
    canvas.transform_matrix[0] = canvas.transform_matrix[0] * sx
    canvas.transform_matrix[4] = canvas.transform_matrix[4] * sy
    damn true
}

slay drawz_rotate(canvas Canvas, angle drip) lit {
    fr fr Apply rotation to current transform
    sus cos_a drip = mathz.cos(angle * 3.14159 / 180.0)
    sus sin_a drip = mathz.sin(angle * 3.14159 / 180.0)
    
    sus m00 drip = canvas.transform_matrix[0]
    sus m01 drip = canvas.transform_matrix[1]
    sus m10 drip = canvas.transform_matrix[3]
    sus m11 drip = canvas.transform_matrix[4]
    
    canvas.transform_matrix[0] = m00 * cos_a - m01 * sin_a
    canvas.transform_matrix[1] = m00 * sin_a + m01 * cos_a
    canvas.transform_matrix[3] = m10 * cos_a - m11 * sin_a
    canvas.transform_matrix[4] = m10 * sin_a + m11 * cos_a
    
    damn true
}

fr fr ===== PLACEHOLDER IMPLEMENTATIONS FOR COMPLEX FUNCTIONS =====
fr fr These would be fully implemented in a production graphics library

slay drawz_draw_line(canvas Canvas, start Point2D, end Point2D) lit {
    fr fr Basic Bresenham line (fallback for non-antialiased)
    sus x0 normie = start.x
    sus y0 normie = start.y
    sus x1 normie = end.x
    sus y1 normie = end.y
    
    sus dx normie = mathz.abs(x1 - x0)
    sus dy normie = mathz.abs(y1 - y0)
    sus sx normie = ready (x0 < x1) 1 otherwise -1
    sus sy normie = ready (y0 < y1) 1 otherwise -1
    sus err normie = dx - dy
    
    bestie (true) {
        drawz_set_pixel_safe(canvas, x0, y0, canvas.stroke_color)
        
        ready (x0 == x1 && y0 == y1) break
        
        sus e2 normie = 2 * err
        ready (e2 > -dy) {
            err = err - dy
            x0 = x0 + sx
        }
        ready (e2 < dx) {
            err = err + dx
            y0 = y0 + sy
        }
    }
    
    damn true
}

slay drawz_get_pixel(canvas Canvas, x normie, y normie) Color {
    sus color Color
    ready (drawz_is_pixel_in_bounds(canvas, x, y)) {
        sus index normie = y * canvas.width + x
        sus rgba normie = canvas.pixels[index]
        color = drawz_rgba_to_color(rgba)
    }
    damn color
}

fr fr Stub implementations for advanced features
slay drawz_sample_gradient(gradient Gradient, x normie, y normie, width normie, height normie) ColorF { damn drawz_create_colorf(0.5, 0.5, 0.5, 1.0) }
slay drawz_get_font_metrics(font_family tea, font_size drip) FontMetrics { sus m FontMetrics; damn m }
slay drawz_measure_text_width(text tea, style TextStyle) drip { damn style.font_size * 0.6 * stringz.len(text) }
slay drawz_render_character_bold(canvas Canvas, char normie, x drip, y drip, width drip, height drip) lit { damn true }
slay drawz_render_character_regular(canvas Canvas, char normie, x drip, y drip, width drip, height drip) lit { damn true }
slay drawz_apply_italic_transform(canvas Canvas, x drip, y drip, width drip, height drip) lit { damn true }
slay drawz_copy_canvas_region(canvas Canvas, rect Rect2D) Canvas { damn canvas }
slay drawz_apply_emboss_filter(canvas Canvas, rect Rect2D, strength drip) lit { damn true }
slay drawz_apply_edge_detection(canvas Canvas, rect Rect2D, strength drip) lit { damn true }
slay drawz_add_noise(canvas Canvas, rect Rect2D, strength drip) lit { damn true }
slay drawz_apply_sepia_tone(canvas Canvas, rect Rect2D, strength drip) lit { damn true }
slay drawz_convert_to_grayscale(canvas Canvas, rect Rect2D) lit { damn true }
slay drawz_invert_colors(canvas Canvas, rect Rect2D) lit { damn true }
slay drawz_adjust_brightness(canvas Canvas, rect Rect2D, strength drip) lit { damn true }
slay drawz_adjust_contrast(canvas Canvas, rect Rect2D, strength drip) lit { damn true }

fr fr Export enhanced module
facts DRAWZ_ENHANCED_MODULE_LOADED lit = true
