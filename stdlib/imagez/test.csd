# imagez - Comprehensive Image Processing Tests
# Pure CURSED test suite for all image processing functionality

yeet "../testz"
yeet "../vibez"
yeet "./core"
yeet "./formats"
yeet "./manipulation"
yeet "./filters"
yeet "./colorspace"

# Test counter and results
sus test_count drip = 0
sus passed_count drip = 0
sus failed_count drip = 0

slay run_imagez_tests() lit {
    vibez.spill("=== CURSED imagez Module Test Suite ===")
    vibez.spill("")
    
    test_count = 0
    passed_count = 0
    failed_count = 0
    
    # Core functionality tests
    test_image_creation()
    test_pixel_operations()
    test_image_validation()
    test_color_structures()
    
    # Color space conversion tests
    test_rgb_hsv_conversion()
    test_rgb_lab_conversion()
    test_rgb_cmyk_conversion()
    test_colorspace_image_conversion()
    
    # Image manipulation tests
    test_image_resizing()
    test_image_cropping()
    test_image_rotation()
    test_image_flipping()
    
    # Filter tests
    test_convolution_filters()
    test_gaussian_blur()
    test_edge_detection()
    test_image_adjustments()
    
    # Format detection tests (without actual file I/O)
    test_format_detection()
    test_metadata_handling()
    
    # Performance and edge case tests
    test_large_image_handling()
    test_error_handling()
    test_memory_safety()
    
    vibez.spill("")
    vibez.spill("=== Test Results ===")
    vibez.spill("Total tests: " + string(test_count))
    vibez.spill("Passed: " + string(passed_count))
    vibez.spill("Failed: " + string(failed_count))
    
    ready (failed_count == 0) {
        vibez.spill("✅ All imagez tests PASSED!")
    } otherwise {
        vibez.spill("❌ " + string(failed_count) + " tests FAILED")
    }
    
    damn based
}

# Test helper function
slay run_test(test_name tea, test_func slay() yikes<lit>) lit {
    test_count = test_count + 1
    vibez.spill("Running: " + test_name)
    
    test_func() fam {
        when error -> {
            vibez.spill("  ❌ FAILED: " + error)
            failed_count = failed_count + 1
        }
    } otherwise {
        vibez.spill("  ✅ PASSED")
        passed_count = passed_count + 1
    }
    
    damn based
}

# Core functionality tests
slay test_image_creation() lit {
    run_test("Image Creation", slay() yikes<lit> {
        sus img Image = create_image(100, 100, 3)
        
        ready (img.width != 100 || img.height != 100 || img.channels != 3) {
            yikes "incorrect image dimensions or channels"
        }
        
        ready (len(img.data) != 100 * 100 * 3) {
            yikes "incorrect data size"
        }
        
        ready (img.color_space != "RGB") {
            yikes "incorrect default color space"
        }
        
        damn based
    })
    
    run_test("RGBA Image Creation", slay() yikes<lit> {
        sus img Image = create_image(50, 75, 4)
        
        ready (img.channels != 4) {
            yikes "incorrect RGBA channels"
        }
        
        ready (img.color_space != "RGBA") {
            yikes "incorrect RGBA color space"
        }
        
        damn based
    })
    
    damn based
}

slay test_pixel_operations() lit {
    run_test("Pixel Get/Set Operations", slay() yikes<lit> {
        sus img Image = create_image(10, 10, 3)
        sus test_pixel drip[value] = [255, 128, 64]
        
        # Set pixel
        set_pixel(&img, 5, 5, test_pixel) fam {
            when e -> yikes e
        }
        
        # Get pixel
        sus retrieved_pixel drip[value] = get_pixel(img, 5, 5) fam {
            when e -> yikes e
        }
        
        ready (len(retrieved_pixel) != 3) {
            yikes "retrieved pixel has wrong channel count"
        }
        
        bestie (i drip = 0; i < 3; i = i + 1) {
            ready (retrieved_pixel[i] != test_pixel[i]) {
                yikes "pixel values don't match"
            }
        }
        
        damn based
    })
    
    run_test("Bounds Checking", slay() yikes<lit> {
        sus img Image = create_image(5, 5, 3)
        
        # Test out of bounds access
        sus result drip[value] = get_pixel(img, 10, 10) fam {
            when _ -> damn []  # Expected to fail
        }
        
        ready (len(result) > 0) {
            yikes "bounds checking failed - should have failed"
        }
        
        damn based
    })
    
    damn based
}

