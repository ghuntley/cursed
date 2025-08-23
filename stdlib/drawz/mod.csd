fr fr CURSED DrawZ Module - 2D Graphics Primitives and Drawing
fr fr Professional 2D graphics operations for CURSED applications
fr fr Pure CURSED implementation with efficient algorithms

yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "memoryz"

fr fr ===== COLOR CONSTANTS =====

facts COLOR_TRANSPARENT normie = 0x00000000
facts COLOR_BLACK normie = 0xFF000000
facts COLOR_WHITE normie = 0xFFFFFFFF
facts COLOR_RED normie = 0xFFFF0000
facts COLOR_GREEN normie = 0xFF00FF00
facts COLOR_BLUE normie = 0xFF0000FF
facts COLOR_YELLOW normie = 0xFFFFFF00
facts COLOR_MAGENTA normie = 0xFFFF00FF
facts COLOR_CYAN normie = 0xFF00FFFF

fr fr ===== DRAWING MODES =====

facts DRAW_MODE_FILL normie = 0
facts DRAW_MODE_STROKE normie = 1
facts DRAW_MODE_BOTH normie = 2

fr fr ===== LINE STYLES =====

facts LINE_SOLID normie = 0
facts LINE_DASHED normie = 1
facts LINE_DOTTED normie = 2
facts LINE_DASH_DOT normie = 3

fr fr ===== 2D GRAPHICS STRUCTURES =====

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

be_like Color = struct {
    r normie,  fr fr 0-255
    g normie,  fr fr 0-255 
    b normie,  fr fr 0-255
    a normie   fr fr 0-255 alpha
}

be_like Canvas = struct {
    width normie,
    height normie,
    pixels [1048576]normie,  fr fr RGBA pixels (1024x1024 max)
    stroke_color Color,
    fill_color Color,
    line_width drip,
    line_style normie
}

be_like BezierCurve = struct {
    start Point2D,
    control1 Point2D,
    control2 Point2D,
    end Point2D
}

fr fr ===== CANVAS OPERATIONS =====

slay drawz_create_canvas(width normie, height normie) Canvas {
    sus canvas Canvas
    canvas.width = width
    canvas.height = height
    
    fr fr Initialize with transparent pixels
    sus i normie = 0
    bestie (i < width * height) {
        canvas.pixels[i] = COLOR_TRANSPARENT
        i = i + 1
    }
    
    fr fr Set default drawing properties
    canvas.stroke_color.r = 0
    canvas.stroke_color.g = 0
    canvas.stroke_color.b = 0
    canvas.stroke_color.a = 255
    
    canvas.fill_color.r = 255
    canvas.fill_color.g = 255
    canvas.fill_color.b = 255
    canvas.fill_color.a = 255
    
    canvas.line_width = 1.0
    canvas.line_style = LINE_SOLID
    
    damn canvas
}

slay drawz_clear_canvas(canvas Canvas, color Color) lit {
    sus rgba normie = (color.a << 24) | (color.r << 16) | (color.g << 8) | color.b
    
    sus i normie = 0
    bestie (i < canvas.width * canvas.height) {
        canvas.pixels[i] = rgba
        i = i + 1
    }
    
    damn true
}

slay drawz_set_pixel(canvas Canvas, x normie, y normie, color Color) lit {
    ready (x >= 0 && x < canvas.width && y >= 0 && y < canvas.height) {
        sus index normie = y * canvas.width + x
        sus rgba normie = (color.a << 24) | (color.r << 16) | (color.g << 8) | color.b
        canvas.pixels[index] = rgba
        damn true
    }
    damn false
}

slay drawz_get_pixel(canvas Canvas, x normie, y normie) Color {
    sus color Color
    ready (x >= 0 && x < canvas.width && y >= 0 && y < canvas.height) {
        sus index normie = y * canvas.width + x
        sus rgba normie = canvas.pixels[index]
        
        color.a = (rgba >> 24) & 0xFF
        color.r = (rgba >> 16) & 0xFF
        color.g = (rgba >> 8) & 0xFF
        color.b = rgba & 0xFF
    }
    damn color
}

