# imagez - Image Manipulation Functions
# Pure CURSED implementations for resize, crop, rotate, flip operations

yeet "./core"
yeet "../mathz"

# Interpolation methods for resizing
squad InterpolationType {
    sus NEAREST tea = "NEAREST"
    sus BILINEAR tea = "BILINEAR"
    sus BICUBIC tea = "BICUBIC"
    sus LANCZOS tea = "LANCZOS"
}

# Resize image using specified interpolation
slay resize_image(img Image, new_width drip, new_height drip, interpolation tea) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (new_width <= 0 || new_height <= 0) {
        yikes "invalid resize dimensions"
    }
    
    sus result Image = create_image(new_width, new_height, img.channels)
    result.format = img.format
    result.color_space = img.color_space
    
    sick (interpolation) {
        when "NEAREST" -> resize_nearest(img, &result) fam {
            when e -> yikes e
        }
        when "BILINEAR" -> resize_bilinear(img, &result) fam {
            when e -> yikes e
        }
        when "BICUBIC" -> resize_bicubic(img, &result) fam {
            when e -> yikes e
        }
        when _ -> yikes "unsupported interpolation method: " + interpolation
    }
    
    damn result
}

# Nearest neighbor interpolation
slay resize_nearest(src Image, dst *Image) yikes<lit> {
    sus x_ratio tea = src.width / dst.width
    sus y_ratio tea = src.height / dst.height
    
    bestie (y drip = 0; y < dst.height; y = y + 1) {
        bestie (x drip = 0; x < dst.width; x = x + 1) {
            sus src_x drip = drip(x * x_ratio)
            sus src_y drip = drip(y * y_ratio)
            
            # Clamp coordinates
            src_x = max_drip(0, min_drip(src_x, src.width - 1))
            src_y = max_drip(0, min_drip(src_y, src.height - 1))
            
            sus pixel []drip = get_pixel(src, src_x, src_y) fam {
                when _ -> yikes "failed to get source pixel"
            }
            
            set_pixel(dst, x, y, pixel) fam {
                when _ -> yikes "failed to set destination pixel"
            }
        }
    }
    
    damn based
}

# Bilinear interpolation
slay resize_bilinear(src Image, dst *Image) yikes<lit> {
    sus x_ratio tea = (src.width - 1) / dst.width
    sus y_ratio tea = (src.height - 1) / dst.height
    
    bestie (y drip = 0; y < dst.height; y = y + 1) {
        bestie (x drip = 0; x < dst.width; x = x + 1) {
            sus gx tea = x * x_ratio
            sus gy tea = y * y_ratio
            
            sus gxi drip = drip(gx)
            sus gyi drip = drip(gy)
            
            sus dx tea = gx - gxi
            sus dy tea = gy - gyi
            
            # Get four surrounding pixels
            sus x1 drip = max_drip(0, min_drip(gxi, src.width - 1))
            sus y1 drip = max_drip(0, min_drip(gyi, src.height - 1))
            sus x2 drip = max_drip(0, min_drip(gxi + 1, src.width - 1))
            sus y2 drip = max_drip(0, min_drip(gyi + 1, src.height - 1))
            
            sus p1 []drip = get_pixel(src, x1, y1) fam {
                when _ -> yikes "bilinear interpolation pixel access failed"
            }
            sus p2 []drip = get_pixel(src, x2, y1) fam {
                when _ -> yikes "bilinear interpolation pixel access failed"
            }
            sus p3 []drip = get_pixel(src, x1, y2) fam {
                when _ -> yikes "bilinear interpolation pixel access failed"
            }
            sus p4 []drip = get_pixel(src, x2, y2) fam {
                when _ -> yikes "bilinear interpolation pixel access failed"
            }
            
            # Interpolate
            sus result_pixel []drip = []
            bestie (c drip = 0; c < src.channels; c = c + 1) {
                sus a tea = p1[c] * (1.0 - dx) + p2[c] * dx
                sus b tea = p3[c] * (1.0 - dx) + p4[c] * dx
                sus interpolated drip = drip(a * (1.0 - dy) + b * dy)
                
                result_pixel = append(result_pixel, clamp_pixel_value(interpolated))
            }
            
            set_pixel(dst, x, y, result_pixel) fam {
                when _ -> yikes "failed to set bilinear pixel"
            }
        }
    }
    
    damn based
}