slay test_image_validation() lit {
    run_test("Image Validation", slay() yikes<lit> {
        # Valid image
        sus valid_img Image = create_image(10, 10, 3)
        validate_image(valid_img) fam {
            when e -> yikes "valid image failed validation: " + e
        }
        
        # Invalid dimensions
        sus invalid_img Image = Image{
            width: 0,
            height: 10,
            channels: 3,
            data: [],
            format: "RGB",
            color_space: "RGB"
        }
        
        sus validation_result lit = based
        validate_image(invalid_img) fam {
            when _ -> validation_result = nah  # Expected to fail
        }
        
        ready (validation_result) {
            yikes "invalid image passed validation"
        }
        
        damn based
    })
    
    damn based
}

slay test_color_structures() lit {
    run_test("Color Structure Operations", slay() yikes<lit> {
        sus rgb RGB = RGB{r: 255, g: 128, b: 64}
        sus rgba RGBA = RGBA{r: 255, g: 128, b: 64, a: 255}
        sus hsv HSV = HSV{h: 180.0, s: 0.5, v: 1.0}
        sus lab LAB = LAB{l: 50.0, a: 25.0, b: -25.0}
        
        # Basic structure validation
        ready (rgb.r != 255 || rgb.g != 128 || rgb.b != 64) {
            yikes "RGB structure values incorrect"
        }
        
        ready (rgba.a != 255) {
            yikes "RGBA alpha value incorrect"
        }
        
        ready (hsv.h != 180.0 || hsv.s != 0.5 || hsv.v != 1.0) {
            yikes "HSV structure values incorrect"
        }
        
        ready (lab.l != 50.0) {
            yikes "LAB structure values incorrect"
        }
        
        damn based
    })
    
    damn based
}

# Color space conversion tests
slay test_rgb_hsv_conversion() lit {
    run_test("RGB to HSV Conversion", slay() yikes<lit> {
        sus rgb RGB = RGB{r: 255, g: 0, b: 0}  # Pure red
        sus hsv HSV = rgb_to_hsv_precise(rgb)
        
        # Red should have hue ~0, saturation 1, value 1
        ready (hsv.h < -1.0 || hsv.h > 1.0) {  # Allow small tolerance
            yikes "red hue conversion incorrect: " + string(hsv.h)
        }
        
        ready (hsv.s < 0.99 || hsv.s > 1.01) {
            yikes "red saturation conversion incorrect: " + string(hsv.s)
        }
        
        ready (hsv.v < 0.99 || hsv.v > 1.01) {
            yikes "red value conversion incorrect: " + string(hsv.v)
        }
        
        damn based
    })
    
    run_test("HSV to RGB Conversion", slay() yikes<lit> {
        sus hsv HSV = HSV{h: 120.0, s: 1.0, v: 1.0}  # Pure green
        sus rgb RGB = hsv_to_rgb_precise(hsv)
        
        # Should result in pure green (0, 255, 0)
        ready (rgb.r > 5) {  # Allow small tolerance for rounding
            yikes "green red component incorrect: " + string(rgb.r)
        }
        
        ready (rgb.g < 250 || rgb.g > 255) {
            yikes "green green component incorrect: " + string(rgb.g)
        }
        
        ready (rgb.b > 5) {
            yikes "green blue component incorrect: " + string(rgb.b)
        }
        
        damn based
    })
    
    run_test("RGB-HSV Round Trip", slay() yikes<lit> {
        sus original RGB = RGB{r: 128, g: 64, b: 192}
        sus hsv HSV = rgb_to_hsv_precise(original)
        sus converted RGB = hsv_to_rgb_precise(hsv)
        
        # Allow small tolerance for rounding errors
        sus tolerance drip = 2
        
        ready (abs(converted.r - original.r) > tolerance) {
            yikes "round trip red component error"
        }
        
        ready (abs(converted.g - original.g) > tolerance) {
            yikes "round trip green component error"
        }
        
        ready (abs(converted.b - original.b) > tolerance) {
            yikes "round trip blue component error"
        }
        
        damn based
    })
    
    damn based
}