fr fr ===== LINE DRAWING =====

slay drawz_draw_line(canvas Canvas, start Point2D, end Point2D) lit {
    fr fr Bresenham's line algorithm
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
        drawz_set_pixel(canvas, x0, y0, canvas.stroke_color)
        
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

slay drawz_draw_thick_line(canvas Canvas, start Point2D, end Point2D, thickness drip) lit {
    fr fr Draw thick line using multiple parallel lines
    sus half_thickness drip = thickness / 2.0
    
    fr fr Calculate perpendicular vector
    sus dx drip = end.x - start.x
    sus dy drip = end.y - start.y
    sus length drip = mathz.sqrt(dx * dx + dy * dy)
    
    ready (length == 0.0) {
        damn false
    }
    
    sus perp_x drip = -dy / length * half_thickness
    sus perp_y drip = dx / length * half_thickness
    
    fr fr Draw multiple lines
    sus offset drip = -half_thickness
    bestie (offset <= half_thickness) {
        sus factor drip = offset / half_thickness
        sus start_offset Point2D
        sus end_offset Point2D
        
        start_offset.x = start.x + perp_x * factor
        start_offset.y = start.y + perp_y * factor
        end_offset.x = end.x + perp_x * factor
        end_offset.y = end.y + perp_y * factor
        
        drawz_draw_line(canvas, start_offset, end_offset)
        offset = offset + 0.5
    }
    
    damn true
}

fr fr ===== RECTANGLE DRAWING =====

slay drawz_draw_rect(canvas Canvas, rect Rect2D, mode normie) lit {
    ready (mode == DRAW_MODE_FILL || mode == DRAW_MODE_BOTH) {
        fr fr Fill rectangle
        sus y normie = rect.y
        bestie (y < rect.y + rect.height) {
            sus x normie = rect.x
            bestie (x < rect.x + rect.width) {
                drawz_set_pixel(canvas, x, y, canvas.fill_color)
                x = x + 1
            }
            y = y + 1
        }
    }
    
    ready (mode == DRAW_MODE_STROKE || mode == DRAW_MODE_BOTH) {
        fr fr Draw rectangle outline
        sus top_left Point2D = {x: rect.x, y: rect.y}
        sus top_right Point2D = {x: rect.x + rect.width - 1, y: rect.y}
        sus bottom_left Point2D = {x: rect.x, y: rect.y + rect.height - 1}
        sus bottom_right Point2D = {x: rect.x + rect.width - 1, y: rect.y + rect.height - 1}
        
        drawz_draw_line(canvas, top_left, top_right)
        drawz_draw_line(canvas, top_right, bottom_right)
        drawz_draw_line(canvas, bottom_right, bottom_left)
        drawz_draw_line(canvas, bottom_left, top_left)
    }
    
    damn true
}

slay drawz_draw_rounded_rect(canvas Canvas, rect Rect2D, radius drip, mode normie) lit {
    fr fr Draw rounded rectangle using circles at corners
    ready (radius <= 0.0) {
        damn drawz_draw_rect(canvas, rect, mode)
    }
    
    fr fr Clamp radius to half of smaller dimension
    sus max_radius drip = mathz.min(rect.width, rect.height) / 2.0
    ready (radius > max_radius) {
        radius = max_radius
    }
    
    fr fr Define corner circles
    sus top_left Circle2D = {
        center: {x: rect.x + radius, y: rect.y + radius},
        radius: radius
    }
    sus top_right Circle2D = {
        center: {x: rect.x + rect.width - radius, y: rect.y + radius},
        radius: radius
    }
    sus bottom_left Circle2D = {
        center: {x: rect.x + radius, y: rect.y + rect.height - radius},
        radius: radius
    }
    sus bottom_right Circle2D = {
        center: {x: rect.x + rect.width - radius, y: rect.y + rect.height - radius},
        radius: radius
    }
    
    ready (mode == DRAW_MODE_FILL || mode == DRAW_MODE_BOTH) {
        fr fr Fill corner circles
        drawz_draw_circle(canvas, top_left, DRAW_MODE_FILL)
        drawz_draw_circle(canvas, top_right, DRAW_MODE_FILL)
        drawz_draw_circle(canvas, bottom_left, DRAW_MODE_FILL)
        drawz_draw_circle(canvas, bottom_right, DRAW_MODE_FILL)
        
        fr fr Fill center rectangles
        sus center_rect Rect2D = {
            x: rect.x + radius,
            y: rect.y,
            width: rect.width - 2 * radius,
            height: rect.height
        }
        drawz_draw_rect(canvas, center_rect, DRAW_MODE_FILL)
        
        sus side_rect1 Rect2D = {
            x: rect.x,
            y: rect.y + radius,
            width: radius,
            height: rect.height - 2 * radius
        }
        drawz_draw_rect(canvas, side_rect1, DRAW_MODE_FILL)
        
        sus side_rect2 Rect2D = {
            x: rect.x + rect.width - radius,
            y: rect.y + radius,
            width: radius,
            height: rect.height - 2 * radius
        }
        drawz_draw_rect(canvas, side_rect2, DRAW_MODE_FILL)
    }
    
    ready (mode == DRAW_MODE_STROKE || mode == DRAW_MODE_BOTH) {
        fr fr Draw corner arcs and straight lines
        drawz_draw_arc(canvas, top_left, 180.0, 270.0)
        drawz_draw_arc(canvas, top_right, 270.0, 360.0)
        drawz_draw_arc(canvas, bottom_right, 0.0, 90.0)
        drawz_draw_arc(canvas, bottom_left, 90.0, 180.0)
        
        fr fr Draw straight edges
        sus top_start Point2D = {x: rect.x + radius, y: rect.y}
        sus top_end Point2D = {x: rect.x + rect.width - radius, y: rect.y}
        drawz_draw_line(canvas, top_start, top_end)
        
        sus right_start Point2D = {x: rect.x + rect.width, y: rect.y + radius}
        sus right_end Point2D = {x: rect.x + rect.width, y: rect.y + rect.height - radius}
        drawz_draw_line(canvas, right_start, right_end)
        
        sus bottom_start Point2D = {x: rect.x + rect.width - radius, y: rect.y + rect.height}
        sus bottom_end Point2D = {x: rect.x + radius, y: rect.y + rect.height}
        drawz_draw_line(canvas, bottom_start, bottom_end)
        
        sus left_start Point2D = {x: rect.x, y: rect.y + rect.height - radius}
        sus left_end Point2D = {x: rect.x, y: rect.y + radius}
        drawz_draw_line(canvas, left_start, left_end)
    }
    
    damn true
}

fr fr ===== CIRCLE DRAWING =====

slay drawz_draw_circle(canvas Canvas, circle Circle2D, mode normie) lit {
    fr fr Bresenham's circle algorithm
    sus cx normie = circle.center.x
    sus cy normie = circle.center.y
    sus r normie = circle.radius
    
    sus x normie = 0
    sus y normie = r
    sus d normie = 3 - 2 * r
    
    bestie (y >= x) {
        ready (mode == DRAW_MODE_STROKE || mode == DRAW_MODE_BOTH) {
            drawz_set_pixel(canvas, cx + x, cy + y, canvas.stroke_color)
            drawz_set_pixel(canvas, cx - x, cy + y, canvas.stroke_color)
            drawz_set_pixel(canvas, cx + x, cy - y, canvas.stroke_color)
            drawz_set_pixel(canvas, cx - x, cy - y, canvas.stroke_color)
            drawz_set_pixel(canvas, cx + y, cy + x, canvas.stroke_color)
            drawz_set_pixel(canvas, cx - y, cy + x, canvas.stroke_color)
            drawz_set_pixel(canvas, cx + y, cy - x, canvas.stroke_color)
            drawz_set_pixel(canvas, cx - y, cy - x, canvas.stroke_color)
        }
        
        ready (mode == DRAW_MODE_FILL || mode == DRAW_MODE_BOTH) {
            fr fr Draw horizontal lines to fill circle
            sus line_y1 normie = cy + y
            sus line_y2 normie = cy - y
            sus line_y3 normie = cy + x
            sus line_y4 normie = cy - x
            
            sus line_x normie = cx - y
            bestie (line_x <= cx + y) {
                drawz_set_pixel(canvas, line_x, line_y1, canvas.fill_color)
                drawz_set_pixel(canvas, line_x, line_y2, canvas.fill_color)
                line_x = line_x + 1
            }
            
            ready (x != y) {
                line_x = cx - x
                bestie (line_x <= cx + x) {
                    drawz_set_pixel(canvas, line_x, line_y3, canvas.fill_color)
                    drawz_set_pixel(canvas, line_x, line_y4, canvas.fill_color)
                    line_x = line_x + 1
                }
            }
        }
        
        ready (d > 0) {
            y = y - 1
            d = d + 4 * (x - y) + 10
        } otherwise {
            d = d + 4 * x + 6
        }
        x = x + 1
    }
    
    damn true
}

slay drawz_draw_arc(canvas Canvas, circle Circle2D, start_angle drip, end_angle drip) lit {
    fr fr Draw arc using parametric equations
    sus cx drip = circle.center.x
    sus cy drip = circle.center.y
    sus r drip = circle.radius
    
    sus start_rad drip = start_angle * 3.14159 / 180.0
    sus end_rad drip = end_angle * 3.14159 / 180.0
    
    sus angle_step drip = 1.0 / r  fr fr Adaptive step size based on radius
    sus angle drip = start_rad
    
    bestie (angle <= end_rad) {
        sus x normie = cx + r * mathz.cos(angle)
        sus y normie = cy + r * mathz.sin(angle)
        drawz_set_pixel(canvas, x, y, canvas.stroke_color)
        angle = angle + angle_step
    }
    
    damn true
}

fr fr ===== BEZIER CURVES =====

slay drawz_draw_bezier_curve(canvas Canvas, curve BezierCurve) lit {
    fr fr Draw cubic Bezier curve using parametric evaluation
    sus steps normie = 100
    sus step drip = 1.0 / steps
    sus t drip = 0.0
    
    bestie (t <= 1.0) {
        fr fr Calculate point on curve
        sus t_inv drip = 1.0 - t
        sus t2 drip = t * t
        sus t3 drip = t2 * t
        sus t_inv2 drip = t_inv * t_inv
        sus t_inv3 drip = t_inv2 * t_inv
        
        sus x drip = t_inv3 * curve.start.x + 
                    3.0 * t_inv2 * t * curve.control1.x +
                    3.0 * t_inv * t2 * curve.control2.x +
                    t3 * curve.end.x
                    
        sus y drip = t_inv3 * curve.start.y + 
                    3.0 * t_inv2 * t * curve.control1.y +
                    3.0 * t_inv * t2 * curve.control2.y +
                    t3 * curve.end.y
        
        drawz_set_pixel(canvas, x, y, canvas.stroke_color)
        t = t + step
    }
    
    damn true
}

fr fr ===== POLYGON DRAWING =====

slay drawz_draw_polygon(canvas Canvas, points [100]Point2D, num_points normie, mode normie) lit {
    ready (num_points < 3) {
        damn false
    }
    
    ready (mode == DRAW_MODE_STROKE || mode == DRAW_MODE_BOTH) {
        fr fr Draw polygon outline
        sus i normie = 0
        bestie (i < num_points) {
            sus next_i normie = (i + 1) % num_points
            drawz_draw_line(canvas, points[i], points[next_i])
            i = i + 1
        }
    }
    
    ready (mode == DRAW_MODE_FILL || mode == DRAW_MODE_BOTH) {
        fr fr Fill polygon using scanline algorithm
        fr fr Find min and max y coordinates
        sus min_y normie = points[0].y
        sus max_y normie = points[0].y
        
        sus i normie = 1
        bestie (i < num_points) {
            ready (points[i].y < min_y) min_y = points[i].y
            ready (points[i].y > max_y) max_y = points[i].y
            i = i + 1
        }
        
        fr fr Scanline fill
        sus y normie = min_y
        bestie (y <= max_y) {
            sus intersections [100]drip
            sus intersection_count normie = 0
            
            fr fr Find intersections with polygon edges
            i = 0
            bestie (i < num_points) {
                sus next_i normie = (i + 1) % num_points
                sus p1 Point2D = points[i]
                sus p2 Point2D = points[next_i]
                
                ready (p1.y != p2.y) {
                    ready ((p1.y <= y && y < p2.y) || (p2.y <= y && y < p1.y)) {
                        sus x_intersect drip = p1.x + (y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y)
                        intersections[intersection_count] = x_intersect
                        intersection_count = intersection_count + 1
                    }
                }
                i = i + 1
            }
            
            fr fr Sort intersections (simple bubble sort)
            sus j normie = 0
            bestie (j < intersection_count - 1) {
                sus k normie = 0
                bestie (k < intersection_count - 1 - j) {
                    ready (intersections[k] > intersections[k + 1]) {
                        sus temp drip = intersections[k]
                        intersections[k] = intersections[k + 1]
                        intersections[k + 1] = temp
                    }
                    k = k + 1
                }
                j = j + 1
            }
            
            fr fr Fill between pairs of intersections
            i = 0
            bestie (i < intersection_count) {
                ready (i + 1 < intersection_count) {
                    sus x_start normie = intersections[i]
                    sus x_end normie = intersections[i + 1]
                    sus x normie = x_start
                    bestie (x <= x_end) {
                        drawz_set_pixel(canvas, x, y, canvas.fill_color)
                        x = x + 1
                    }
                }
                i = i + 2
            }
            
            y = y + 1
        }
    }
    
    damn true
}

fr fr ===== TEXT RENDERING =====

slay drawz_draw_text(canvas Canvas, text tea, x normie, y normie, font_size normie) lit {
    fr fr Simple bitmap font rendering (placeholder implementation)
    yeet "stringz"
    
    sus char_width normie = font_size / 2
    sus char_height normie = font_size
    sus current_x normie = x
    
    sus i normie = 0
    bestie (i < stringz.len(text)) {
        sus char normie = stringz.char_at(text, i)
        
        fr fr Draw simple character outline (placeholder)
        sus char_rect Rect2D = {
            x: current_x,
            y: y,
            width: char_width,
            height: char_height
        }
        drawz_draw_rect(canvas, char_rect, DRAW_MODE_STROKE)
        
        current_x = current_x + char_width + 2
        i = i + 1
    }
    
    damn true
}

fr fr ===== COLOR UTILITIES =====

slay drawz_create_color(r normie, g normie, b normie, a normie) Color {
    sus color Color
    color.r = r
    color.g = g
    color.b = b
    color.a = a
    damn color
}

slay drawz_blend_colors(base Color, overlay Color, alpha drip) Color {
    sus result Color
    sus inv_alpha drip = 1.0 - alpha
    
    result.r = base.r * inv_alpha + overlay.r * alpha
    result.g = base.g * inv_alpha + overlay.g * alpha
    result.b = base.b * inv_alpha + overlay.b * alpha
    result.a = base.a * inv_alpha + overlay.a * alpha
    
    damn result
}

slay drawz_hsv_to_rgb(h drip, s drip, v drip) Color {
    fr fr Convert HSV to RGB color space
    sus c drip = v * s
    sus x drip = c * (1.0 - mathz.abs((h / 60.0) % 2.0 - 1.0))
    sus m drip = v - c
    
    sus r drip = 0.0
    sus g drip = 0.0
    sus b drip = 0.0
    
    ready (h >= 0.0 && h < 60.0) {
        r = c
        g = x
        b = 0.0
    } otherwise ready (h >= 60.0 && h < 120.0) {
        r = x
        g = c
        b = 0.0
    } otherwise ready (h >= 120.0 && h < 180.0) {
        r = 0.0
        g = c
        b = x
    } otherwise ready (h >= 180.0 && h < 240.0) {
        r = 0.0
        g = x
        b = c
    } otherwise ready (h >= 240.0 && h < 300.0) {
        r = x
        g = 0.0
        b = c
    } otherwise {
        r = c
        g = 0.0
        b = x
    }
    
    sus color Color
    color.r = (r + m) * 255.0
    color.g = (g + m) * 255.0
    color.b = (b + m) * 255.0
    color.a = 255
    
    damn color
}

fr fr ===== IMAGE OPERATIONS =====

slay drawz_save_canvas_to_ppm(canvas Canvas, filename tea) lit {
    fr fr Save canvas as PPM image format (simple text-based format)
    yeet "filez"
    
    sus header tea = "P3\n" + canvas.width + " " + canvas.height + "\n255\n"
    sus success lit = filez.write_file(filename, header)
    
    ready (!success) {
        damn false
    }
    
    sus y normie = 0
    bestie (y < canvas.height) {
        sus x normie = 0
        bestie (x < canvas.width) {
            sus color Color = drawz_get_pixel(canvas, x, y)
            sus pixel_data tea = color.r + " " + color.g + " " + color.b + " "
            filez.append_to_file(filename, pixel_data)
            x = x + 1
        }
        filez.append_to_file(filename, "\n")
        y = y + 1
    }
    
    damn true
}

fr fr ===== TRANSFORMATION UTILITIES =====

slay drawz_rotate_point(point Point2D, center Point2D, angle drip) Point2D {
    fr fr Rotate point around center by angle in degrees
    sus angle_rad drip = angle * 3.14159 / 180.0
    sus cos_a drip = mathz.cos(angle_rad)
    sus sin_a drip = mathz.sin(angle_rad)
    
    sus dx drip = point.x - center.x
    sus dy drip = point.y - center.y
    
    sus result Point2D
    result.x = center.x + dx * cos_a - dy * sin_a
    result.y = center.y + dx * sin_a + dy * cos_a
    
    damn result
}

slay drawz_scale_point(point Point2D, center Point2D, scale_x drip, scale_y drip) Point2D {
    sus result Point2D
    result.x = center.x + (point.x - center.x) * scale_x
    result.y = center.y + (point.y - center.y) * scale_y
    damn result
}

fr fr ===== ADVANCED DRAWING UTILITIES =====

slay drawz_draw_gradient_rect(canvas Canvas, rect Rect2D, start_color Color, end_color Color, vertical lit) lit {
    fr fr Draw rectangle with linear gradient
    sus y normie = rect.y
    bestie (y < rect.y + rect.height) {
        sus x normie = rect.x
        bestie (x < rect.x + rect.width) {
            sus t drip
            ready (vertical) {
                t = (y - rect.y) / rect.height
            } otherwise {
                t = (x - rect.x) / rect.width
            }
            
            sus blended_color Color = drawz_blend_colors(start_color, end_color, t)
            drawz_set_pixel(canvas, x, y, blended_color)
            x = x + 1
        }
        y = y + 1
    }
    
    damn true
}

slay drawz_draw_checkered_pattern(canvas Canvas, rect Rect2D, color1 Color, color2 Color, square_size normie) lit {
    fr fr Draw checkered pattern within rectangle
    sus y normie = rect.y
    bestie (y < rect.y + rect.height) {
        sus x normie = rect.x
        bestie (x < rect.x + rect.width) {
            sus check_x normie = (x - rect.x) / square_size
            sus check_y normie = (y - rect.y) / square_size
            sus use_color1 lit = ((check_x + check_y) % 2) == 0
            
            sus color Color = ready (use_color1) color1 otherwise color2
            drawz_set_pixel(canvas, x, y, color)
            x = x + 1
        }
        y = y + 1
    }
    
    damn true
}

fr fr Export all public functions for use by other modules
facts DRAWZ_MODULE_LOADED lit = true