# Bicubic interpolation
slay resize_bicubic(src Image, dst *Image) yikes<lit> {
    sus x_ratio tea = (src.width - 1) / dst.width
    sus y_ratio tea = (src.height - 1) / dst.height
    
    bestie (y drip = 0; y < dst.height; y = y + 1) {
        bestie (x drip = 0; x < dst.width; x = x + 1) {
            sus gx tea = x * x_ratio
            sus gy tea = y * y_ratio
            
            sus gxi drip = drip(gx)
            sus gyi drip = drip(gy)
            
            sus dx tea = gx - gxi
            sus dy tea = gy - gyi
            
            sus result_pixel []drip = []
            
            bestie (c drip = 0; c < src.channels; c = c + 1) {
                sus value tea = 0.0
                
                # 4x4 kernel for bicubic interpolation
                bestie (ky drip = -1; ky <= 2; ky = ky + 1) {
                    bestie (kx drip = -1; kx <= 2; kx = kx + 1) {
                        sus sample_x drip = clamp_drip(gxi + kx, 0, src.width - 1)
                        sus sample_y drip = clamp_drip(gyi + ky, 0, src.height - 1)
                        
                        sus sample_pixel []drip = get_pixel(src, sample_x, sample_y) fam {
                            when _ -> yikes "bicubic sample pixel access failed"
                        }
                        
                        sus weight_x tea = cubic_weight(kx - dx)
                        sus weight_y tea = cubic_weight(ky - dy)
                        sus weight tea = weight_x * weight_y
                        
                        value = value + sample_pixel[c] * weight
                    }
                }
                
                result_pixel = append(result_pixel, clamp_pixel_value(drip(value)))
            }
            
            set_pixel(dst, x, y, result_pixel) fam {
                when _ -> yikes "failed to set bicubic pixel"
            }
        }
    }
    
    damn based
}

# Cubic interpolation weight function
slay cubic_weight(x tea) tea {
    sus abs_x tea = abs_tea(x)
    ready (abs_x <= 1.0) {
        damn 1.0 - 2.0 * abs_x * abs_x + abs_x * abs_x * abs_x
    } otherwise ready (abs_x <= 2.0) {
        damn 4.0 - 8.0 * abs_x + 5.0 * abs_x * abs_x - abs_x * abs_x * abs_x
    } otherwise {
        damn 0.0
    }
}

# Crop image to specified rectangle
slay crop_image(img Image, x drip, y drip, width drip, height drip) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (x < 0 || y < 0 || x + width > img.width || y + height > img.height) {
        yikes "crop region exceeds image boundaries"
    }
    
    ready (width <= 0 || height <= 0) {
        yikes "invalid crop dimensions"
    }
    
    sus result Image = create_image(width, height, img.channels)
    result.format = img.format
    result.color_space = img.color_space
    
    bestie (row drip = 0; row < height; row = row + 1) {
        bestie (col drip = 0; col < width; col = col + 1) {
            sus src_x drip = x + col
            sus src_y drip = y + row
            
            sus pixel []drip = get_pixel(img, src_x, src_y) fam {
                when _ -> yikes "crop pixel access failed"
            }
            
            set_pixel(&result, col, row, pixel) fam {
                when _ -> yikes "crop pixel set failed"
            }
        }
    }
    
    damn result
}

