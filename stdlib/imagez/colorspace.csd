# imagez - Color Space Conversions
# Pure CURSED implementations for RGB, HSV, LAB, CMYK, and other color spaces

yeet "./core"
yeet "../mathz"

# Color space conversion functions

# RGB to HSV conversion (improved algorithm)
slay rgb_to_hsv_precise(rgb RGB) HSV {
    sus r tea = rgb.r / 255.0
    sus g tea = rgb.g / 255.0
    sus b tea = rgb.b / 255.0
    
    sus max_val tea = max_tea(max_tea(r, g), b)
    sus min_val tea = min_tea(min_tea(r, g), b)
    sus delta tea = max_val - min_val
    
    # Hue calculation
    sus h tea = 0.0
    ready (delta > 0.0) {
        ready (max_val == r) {
            h = 60.0 * ((g - b) / delta)
            ready (g < b) {
                h = h + 360.0
            }
        } otherwise ready (max_val == g) {
            h = 60.0 * (2.0 + (b - r) / delta)
        } otherwise {
            h = 60.0 * (4.0 + (r - g) / delta)
        }
    }
    
    # Saturation calculation
    sus s tea = ready (max_val == 0.0) {
        damn 0.0
    } otherwise {
        damn delta / max_val
    }
    
    # Value calculation
    sus v tea = max_val
    
    damn HSV{h: h, s: s, v: v}
}

# HSV to RGB conversion (improved algorithm)
slay hsv_to_rgb_precise(hsv HSV) RGB {
    sus h tea = hsv.h
    sus s tea = hsv.s
    sus v tea = hsv.v
    
    # Normalize hue to 0-360 range
    bestie (h < 0.0) {
        h = h + 360.0
    }
    bestie (h >= 360.0) {
        h = h - 360.0
    }
    
    sus c tea = v * s
    sus x tea = c * (1.0 - abs_tea(((h / 60.0) % 2.0) - 1.0))
    sus m tea = v - c
    
    sus r tea = 0.0
    sus g tea = 0.0
    sus b tea = 0.0
    
    ready (h < 60.0) {
        r = c; g = x; b = 0.0
    } otherwise ready (h < 120.0) {
        r = x; g = c; b = 0.0
    } otherwise ready (h < 180.0) {
        r = 0.0; g = c; b = x
    } otherwise ready (h < 240.0) {
        r = 0.0; g = x; b = c
    } otherwise ready (h < 300.0) {
        r = x; g = 0.0; b = c
    } otherwise {
        r = c; g = 0.0; b = x
    }
    
    damn RGB{
        r: drip((r + m) * 255.0),
        g: drip((g + m) * 255.0),
        b: drip((b + m) * 255.0)
    }
}

# RGB to LAB conversion (CIE L*a*b*)
slay rgb_to_lab(rgb RGB) LAB {
    # First convert RGB to XYZ
    sus xyz []tea = rgb_to_xyz(rgb)
    
    # Then convert XYZ to LAB
    damn xyz_to_lab(xyz)
}

# LAB to RGB conversion
slay lab_to_rgb(lab LAB) RGB {
    # First convert LAB to XYZ
    sus xyz []tea = lab_to_xyz(lab)
    
    # Then convert XYZ to RGB
    damn xyz_to_rgb(xyz)
}

