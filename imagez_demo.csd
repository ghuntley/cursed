# imagez Package Demo - Comprehensive Image Processing
# Demonstrates all major features of the imagez stdlib package

yeet "vibez"
yeet "stdlib/imagez"

slay main() lit {
    vibez.spill("🎨 CURSED imagez Package Demo")
    vibez.spill("====================================")
    vibez.spill("")
    
    # Demo 1: Basic Image Creation and Manipulation
    demo_basic_image_operations()
    vibez.spill("")
    
    # Demo 2: Color Space Conversions
    demo_color_space_conversions()
    vibez.spill("")
    
    # Demo 3: Image Filters and Effects
    demo_filters_and_effects()
    vibez.spill("")
    
    # Demo 4: Advanced Image Processing
    demo_advanced_processing()
    vibez.spill("")
    
    # Demo 5: Performance and Memory Safety
    demo_performance_features()
    vibez.spill("")
    
    # Run comprehensive test suite
    vibez.spill("🧪 Running comprehensive test suite...")
    test_imagez()
    
    vibez.spill("")
    vibez.spill("✅ imagez package demo completed successfully!")
}

slay demo_basic_image_operations() lit {
    vibez.spill("📸 Demo 1: Basic Image Operations")
    vibez.spill("----------------------------------")
    
    # Create a new RGB image
    sus img Image = create_image(100, 100, 3)
    vibez.spill("✓ Created 100x100 RGB image")
    
    # Set some pixels with different colors
    set_pixel(&img, 25, 25, [255, 0, 0]) fam {    # Red
        when e -> vibez.spill("Error setting red pixel: " + e)
    }
    
    set_pixel(&img, 50, 50, [0, 255, 0]) fam {    # Green  
        when e -> vibez.spill("Error setting green pixel: " + e)
    }
    
    set_pixel(&img, 75, 75, [0, 0, 255]) fam {    # Blue
        when e -> vibez.spill("Error setting blue pixel: " + e)
    }
    
    vibez.spill("✓ Set RGB pixels at different positions")
    
    # Read back pixels
    sus red_pixel []drip = get_pixel(img, 25, 25) fam {
        when e -> {
            vibez.spill("Error reading red pixel: " + e)
            damn []
        }
    }
    
    vibez.spill("✓ Read red pixel: [" + string(red_pixel[0]) + ", " + string(red_pixel[1]) + ", " + string(red_pixel[2]) + "]")
    
    # Image validation
    validate_image(img) fam {
        when e -> vibez.spill("Image validation failed: " + e)
    } otherwise {
        vibez.spill("✓ Image validation passed")
    }
    
    # Get image info
    sus info tea = get_image_info(img)
    vibez.spill("✓ " + info)
    
    # Clone image
    sus cloned Image = clone_image(img)
    vibez.spill("✓ Image cloned successfully")
    
    # Create RGBA image
    sus rgba_img Image = create_image(50, 50, 4)
    set_pixel(&rgba_img, 25, 25, [255, 128, 64, 200]) fam {
        when e -> vibez.spill("Error setting RGBA pixel: " + e)
    }
    vibez.spill("✓ Created and populated RGBA image")
}