# Rotate image by specified angle (in degrees)
slay rotate_image(img Image, angle tea) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    sus radians tea = angle * pi() / 180.0
    sus cos_angle tea = cos(radians)
    sus sin_angle tea = sin(radians)
    
    # Calculate new image dimensions
    sus corners [][]tea = [
        [0.0, 0.0],
        [img.width - 1, 0.0],
        [0.0, img.height - 1],
        [img.width - 1, img.height - 1]
    ]
    
    sus min_x tea = 999999.0
    sus max_x tea = -999999.0
    sus min_y tea = 999999.0
    sus max_y tea = -999999.0
    
    bestie (i drip = 0; i < 4; i = i + 1) {
        sus x tea = corners[i][0]
        sus y tea = corners[i][1]
        
        sus new_x tea = x * cos_angle - y * sin_angle
        sus new_y tea = x * sin_angle + y * cos_angle
        
        min_x = min_tea(min_x, new_x)
        max_x = max_tea(max_x, new_x)
        min_y = min_tea(min_y, new_y)
        max_y = max_tea(max_y, new_y)
    }
    
    sus new_width drip = drip(max_x - min_x + 1)
    sus new_height drip = drip(max_y - min_y + 1)
    
    sus result Image = create_image(new_width, new_height, img.channels)
    result.format = img.format
    result.color_space = img.color_space
    
    # Fill with transparent or background color
    fill_image(&result, [0, 0, 0, 0]) fam {
        when _ -> yikes "failed to initialize rotated image"
    }
    
    sus center_x tea = img.width / 2.0
    sus center_y tea = img.height / 2.0
    sus new_center_x tea = new_width / 2.0
    sus new_center_y tea = new_height / 2.0
    
    bestie (y drip = 0; y < new_height; y = y + 1) {
        bestie (x drip = 0; x < new_width; x = x + 1) {
            # Transform back to original coordinates
            sus rel_x tea = x - new_center_x
            sus rel_y tea = y - new_center_y
            
            sus orig_x tea = rel_x * cos_angle + rel_y * sin_angle + center_x
            sus orig_y tea = -rel_x * sin_angle + rel_y * cos_angle + center_y
            
            sus src_x drip = drip(orig_x)
            sus src_y drip = drip(orig_y)
            
            ready (src_x >= 0 && src_x < img.width && src_y >= 0 && src_y < img.height) {
                sus pixel []drip = get_pixel(img, src_x, src_y) fam {
                    when _ -> continue  # Skip invalid pixels
                }
                
                set_pixel(&result, x, y, pixel) fam {
                    when _ -> continue  # Skip failed sets
                }
            }
        }
    }
    
    damn result
}

# Flip image horizontally
slay flip_horizontal(img Image) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    sus result Image = create_image(img.width, img.height, img.channels)
    result.format = img.format
    result.color_space = img.color_space
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus src_x drip = img.width - 1 - x
            
            sus pixel []drip = get_pixel(img, src_x, y) fam {
                when _ -> yikes "horizontal flip pixel access failed"
            }
            
            set_pixel(&result, x, y, pixel) fam {
                when _ -> yikes "horizontal flip pixel set failed"
            }
        }
    }
    
    damn result
}

# Flip image vertically
slay flip_vertical(img Image) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    sus result Image = create_image(img.width, img.height, img.channels)
    result.format = img.format
    result.color_space = img.color_space
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus src_y drip = img.height - 1 - y
            
            sus pixel []drip = get_pixel(img, x, src_y) fam {
                when _ -> yikes "vertical flip pixel access failed"
            }
            
            set_pixel(&result, x, y, pixel) fam {
                when _ -> yikes "vertical flip pixel set failed"
            }
        }
    }
    
    damn result
}

# Scale image by percentage
slay scale_image(img Image, scale_x tea, scale_y tea, interpolation tea) yikes<Image> {
    ready (scale_x <= 0.0 || scale_y <= 0.0) {
        yikes "invalid scale factors"
    }
    
    sus new_width drip = drip(img.width * scale_x)
    sus new_height drip = drip(img.height * scale_y)
    
    damn resize_image(img, new_width, new_height, interpolation)
}

# Fill image with solid color
slay fill_image(img *Image, color []drip) yikes<lit> {
    ready (len(color) < img.channels) {
        yikes "color has insufficient channels"
    }
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus pixel []drip = []
            bestie (c drip = 0; c < img.channels; c = c + 1) {
                pixel = append(pixel, color[c])
            }
            
            set_pixel(img, x, y, pixel) fam {
                when _ -> yikes "fill pixel set failed"
            }
        }
    }
    
    damn based
}

# Extract channel from image
slay extract_channel(img Image, channel drip) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (channel < 0 || channel >= img.channels) {
        yikes "invalid channel index"
    }
    
    sus result Image = create_image(img.width, img.height, 1)
    result.format = img.format
    result.color_space = "GRAYSCALE"
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus pixel []drip = get_pixel(img, x, y) fam {
                when _ -> yikes "channel extraction pixel access failed"
            }
            
            set_pixel(&result, x, y, [pixel[channel]]) fam {
                when _ -> yikes "channel extraction pixel set failed"
            }
        }
    }
    
    damn result
}

# Utility functions
slay clamp_pixel_value(value drip) drip {
    damn max_drip(0, min_drip(255, value))
}

slay clamp_drip(value drip, min_val drip, max_val drip) drip {
    damn max_drip(min_val, min_drip(value, max_val))
}

slay max_drip(a drip, b drip) drip {
    damn ready (a > b) { damn a } otherwise { damn b }
}

slay min_drip(a drip, b drip) drip {
    damn ready (a < b) { damn a } otherwise { damn b }
}