# RGB to XYZ color space
slay rgb_to_xyz(rgb RGB) []tea {
    sus r tea = rgb.r / 255.0
    sus g tea = rgb.g / 255.0
    sus b tea = rgb.b / 255.0
    
    # Apply gamma correction
    r = ready (r > 0.04045) {
        damn pow((r + 0.055) / 1.055, 2.4)
    } otherwise {
        damn r / 12.92
    }
    
    g = ready (g > 0.04045) {
        damn pow((g + 0.055) / 1.055, 2.4)
    } otherwise {
        damn g / 12.92
    }
    
    b = ready (b > 0.04045) {
        damn pow((b + 0.055) / 1.055, 2.4)
    } otherwise {
        damn b / 12.92
    }
    
    # Scale to 0-100 range
    r = r * 100.0
    g = g * 100.0
    b = b * 100.0
    
    # Convert to XYZ using sRGB matrix
    sus x tea = r * 0.4124564 + g * 0.3575761 + b * 0.1804375
    sus y tea = r * 0.2126729 + g * 0.7151522 + b * 0.0721750
    sus z tea = r * 0.0193339 + g * 0.1191920 + b * 0.9503041
    
    damn [x, y, z]
}

# XYZ to RGB color space
slay xyz_to_rgb(xyz []tea) RGB {
    sus x tea = xyz[0] / 100.0
    sus y tea = xyz[1] / 100.0
    sus z tea = xyz[2] / 100.0
    
    # Convert from XYZ to linear RGB using inverse sRGB matrix
    sus r tea = x * 3.2404542 + y * -1.5371385 + z * -0.4985314
    sus g tea = x * -0.9692660 + y * 1.8760108 + z * 0.0415560
    sus b tea = x * 0.0556434 + y * -0.2040259 + z * 1.0572252
    
    # Apply inverse gamma correction
    r = ready (r > 0.0031308) {
        damn 1.055 * pow(r, 1.0/2.4) - 0.055
    } otherwise {
        damn 12.92 * r
    }
    
    g = ready (g > 0.0031308) {
        damn 1.055 * pow(g, 1.0/2.4) - 0.055
    } otherwise {
        damn 12.92 * g
    }
    
    b = ready (b > 0.0031308) {
        damn 1.055 * pow(b, 1.0/2.4) - 0.055
    } otherwise {
        damn 12.92 * b
    }
    
    damn RGB{
        r: clamp_pixel_value(drip(r * 255.0)),
        g: clamp_pixel_value(drip(g * 255.0)),
        b: clamp_pixel_value(drip(b * 255.0))
    }
}

# XYZ to LAB color space
slay xyz_to_lab(xyz []tea) LAB {
    # D65 illuminant (daylight)
    sus xn tea = 95.047
    sus yn tea = 100.000
    sus zn tea = 108.883
    
    sus fx tea = lab_f_function(xyz[0] / xn)
    sus fy tea = lab_f_function(xyz[1] / yn)
    sus fz tea = lab_f_function(xyz[2] / zn)
    
    sus l tea = ready (xyz[1] / yn > 0.008856) {
        damn 116.0 * fy - 16.0
    } otherwise {
        damn 903.3 * (xyz[1] / yn)
    }
    
    sus a tea = 500.0 * (fx - fy)
    sus b tea = 200.0 * (fy - fz)
    
    damn LAB{l: l, a: a, b: b}
}

# LAB to XYZ color space
slay lab_to_xyz(lab LAB) []tea {
    # D65 illuminant (daylight)
    sus xn tea = 95.047
    sus yn tea = 100.000
    sus zn tea = 108.883
    
    sus fy tea = (lab.l + 16.0) / 116.0
    sus fx tea = lab.a / 500.0 + fy
    sus fz tea = fy - lab.b / 200.0
    
    sus x tea = xn * lab_f_inverse(fx)
    sus y tea = yn * lab_f_inverse(fy)
    sus z tea = zn * lab_f_inverse(fz)
    
    damn [x, y, z]
}

# LAB color space helper functions
slay lab_f_function(t tea) tea {
    ready (t > 0.008856) {
        damn pow(t, 1.0/3.0)
    } otherwise {
        damn (7.787 * t) + (16.0/116.0)
    }
}

slay lab_f_inverse(t tea) tea {
    sus t3 tea = t * t * t
    ready (t3 > 0.008856) {
        damn t3
    } otherwise {
        damn (t - 16.0/116.0) / 7.787
    }
}