slay demo_color_space_conversions() lit {
    vibez.spill("🌈 Demo 2: Color Space Conversions")
    vibez.spill("-----------------------------------")
    
    # RGB to HSV conversion
    sus rgb RGB = RGB{r: 255, g: 128, b: 64}
    sus hsv HSV = rgb_to_hsv_precise(rgb)
    vibez.spill("✓ RGB(255,128,64) -> HSV(" + string(hsv.h) + "°, " + string(hsv.s) + ", " + string(hsv.v) + ")")
    
    # HSV back to RGB
    sus back_rgb RGB = hsv_to_rgb_precise(hsv)
    vibez.spill("✓ HSV -> RGB(" + string(back_rgb.r) + ", " + string(back_rgb.g) + ", " + string(back_rgb.b) + ")")
    
    # RGB to LAB conversion
    sus lab LAB = rgb_to_lab(rgb)
    vibez.spill("✓ RGB -> LAB(L=" + string(lab.l) + ", a=" + string(lab.a) + ", b=" + string(lab.b) + ")")
    
    # RGB to CMYK conversion
    sus cmyk []tea = rgb_to_cmyk(rgb)
    vibez.spill("✓ RGB -> CMYK(C=" + string(cmyk[0]) + ", M=" + string(cmyk[1]) + ", Y=" + string(cmyk[2]) + ", K=" + string(cmyk[3]) + ")")
    
    # Image color space conversions
    sus test_img Image = create_image(20, 20, 3)
    
    # Fill with gradient colors
    bestie (y drip = 0; y < 20; y = y + 1) {
        bestie (x drip = 0; x < 20; x = x + 1) {
            sus r drip = x * 12
            sus g drip = y * 12  
            sus b drip = (x + y) * 6
            set_pixel(&test_img, x, y, [r, g, b]) fam { when _ -> nah }
        }
    }
    
    vibez.spill("✓ Created test image with color gradient")
    
    # Convert to grayscale
    sus gray_img Image = convert_to_grayscale(test_img) fam {
        when e -> {
            vibez.spill("Grayscale conversion failed: " + e)
            damn
        }
    }
    vibez.spill("✓ Converted to grayscale: " + get_image_info(gray_img))
    
    # Convert to RGBA  
    sus rgba_img Image = convert_to_rgba(test_img) fam {
        when e -> {
            vibez.spill("RGBA conversion failed: " + e)
            damn
        }
    }
    vibez.spill("✓ Converted to RGBA: " + get_image_info(rgba_img))
    
    # Convert to HSV image
    sus hsv_img Image = convert_image_colorspace(test_img, "HSV") fam {
        when e -> {
            vibez.spill("HSV conversion failed: " + e)
            damn
        }
    }
    vibez.spill("✓ Converted to HSV: " + get_image_info(hsv_img))
}