slay test_rgb_lab_conversion() lit {
    run_test("RGB to LAB Conversion", slay() yikes<lit> {
        sus white RGB = RGB{r: 255, g: 255, b: 255}
        sus lab LAB = rgb_to_lab(white)
        
        # White should have high L value, near-zero a and b
        ready (lab.l < 95.0 || lab.l > 105.0) {
            yikes "white LAB L component incorrect: " + string(lab.l)
        }
        
        ready (abs_tea(lab.a) > 5.0 || abs_tea(lab.b) > 5.0) {
            yikes "white LAB a/b components should be near zero"
        }
        
        damn based
    })
    
    run_test("LAB to RGB Conversion", slay() yikes<lit> {
        sus lab LAB = LAB{l: 0.0, a: 0.0, b: 0.0}  # Black
        sus rgb RGB = lab_to_rgb(lab)
        
        # Should result in black or very dark
        ready (rgb.r > 10 || rgb.g > 10 || rgb.b > 10) {
            yikes "black LAB conversion incorrect"
        }
        
        damn based
    })
    
    damn based
}

slay test_rgb_cmyk_conversion() lit {
    run_test("RGB to CMYK Conversion", slay() yikes<lit> {
        sus black RGB = RGB{r: 0, g: 0, b: 0}
        sus cmyk tea[value] = rgb_to_cmyk(black)
        
        ready (len(cmyk) != 4) {
            yikes "CMYK conversion wrong number of components"
        }
        
        # Black should have K=1, CMY=0
        ready (cmyk[3] < 0.99) {
            yikes "black K component should be ~1.0"
        }
        
        damn based
    })
    
    run_test("CMYK to RGB Conversion", slay() yikes<lit> {
        sus cmyk tea[value] = [0.0, 0.0, 0.0, 0.0]  # White
        sus rgb RGB = cmyk_to_rgb(cmyk)
        
        # Should result in white
        ready (rgb.r < 250 || rgb.g < 250 || rgb.b < 250) {
            yikes "white CMYK conversion incorrect"
        }
        
        damn based
    })
    
    damn based
}

slay test_colorspace_image_conversion() lit {
    run_test("Image Grayscale Conversion", slay() yikes<lit> {
        sus img Image = create_image(10, 10, 3)
        
        # Fill with known RGB values
        sus red_pixel drip[value] = [255, 0, 0]
        set_pixel(&img, 5, 5, red_pixel) fam {
            when e -> yikes e
        }
        
        sus gray_img Image = convert_to_grayscale(img) fam {
            when e -> yikes e
        }
        
        ready (gray_img.channels != 1) {
            yikes "grayscale image should have 1 channel"
        }
        
        ready (gray_img.color_space != "GRAYSCALE") {
            yikes "grayscale color space incorrect"
        }
        
        sus gray_pixel drip[value] = get_pixel(gray_img, 5, 5) fam {
            when e -> yikes e
        }
        
        # Red pixel should convert to some gray value
        ready (len(gray_pixel) != 1) {
            yikes "grayscale pixel should have 1 component"
        }
        
        damn based
    })
    
    damn based
}