# RGB to CMYK conversion
slay rgb_to_cmyk(rgb RGB) []tea {
    sus r tea = rgb.r / 255.0
    sus g tea = rgb.g / 255.0
    sus b tea = rgb.b / 255.0
    
    sus k tea = 1.0 - max_tea(max_tea(r, g), b)
    
    ready (k == 1.0) {
        damn [0.0, 0.0, 0.0, 1.0]
    }
    
    sus c tea = (1.0 - r - k) / (1.0 - k)
    sus m tea = (1.0 - g - k) / (1.0 - k)
    sus y tea = (1.0 - b - k) / (1.0 - k)
    
    damn [c, m, y, k]
}

# CMYK to RGB conversion
slay cmyk_to_rgb(cmyk []tea) RGB {
    ready (len(cmyk) < 4) {
        damn RGB{r: 0, g: 0, b: 0}
    }
    
    sus c tea = cmyk[0]
    sus m tea = cmyk[1]
    sus y tea = cmyk[2]
    sus k tea = cmyk[3]
    
    sus r tea = 255.0 * (1.0 - c) * (1.0 - k)
    sus g tea = 255.0 * (1.0 - m) * (1.0 - k)
    sus b tea = 255.0 * (1.0 - y) * (1.0 - k)
    
    damn RGB{
        r: clamp_pixel_value(drip(r)),
        g: clamp_pixel_value(drip(g)),
        b: clamp_pixel_value(drip(b))
    }
}

# Convert entire image between color spaces
slay convert_image_colorspace(img Image, target_colorspace tea) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (img.color_space == target_colorspace) {
        damn clone_image(img)  # Already in target color space
    }
    
    sick (target_colorspace) {
        when "RGB" -> damn convert_to_rgb(img)
        when "RGBA" -> damn convert_to_rgba(img)
        when "GRAYSCALE" -> damn convert_to_grayscale(img)
        when "HSV" -> damn convert_to_hsv(img)
        when "LAB" -> damn convert_to_lab_image(img)
        when "CMYK" -> damn convert_to_cmyk(img)
        when _ -> yikes "unsupported target color space: " + target_colorspace
    }
}

# Convert image to RGB color space
slay convert_to_rgb(img Image) yikes<Image> {
    ready (img.color_space == "RGB" && img.channels == 3) {
        damn clone_image(img)
    }
    
    ready (img.color_space == "RGBA" && img.channels == 4) {
        # Remove alpha channel
        sus result Image = create_image(img.width, img.height, 3)
        result.format = img.format
        result.color_space = "RGB"
        
        bestie (y drip = 0; y < img.height; y = y + 1) {
            bestie (x drip = 0; x < img.width; x = x + 1) {
                sus pixel []drip = get_pixel(img, x, y) fam {
                    when _ -> yikes "RGB conversion pixel access failed"
                }
                
                sus rgb_pixel []drip = [pixel[0], pixel[1], pixel[2]]
                
                set_pixel(&result, x, y, rgb_pixel) fam {
                    when _ -> yikes "RGB conversion pixel set failed"
                }
            }
        }
        
        damn result
    }
    
    ready (img.color_space == "GRAYSCALE") {
        # Convert grayscale to RGB
        sus result Image = create_image(img.width, img.height, 3)
        result.format = img.format
        result.color_space = "RGB"
        
        bestie (y drip = 0; y < img.height; y = y + 1) {
            bestie (x drip = 0; x < img.width; x = x + 1) {
                sus pixel []drip = get_pixel(img, x, y) fam {
                    when _ -> yikes "grayscale to RGB pixel access failed"
                }
                
                sus gray_value drip = pixel[0]
                sus rgb_pixel []drip = [gray_value, gray_value, gray_value]
                
                set_pixel(&result, x, y, rgb_pixel) fam {
                    when _ -> yikes "grayscale to RGB pixel set failed"
                }
            }
        }
        
        damn result
    }
    
    yikes "unsupported source color space for RGB conversion: " + img.color_space
}