slay demo_filters_and_effects() lit {
    vibez.spill("🎭 Demo 3: Filters and Effects")
    vibez.spill("-------------------------------")
    
    # Create test image with pattern
    sus test_img Image = create_image(50, 50, 3)
    
    # Create checkerboard pattern
    bestie (y drip = 0; y < 50; y = y + 1) {
        bestie (x drip = 0; x < 50; x = x + 1) {
            sus checker lit = ((x / 10) % 2) == ((y / 10) % 2)
            sus color drip = ready (checker) { damn 255 } otherwise { damn 0 }
            set_pixel(&test_img, x, y, [color, color, color]) fam { when _ -> nah }
        }
    }
    vibez.spill("✓ Created 50x50 checkerboard test image")
    
    # Apply blur filter
    sus blurred Image = apply_filter(test_img, BLUR_3X3) fam {
        when e -> {
            vibez.spill("Blur filter failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Applied 3x3 blur filter")
    
    # Apply sharpen filter
    sus sharpened Image = apply_filter(test_img, SHARPEN_3X3) fam {
        when e -> {
            vibez.spill("Sharpen filter failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Applied 3x3 sharpen filter")
    
    # Gaussian blur
    sus gaussian_blurred Image = gaussian_blur(test_img, 2.0) fam {
        when e -> {
            vibez.spill("Gaussian blur failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Applied Gaussian blur (σ=2.0)")
    
    # Box blur
    sus box_blurred Image = box_blur(test_img, 3) fam {
        when e -> {
            vibez.spill("Box blur failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Applied box blur (radius=3)")
    
    # Edge detection
    sus edges Image = sobel_edge_detection(test_img) fam {
        when e -> {
            vibez.spill("Edge detection failed: " + e)
            damn create_image(1, 1, 1)
        }
    }
    vibez.spill("✓ Applied Sobel edge detection: " + get_image_info(edges))
    
    # Motion blur
    sus motion_blurred Image = motion_blur(test_img, 45.0, 5) fam {
        when e -> {
            vibez.spill("Motion blur failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Applied motion blur (45°, distance=5)")
    
    # Preset filters
    sus embossed Image = apply_preset_filter(test_img, "EMBOSS") fam {
        when e -> {
            vibez.spill("Emboss filter failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Applied emboss preset filter")
    
    # Image adjustments
    sus brightened Image = adjust_brightness(test_img, 50) fam {
        when e -> {
            vibez.spill("Brightness adjustment failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Adjusted brightness (+50)")
    
    sus contrasted Image = adjust_contrast(test_img, 25.0) fam {
        when e -> {
            vibez.spill("Contrast adjustment failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Adjusted contrast (+25)")
    
    sus gamma_corrected Image = adjust_gamma(test_img, 0.8) fam {
        when e -> {
            vibez.spill("Gamma correction failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Applied gamma correction (γ=0.8)")
}

slay demo_advanced_processing() lit {
    vibez.spill("🔬 Demo 4: Advanced Image Processing")
    vibez.spill("------------------------------------")
    
    # Create color test image  
    sus color_img Image = create_image(30, 30, 3)
    
    # Fill with color gradient
    bestie (y drip = 0; y < 30; y = y + 1) {
        bestie (x drip = 0; x < 30; x = x + 1) {
            sus r drip = (x * 255) / 29
            sus g drip = (y * 255) / 29
            sus b drip = ((x + y) * 255) / 58
            set_pixel(&color_img, x, y, [r, g, b]) fam { when _ -> nah }
        }
    }
    vibez.spill("✓ Created 30x30 color gradient image")
    
    # Resize with different interpolation methods
    sus nearest_resized Image = resize_image(color_img, 60, 60, "NEAREST") fam {
        when e -> {
            vibez.spill("Nearest resize failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Resized with nearest neighbor (30x30 -> 60x60)")
    
    sus bilinear_resized Image = resize_image(color_img, 15, 15, "BILINEAR") fam {
        when e -> {
            vibez.spill("Bilinear resize failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Resized with bilinear interpolation (30x30 -> 15x15)")
    
    sus bicubic_resized Image = resize_image(color_img, 45, 45, "BICUBIC") fam {
        when e -> {
            vibez.spill("Bicubic resize failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Resized with bicubic interpolation (30x30 -> 45x45)")
    
    # Scaling
    sus scaled_img Image = scale_image(color_img, 0.5, 0.5, "BILINEAR") fam {
        when e -> {
            vibez.spill("Image scaling failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Scaled image by 0.5x: " + get_image_info(scaled_img))
    
    # Cropping
    sus cropped Image = crop_image(color_img, 10, 10, 10, 10) fam {
        when e -> {
            vibez.spill("Image cropping failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Cropped 10x10 region: " + get_image_info(cropped))
    
    # Rotation
    sus rotated Image = rotate_image(color_img, 45.0) fam {
        when e -> {
            vibez.spill("Image rotation failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Rotated image 45°: " + get_image_info(rotated))
    
    # Flipping
    sus h_flipped Image = flip_horizontal(color_img) fam {
        when e -> {
            vibez.spill("Horizontal flip failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Horizontally flipped image")
    
    sus v_flipped Image = flip_vertical(color_img) fam {
        when e -> {
            vibez.spill("Vertical flip failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Vertically flipped image")
    
    # Channel extraction
    sus red_channel Image = extract_channel(color_img, 0) fam {
        when e -> {
            vibez.spill("Red channel extraction failed: " + e)
            damn create_image(1, 1, 1)
        }
    }
    vibez.spill("✓ Extracted red channel: " + get_image_info(red_channel))
    
    # Color temperature adjustment
    sus warmer Image = adjust_color_temperature(color_img, 15) fam {
        when e -> {
            vibez.spill("Color temperature adjustment failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Adjusted color temperature (+15)")
    
    # White balance
    sus balanced Image = white_balance(color_img, 1.1, 1.0, 0.9) fam {
        when e -> {
            vibez.spill("White balance failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Applied white balance (1.1, 1.0, 0.9)")
    
    # Unsharp mask sharpening
    sus sharp Image = unsharp_mask(color_img, 1.0, 1.5, 5) fam {
        when e -> {
            vibez.spill("Unsharp mask failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Applied unsharp mask sharpening")
    
    # Median filter
    sus denoised Image = median_filter(color_img, 1) fam {
        when e -> {
            vibez.spill("Median filter failed: " + e)
            damn create_image(1, 1, 3)
        }
    }
    vibez.spill("✓ Applied median filter (radius=1)")
}

slay demo_performance_features() lit {
    vibez.spill("⚡ Demo 5: Performance and Memory Safety")
    vibez.spill("----------------------------------------")
    
    # Large image handling
    sus large_img Image = create_image(500, 500, 3)
    vibez.spill("✓ Created large 500x500 image (" + string(len(large_img.data)) + " bytes)")
    
    # Memory safety - bounds checking
    sus out_of_bounds_result []drip = get_pixel(large_img, 1000, 1000) fam {
        when e -> {
            vibez.spill("✓ Bounds checking caught out-of-bounds access: " + e)
            damn []
        }
    }
    
    # Validation of invalid image
    sus invalid_img Image = Image{
        width: 0,
        height: 100, 
        channels: 3,
        data: [],
        format: "RGB",
        color_space: "RGB"
    }
    
    validate_image(invalid_img) fam {
        when e -> {
            vibez.spill("✓ Image validation caught invalid image: " + e)
        }
    }
    
    # Performance test - multiple operations on large image
    sus start_time drip = get_time_ms()  # Simulated timing
    
    sus processed Image = large_img
    
    # Chain of operations
    processed = resize_image(processed, 400, 400, "BILINEAR") fam {
        when e -> {
            vibez.spill("Performance test resize failed: " + e) 
            damn processed
        }
    }
    
    processed = gaussian_blur(processed, 1.0) fam {
        when e -> {
            vibez.spill("Performance test blur failed: " + e)
            damn processed
        }
    }
    
    processed = adjust_brightness(processed, 10) fam {
        when e -> {
            vibez.spill("Performance test brightness failed: " + e)
            damn processed
        }
    }
    
    sus end_time drip = get_time_ms()  # Simulated timing
    sus elapsed drip = end_time - start_time
    
    vibez.spill("✓ Performance test completed:")
    vibez.spill("  - Resized 500x500 -> 400x400")
    vibez.spill("  - Applied Gaussian blur")  
    vibez.spill("  - Adjusted brightness")
    vibez.spill("  - Final result: " + get_image_info(processed))
    
    # Memory usage demonstration
    sus memory_usage drip = len(large_img.data) + len(processed.data)
    vibez.spill("✓ Memory usage: ~" + string(memory_usage / 1024) + " KB")
    
    # Format detection
    sus png_format tea = detect_format_from_extension("test.png") fam {
        when e -> {
            vibez.spill("Format detection failed: " + e)
            damn "UNKNOWN"
        }
    }
    vibez.spill("✓ Format detection: test.png -> " + png_format)
    
    sus jpg_format tea = detect_format_from_extension("photo.jpg") fam {
        when e -> {
            vibez.spill("Format detection failed: " + e)
            damn "UNKNOWN"
        }
    }
    vibez.spill("✓ Format detection: photo.jpg -> " + jpg_format)
    
    # Error handling demonstration
    sus error_result Image = resize_image(large_img, -10, -10, "INVALID") fam {
        when e -> {
            vibez.spill("✓ Error handling caught invalid parameters: " + e)
            damn create_image(1, 1, 3)
        }
    }
    
    vibez.spill("✓ All performance and safety features demonstrated")
}

# Simulated timing function (would use actual timing in real implementation)
slay get_time_ms() drip {
    damn 1000  # Placeholder
}

# Run the demo
main()