# Image manipulation tests
slay test_image_resizing() lit {
    run_test("Nearest Neighbor Resize", slay() yikes<lit> {
        sus img Image = create_image(2, 2, 3)
        sus resized Image = resize_image(img, 4, 4, "NEAREST") fam {
            when e -> yikes e
        }
        
        ready (resized.width != 4 || resized.height != 4) {
            yikes "resized dimensions incorrect"
        }
        
        ready (resized.channels != 3) {
            yikes "resized channels incorrect"
        }
        
        damn based
    })
    
    run_test("Bilinear Resize", slay() yikes<lit> {
        sus img Image = create_image(3, 3, 3)
        sus resized Image = resize_image(img, 6, 6, "BILINEAR") fam {
            when e -> yikes e
        }
        
        ready (resized.width != 6 || resized.height != 6) {
            yikes "bilinear resized dimensions incorrect"
        }
        
        damn based
    })
    
    damn based
}

slay test_image_cropping() lit {
    run_test("Image Cropping", slay() yikes<lit> {
        sus img Image = create_image(10, 10, 3)
        
        # Set a known pixel
        sus test_pixel drip[value] = [123, 45, 67]
        set_pixel(&img, 7, 8, test_pixel) fam {
            when e -> yikes e
        }
        
        # Crop region that includes the test pixel
        sus cropped Image = crop_image(img, 5, 5, 5, 5) fam {
            when e -> yikes e
        }
        
        ready (cropped.width != 5 || cropped.height != 5) {
            yikes "cropped dimensions incorrect"
        }
        
        # Test pixel should now be at (2, 3) in cropped image
        sus cropped_pixel drip[value] = get_pixel(cropped, 2, 3) fam {
            when e -> yikes e
        }
        
        bestie (i drip = 0; i < 3; i = i + 1) {
            ready (cropped_pixel[i] != test_pixel[i]) {
                yikes "cropped pixel values incorrect"
            }
        }
        
        damn based
    })
    
    damn based
}

slay test_image_rotation() lit {
    run_test("Image Rotation 90 degrees", slay() yikes<lit> {
        sus img Image = create_image(4, 4, 3)
        sus rotated Image = rotate_image(img, 90.0) fam {
            when e -> yikes e
        }
        
        # After 90 degree rotation, dimensions should be swapped approximately
        ready (rotated.width <= 0 || rotated.height <= 0) {
            yikes "rotated image has invalid dimensions"
        }
        
        damn based
    })
    
    damn based
}

slay test_image_flipping() lit {
    run_test("Horizontal Flip", slay() yikes<lit> {
        sus img Image = create_image(5, 3, 3)
        
        # Set pixel at left side
        sus test_pixel drip[value] = [255, 128, 64]
        set_pixel(&img, 0, 1, test_pixel) fam {
            when e -> yikes e
        }
        
        sus flipped Image = flip_horizontal(img) fam {
            when e -> yikes e
        }
        
        # Pixel should now be on the right side
        sus flipped_pixel drip[value] = get_pixel(flipped, 4, 1) fam {
            when e -> yikes e
        }
        
        bestie (i drip = 0; i < 3; i = i + 1) {
            ready (flipped_pixel[i] != test_pixel[i]) {
                yikes "horizontally flipped pixel incorrect"
            }
        }
        
        damn based
    })
    
    run_test("Vertical Flip", slay() yikes<lit> {
        sus img Image = create_image(3, 5, 3)
        
        # Set pixel at top
        sus test_pixel drip[value] = [200, 100, 50]
        set_pixel(&img, 1, 0, test_pixel) fam {
            when e -> yikes e
        }
        
        sus flipped Image = flip_vertical(img) fam {
            when e -> yikes e
        }
        
        # Pixel should now be at bottom
        sus flipped_pixel drip[value] = get_pixel(flipped, 1, 4) fam {
            when e -> yikes e
        }
        
        bestie (i drip = 0; i < 3; i = i + 1) {
            ready (flipped_pixel[i] != test_pixel[i]) {
                yikes "vertically flipped pixel incorrect"
            }
        }
        
        damn based
    })
    
    damn based
}