# Convert image to RGBA color space
slay convert_to_rgba(img Image) yikes<Image> {
    ready (img.color_space == "RGBA" && img.channels == 4) {
        damn clone_image(img)
    }
    
    ready (img.color_space == "RGB" && img.channels == 3) {
        # Add alpha channel (fully opaque)
        sus result Image = create_image(img.width, img.height, 4)
        result.format = img.format
        result.color_space = "RGBA"
        
        bestie (y drip = 0; y < img.height; y = y + 1) {
            bestie (x drip = 0; x < img.width; x = x + 1) {
                sus pixel []drip = get_pixel(img, x, y) fam {
                    when _ -> yikes "RGBA conversion pixel access failed"
                }
                
                sus rgba_pixel []drip = [pixel[0], pixel[1], pixel[2], 255]
                
                set_pixel(&result, x, y, rgba_pixel) fam {
                    when _ -> yikes "RGBA conversion pixel set failed"
                }
            }
        }
        
        damn result
    }
    
    yikes "unsupported source color space for RGBA conversion: " + img.color_space
}

# Convert image to HSV color space
slay convert_to_hsv(img Image) yikes<Image> {
    ready (img.channels < 3) {
        yikes "HSV conversion requires at least 3 channels"
    }
    
    sus result Image = create_image(img.width, img.height, 3)
    result.format = img.format
    result.color_space = "HSV"
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus pixel []drip = get_pixel(img, x, y) fam {
                when _ -> yikes "HSV conversion pixel access failed"
            }
            
            sus rgb RGB = RGB{r: pixel[0], g: pixel[1], b: pixel[2]}
            sus hsv HSV = rgb_to_hsv_precise(rgb)
            
            # Encode HSV as RGB values for storage
            sus h_scaled drip = drip(hsv.h * 255.0 / 360.0)
            sus s_scaled drip = drip(hsv.s * 255.0)
            sus v_scaled drip = drip(hsv.v * 255.0)
            
            sus hsv_pixel []drip = [
                clamp_pixel_value(h_scaled),
                clamp_pixel_value(s_scaled),
                clamp_pixel_value(v_scaled)
            ]
            
            set_pixel(&result, x, y, hsv_pixel) fam {
                when _ -> yikes "HSV conversion pixel set failed"
            }
        }
    }
    
    damn result
}

# Convert image to LAB color space
slay convert_to_lab_image(img Image) yikes<Image> {
    ready (img.channels < 3) {
        yikes "LAB conversion requires at least 3 channels"
    }
    
    sus result Image = create_image(img.width, img.height, 3)
    result.format = img.format
    result.color_space = "LAB"
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus pixel []drip = get_pixel(img, x, y) fam {
                when _ -> yikes "LAB conversion pixel access failed"
            }
            
            sus rgb RGB = RGB{r: pixel[0], g: pixel[1], b: pixel[2]}
            sus lab LAB = rgb_to_lab(rgb)
            
            # Encode LAB as RGB values for storage
            # L: 0-100 -> 0-255, a: -128-127 -> 0-255, b: -128-127 -> 0-255
            sus l_scaled drip = clamp_pixel_value(drip(lab.l * 255.0 / 100.0))
            sus a_scaled drip = clamp_pixel_value(drip((lab.a + 128.0) * 255.0 / 255.0))
            sus b_scaled drip = clamp_pixel_value(drip((lab.b + 128.0) * 255.0 / 255.0))
            
            sus lab_pixel []drip = [l_scaled, a_scaled, b_scaled]
            
            set_pixel(&result, x, y, lab_pixel) fam {
                when _ -> yikes "LAB conversion pixel set failed"
            }
        }
    }
    
    damn result
}

# Convert image to CMYK color space
slay convert_to_cmyk(img Image) yikes<Image> {
    ready (img.channels < 3) {
        yikes "CMYK conversion requires at least 3 channels"
    }
    
    sus result Image = create_image(img.width, img.height, 4)
    result.format = img.format
    result.color_space = "CMYK"
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus pixel []drip = get_pixel(img, x, y) fam {
                when _ -> yikes "CMYK conversion pixel access failed"
            }
            
            sus rgb RGB = RGB{r: pixel[0], g: pixel[1], b: pixel[2]}
            sus cmyk []tea = rgb_to_cmyk(rgb)
            
            # Scale CMYK values to 0-255 range
            sus cmyk_pixel []drip = [
                clamp_pixel_value(drip(cmyk[0] * 255.0)),
                clamp_pixel_value(drip(cmyk[1] * 255.0)),
                clamp_pixel_value(drip(cmyk[2] * 255.0)),
                clamp_pixel_value(drip(cmyk[3] * 255.0))
            ]
            
            set_pixel(&result, x, y, cmyk_pixel) fam {
                when _ -> yikes "CMYK conversion pixel set failed"
            }
        }
    }
    
    damn result
}

# Color temperature adjustment (warm/cool)
slay adjust_color_temperature(img Image, temperature drip) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (img.channels < 3) {
        yikes "color temperature adjustment requires at least 3 channels"
    }
    
    sus result Image = clone_image(img)
    
    # Calculate temperature adjustment factors
    sus temp_factor tea = temperature / 100.0
    sus red_factor tea = ready (temp_factor > 0.0) {
        damn 1.0 + temp_factor * 0.3
    } otherwise {
        damn 1.0
    }
    sus blue_factor tea = ready (temp_factor < 0.0) {
        damn 1.0 - temp_factor * 0.3
    } otherwise {
        damn 1.0
    }
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus pixel []drip = get_pixel(img, x, y) fam {
                when _ -> yikes "temperature adjustment pixel access failed"
            }
            
            sus adjusted_pixel []drip = []
            
            # Adjust red channel
            sus new_red drip = clamp_pixel_value(drip(pixel[0] * red_factor))
            adjusted_pixel = append(adjusted_pixel, new_red)
            
            # Keep green channel unchanged
            adjusted_pixel = append(adjusted_pixel, pixel[1])
            
            # Adjust blue channel
            sus new_blue drip = clamp_pixel_value(drip(pixel[2] * blue_factor))
            adjusted_pixel = append(adjusted_pixel, new_blue)
            
            # Preserve alpha channel if present
            ready (img.channels == 4) {
                adjusted_pixel = append(adjusted_pixel, pixel[3])
            }
            
            set_pixel(&result, x, y, adjusted_pixel) fam {
                when _ -> yikes "temperature adjustment pixel set failed"
            }
        }
    }
    
    damn result
}

# White balance correction
slay white_balance(img Image, red_gain tea, green_gain tea, blue_gain tea) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (img.channels < 3) {
        yikes "white balance requires at least 3 channels"
    }
    
    ready (red_gain <= 0.0 || green_gain <= 0.0 || blue_gain <= 0.0) {
        yikes "invalid white balance gains"
    }
    
    sus result Image = clone_image(img)
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus pixel []drip = get_pixel(img, x, y) fam {
                when _ -> yikes "white balance pixel access failed"
            }
            
            sus balanced_pixel []drip = [
                clamp_pixel_value(drip(pixel[0] * red_gain)),
                clamp_pixel_value(drip(pixel[1] * green_gain)),
                clamp_pixel_value(drip(pixel[2] * blue_gain))
            ]
            
            # Preserve alpha channel if present
            ready (img.channels == 4) {
                balanced_pixel = append(balanced_pixel, pixel[3])
            }
            
            set_pixel(&result, x, y, balanced_pixel) fam {
                when _ -> yikes "white balance pixel set failed"
            }
        }
    }
    
    damn result
}