# Filter tests
slay test_convolution_filters() lit {
    run_test("Blur Filter", slay() yikes<lit> {
        sus img Image = create_image(5, 5, 3)
        
        # Fill with white
        fill_image(&img, [255, 255, 255]) fam {
            when e -> yikes e
        }
        
        sus blurred Image = apply_filter(img, BLUR_3X3) fam {
            when e -> yikes e
        }
        
        ready (blurred.width != img.width || blurred.height != img.height) {
            yikes "blur filter changed image dimensions"
        }
        
        damn based
    })
    
    run_test("Sharpen Filter", slay() yikes<lit> {
        sus img Image = create_image(5, 5, 3)
        sus sharpened Image = apply_filter(img, SHARPEN_3X3) fam {
            when e -> yikes e
        }
        
        ready (sharpened.width != img.width || sharpened.height != img.height) {
            yikes "sharpen filter changed image dimensions"
        }
        
        damn based
    })
    
    damn based
}

slay test_gaussian_blur() lit {
    run_test("Gaussian Blur", slay() yikes<lit> {
        sus img Image = create_image(10, 10, 3)
        sus blurred Image = gaussian_blur(img, 1.0) fam {
            when e -> yikes e
        }
        
        ready (blurred.width != img.width || blurred.height != img.height) {
            yikes "gaussian blur changed image dimensions"
        }
        
        damn based
    })
    
    run_test("Box Blur", slay() yikes<lit> {
        sus img Image = create_image(7, 7, 3)
        sus blurred Image = box_blur(img, 1) fam {
            when e -> yikes e
        }
        
        ready (blurred.width != img.width || blurred.height != img.height) {
            yikes "box blur changed image dimensions"
        }
        
        damn based
    })
    
    damn based
}

slay test_edge_detection() lit {
    run_test("Sobel Edge Detection", slay() yikes<lit> {
        sus img Image = create_image(10, 10, 3)
        
        # Create a simple edge pattern
        bestie (x drip = 0; x < 5; x = x + 1) {
            bestie (y drip = 0; y < 10; y = y + 1) {
                set_pixel(&img, x, y, [0, 0, 0]) fam { when _ -> nah }
            }
        }
        bestie (x drip = 5; x < 10; x = x + 1) {
            bestie (y drip = 0; y < 10; y = y + 1) {
                set_pixel(&img, x, y, [255, 255, 255]) fam { when _ -> nah }
            }
        }
        
        sus edges Image = sobel_edge_detection(img) fam {
            when e -> yikes e
        }
        
        ready (edges.channels != 1) {
            yikes "edge detection should produce grayscale image"
        }
        
        damn based
    })
    
    damn based
}

slay test_image_adjustments() lit {
    run_test("Brightness Adjustment", slay() yikes<lit> {
        sus img Image = create_image(3, 3, 3)
        
        # Set a known pixel
        sus original_pixel drip[value] = [128, 128, 128]
        set_pixel(&img, 1, 1, original_pixel) fam {
            when e -> yikes e
        }
        
        sus brightened Image = adjust_brightness(img, 50) fam {
            when e -> yikes e
        }
        
        sus adjusted_pixel drip[value] = get_pixel(brightened, 1, 1) fam {
            when e -> yikes e
        }
        
        # Should be brighter (but clamped to 255)
        ready (adjusted_pixel[0] <= original_pixel[0]) {
            yikes "brightness adjustment failed"
        }
        
        damn based
    })
    
    run_test("Contrast Adjustment", slay() yikes<lit> {
        sus img Image = create_image(3, 3, 3)
        sus contrasted Image = adjust_contrast(img, 50.0) fam {
            when e -> yikes e
        }
        
        ready (contrasted.width != img.width || contrasted.height != img.height) {
            yikes "contrast adjustment changed dimensions"
        }
        
        damn based
    })
    
    damn based
}

# Format and metadata tests
slay test_format_detection() lit {
    run_test("Format Detection from Extension", slay() yikes<lit> {
        sus png_format tea = detect_format_from_extension("test.png") fam {
            when e -> yikes e
        }
        
        ready (png_format != "PNG") {
            yikes "PNG format detection failed"
        }
        
        sus jpg_format tea = detect_format_from_extension("test.jpg") fam {
            when e -> yikes e
        }
        
        ready (jpg_format != "JPEG") {
            yikes "JPEG format detection failed"
        }
        
        damn based
    })
    
    damn based
}

slay test_metadata_handling() lit {
    run_test("Image Metadata Structure", slay() yikes<lit> {
        sus metadata ImageMetadata = ImageMetadata{
            creation_time: 1234567890,
            modification_time: 1234567890,
            dpi_x: 72,
            dpi_y: 72,
            compression_quality: 90,
            author: "Test Author",
            description: "Test Image",
            copyright: "Test Copyright",
            software: "CURSED imagez"
        }
        
        ready (metadata.author != "Test Author") {
            yikes "metadata author field incorrect"
        }
        
        ready (metadata.dpi_x != 72) {
            yikes "metadata DPI field incorrect"
        }
        
        damn based
    })
    
    damn based
}

# Performance and edge case tests
slay test_large_image_handling() lit {
    run_test("Large Image Creation", slay() yikes<lit> {
        # Test reasonably large image (not too large for test environment)
        sus img Image = create_image(1000, 1000, 3)
        
        ready (len(img.data) != 1000 * 1000 * 3) {
            yikes "large image data size incorrect"
        }
        
        validate_image(img) fam {
            when e -> yikes "large image validation failed: " + e
        }
        
        damn based
    })
    
    damn based
}

slay test_error_handling() lit {
    run_test("Invalid Resize Parameters", slay() yikes<lit> {
        sus img Image = create_image(10, 10, 3)
        
        # Test invalid dimensions
        sus result Image = resize_image(img, 0, 0, "NEAREST") fam {
            when _ -> damn create_image(1, 1, 1)  # Expected to fail
        }
        
        ready (result.width != 1) {
            yikes "error handling failed - should have returned default"
        }
        
        damn based
    })
    
    run_test("Invalid Crop Parameters", slay() yikes<lit> {
        sus img Image = create_image(5, 5, 3)
        
        # Test out of bounds crop
        sus result Image = crop_image(img, 10, 10, 2, 2) fam {
            when _ -> damn create_image(1, 1, 1)  # Expected to fail
        }
        
        ready (result.width != 1) {
            yikes "crop error handling failed"
        }
        
        damn based
    })
    
    damn based
}

slay test_memory_safety() lit {
    run_test("Memory Safety - Clone Operations", slay() yikes<lit> {
        sus img Image = create_image(5, 5, 3)
        
        # Set original pixel
        sus original_pixel drip[value] = [100, 200, 150]
        set_pixel(&img, 2, 2, original_pixel) fam {
            when e -> yikes e
        }
        
        # Clone image
        sus cloned Image = clone_image(img)
        
        # Modify original
        set_pixel(&img, 2, 2, [255, 0, 0]) fam {
            when e -> yikes e
        }
        
        # Cloned should be unchanged
        sus cloned_pixel drip[value] = get_pixel(cloned, 2, 2) fam {
            when e -> yikes e
        }
        
        bestie (i drip = 0; i < 3; i = i + 1) {
            ready (cloned_pixel[i] != original_pixel[i]) {
                yikes "clone operation not memory safe"
            }
        }
        
        damn based
    })
    
    damn based
}

# Helper function for absolute value (until we can import mathz properly)
slay abs(x drip) drip {
    damn ready (x < 0) { damn -x } otherwise { damn x }
}

# Run all tests
run_imagez_tests()
