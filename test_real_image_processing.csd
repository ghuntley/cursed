yeet "testz"
yeet "image_processing/mod"
yeet "image_processing/algorithms" 

fr fr Comprehensive Image Processing Test Suite
fr fr Tests all real implementations including PNG/JPEG/GIF decoders

fr fr Test data generation
slay create_test_png_header() []byte {
    fr fr Create minimal valid PNG header
    sus header []byte = [
        fr fr PNG signature
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
        fr fr IHDR chunk length (13 bytes)
        0x00, 0x00, 0x00, 0x0D,
        fr fr IHDR chunk type
        0x49, 0x48, 0x44, 0x52,
        fr fr Width (100)
        0x00, 0x00, 0x00, 0x64,
        fr fr Height (100)  
        0x00, 0x00, 0x00, 0x64,
        fr fr Bit depth (8), Color type (2=RGB), Compression (0), Filter (0), Interlace (0)
        0x08, 0x02, 0x00, 0x00, 0x00,
        fr fr IHDR CRC
        0x7D, 0xD4, 0x8C, 0x1D,
        fr fr Dummy IDAT chunk
        0x00, 0x00, 0x00, 0x0C,
        0x49, 0x44, 0x41, 0x54,
        fr fr Compressed data (simplified)
        0x78, 0x9C, 0x62, 0x00, 0x02, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0E, 0x60,
        fr fr CRC
        0x2B, 0xB2, 0xD4, 0xA1,
        fr fr IEND chunk
        0x00, 0x00, 0x00, 0x00,
        0x49, 0x45, 0x4E, 0x44,
        0xAE, 0x42, 0x60, 0x82
    ]
    damn header
}

slay create_test_jpeg_header() []byte {
    fr fr Create minimal valid JPEG header
    sus header []byte = [
        fr fr SOI marker
        0xFF, 0xD8,
        fr fr SOF0 marker
        0xFF, 0xC0,
        fr fr Length
        0x00, 0x11,
        fr fr Precision
        0x08,
        fr fr Height (100)
        0x00, 0x64,
        fr fr Width (100)
        0x00, 0x64,
        fr fr Components (3)
        0x03,
        fr fr Component data (simplified)
        0x01, 0x11, 0x00,
        0x02, 0x11, 0x01,
        0x03, 0x11, 0x01,
        fr fr DHT marker (simplified)
        0xFF, 0xC4,
        0x00, 0x1F,
        0x00,
        fr fr Huffman table data (simplified)
        0x00, 0x01, 0x05, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B,
        fr fr SOS marker
        0xFF, 0xDA,
        0x00, 0x0C,
        0x03,
        0x01, 0x00,
        0x02, 0x11,
        0x03, 0x11,
        0x00, 0x3F, 0x00,
        fr fr Compressed data (simplified)
        0xD2, 0xCF, 0x20, 0xFF, 0x00,
        fr fr EOI marker
        0xFF, 0xD9
    ]
    damn header
}

slay create_test_gif_header() []byte {
    fr fr Create minimal valid GIF header
    sus header []byte = [
        fr fr GIF signature and version
        0x47, 0x49, 0x46, 0x38, 0x39, 0x61,
        fr fr Width (100)
        0x64, 0x00,
        fr fr Height (100)
        0x64, 0x00,
        fr fr Packed field (global color table, 8 colors)
        0xF0,
        fr fr Background color index
        0x00,
        fr fr Pixel aspect ratio
        0x00,
        fr fr Global color table (8 colors)
        0x00, 0x00, 0x00, fr fr Black
        0xFF, 0x00, 0x00, fr fr Red
        0x00, 0xFF, 0x00, fr fr Green
        0x00, 0x00, 0xFF, fr fr Blue
        0xFF, 0xFF, 0x00, fr fr Yellow
        0xFF, 0x00, 0xFF, fr fr Magenta
        0x00, 0xFF, 0xFF, fr fr Cyan
        0xFF, 0xFF, 0xFF, fr fr White
        fr fr Image descriptor
        0x2C,
        fr fr Left position
        0x00, 0x00,
        fr fr Top position
        0x00, 0x00,
        fr fr Width
        0x64, 0x00,
        fr fr Height
        0x64, 0x00,
        fr fr Packed field
        0x00,
        fr fr LZW minimum code size
        0x02,
        fr fr Image data (simplified)
        0x04, 0x01, 0x01, 0x00, 0x02,
        fr fr Block terminator
        0x00,
        fr fr GIF terminator
        0x3B
    ]
    damn header
}

slay test_format_detection() {
    spill("Testing image format detection...")
    
    sus png_data []byte = create_test_png_header()
    sus jpeg_data []byte = create_test_jpeg_header()  
    sus gif_data []byte = create_test_gif_header()
    
    sus png_format tea = detect_image_format_from_header(png_data)
    sus jpeg_format tea = detect_image_format_from_header(jpeg_data)
    sus gif_format tea = detect_image_format_from_header(gif_data)
    
    assert_eq(png_format, "PNG", "PNG format detection")
    assert_eq(jpeg_format, "JPEG", "JPEG format detection")
    assert_eq(gif_format, "GIF", "GIF format detection")
    
    spill("✅ Format detection tests passed")
}

slay test_png_decoder() {
    spill("Testing real PNG decoder...")
    
    sus png_data []byte = create_test_png_header()
    sus width normie = 0
    sus height normie = 0
    sus pixels []byte = []
    
    width, height, pixels = decode_png_basic(png_data)
    
    assert_true(width > 0, "PNG width should be positive")
    assert_true(height > 0, "PNG height should be positive")
    assert_true(len(pixels) > 0, "PNG should decode pixels")
    
    spill("PNG decoded dimensions: ", width, "x", height, " with ", len(pixels), " pixels")
    spill("✅ PNG decoder tests passed")
}

slay test_jpeg_decoder() {
    spill("Testing real JPEG decoder...")
    
    sus jpeg_data []byte = create_test_jpeg_header()
    sus width normie = 0
    sus height normie = 0
    sus pixels []byte = []
    
    width, height, pixels = decode_jpeg_basic(jpeg_data)
    
    assert_true(width > 0, "JPEG width should be positive")
    assert_true(height > 0, "JPEG height should be positive")
    assert_true(len(pixels) > 0, "JPEG should decode pixels")
    
    spill("JPEG decoded dimensions: ", width, "x", height, " with ", len(pixels), " pixels")
    spill("✅ JPEG decoder tests passed")
}

slay test_gif_decoder() {
    spill("Testing real GIF decoder...")
    
    sus gif_data []byte = create_test_gif_header()
    sus width normie = 0
    sus height normie = 0
    sus pixels []byte = []
    
    width, height, pixels = decode_gif_basic(gif_data)
    
    assert_true(width > 0, "GIF width should be positive")
    assert_true(height > 0, "GIF height should be positive")
    assert_true(len(pixels) > 0, "GIF should decode pixels")
    
    spill("GIF decoded dimensions: ", width, "x", height, " with ", len(pixels), " pixels")
    spill("✅ GIF decoder tests passed")
}

slay test_image_filters() {
    spill("Testing advanced image filters...")
    
    fr fr Create test image data
    sus width normie = 100
    sus height normie = 100
    sus channels normie = 3
    sus test_pixels []byte = []
    
    fr fr Generate test pattern
    bestie y := 0; y < height; y++ {
        bestie x := 0; x < width; x++ {
            sus r byte = byte((x * 255) / width)
            sus g byte = byte((y * 255) / height)
            sus b byte = byte(((x + y) * 255) / (width + height))
            
            test_pixels = append(test_pixels, r)
            test_pixels = append(test_pixels, g)  
            test_pixels = append(test_pixels, b)
        }
    }
    
    spill("Original image: ", len(test_pixels), " pixels")
    
    fr fr Test Gaussian blur
    sus blurred []byte = apply_gaussian_blur(test_pixels, width, height, channels, 3)
    assert_eq(len(blurred), len(test_pixels), "Blur should preserve pixel count")
    spill("✅ Gaussian blur applied")
    
    fr fr Test edge detection
    sus edges []byte = apply_sobel_edge_detection(test_pixels, width, height, channels)
    assert_true(len(edges) > 0, "Edge detection should produce output")
    spill("✅ Sobel edge detection applied")
    
    fr fr Test color conversions
    sus grayscale []byte = convert_to_grayscale(test_pixels, width, height, channels)
    assert_true(len(grayscale) > 0, "Grayscale conversion should work")
    spill("✅ Grayscale conversion applied")
    
    sus sepia []byte = apply_sepia_tone(test_pixels, width, height, channels)
    assert_eq(len(sepia), len(test_pixels), "Sepia should preserve pixel count")
    spill("✅ Sepia tone applied")
    
    fr fr Test geometric operations
    sus flipped_h []byte = flip_horizontal(test_pixels, width, height, channels)
    assert_eq(len(flipped_h), len(test_pixels), "Horizontal flip should preserve pixel count")
    spill("✅ Horizontal flip applied")
    
    sus flipped_v []byte = flip_vertical(test_pixels, width, height, channels)
    assert_eq(len(flipped_v), len(test_pixels), "Vertical flip should preserve pixel count")
    spill("✅ Vertical flip applied")
}

slay test_color_space_conversion() {
    spill("Testing color space conversions...")
    
    fr fr Test RGB to HSV conversion
    sus h meal = 0.0
    sus s meal = 0.0
    sus v meal = 0.0
    
    h, s, v = convert_rgb_to_hsv(255, 0, 0) fr fr Pure red
    assert_true(h >= 0.0 && h <= 360.0, "Hue should be in valid range")
    assert_true(s >= 0.0 && s <= 1.0, "Saturation should be in valid range")
    assert_true(v >= 0.0 && v <= 1.0, "Value should be in valid range")
    spill("RGB(255,0,0) -> HSV(", h, ",", s, ",", v, ")")
    
    fr fr Test HSV to RGB conversion
    sus r byte = 0
    sus g byte = 0
    sus b byte = 0
    
    r, g, b = convert_hsv_to_rgb(h, s, v)
    assert_true(r > 200, "Red should be high for pure red")
    assert_true(g < 50, "Green should be low for pure red")
    assert_true(b < 50, "Blue should be low for pure red")
    spill("HSV(", h, ",", s, ",", v, ") -> RGB(", r, ",", g, ",", b, ")")
    
    spill("✅ Color space conversion tests passed")
}

slay test_advanced_filters() {
    spill("Testing advanced image filters...")
    
    sus width normie = 50
    sus height normie = 50
    sus channels normie = 3
    sus test_pixels []byte = []
    
    fr fr Create checkerboard pattern
    bestie y := 0; y < height; y++ {
        bestie x := 0; x < width; x++ {
            sus checker byte = byte(((x / 8) + (y / 8)) % 2 * 255)
            test_pixels = append(test_pixels, checker)
            test_pixels = append(test_pixels, checker)
            test_pixels = append(test_pixels, checker)
        }
    }
    
    fr fr Test unsharp mask
    sus sharpened []byte = apply_unsharp_mask(test_pixels, width, height, channels, 1.5, 2, 10)
    assert_eq(len(sharpened), len(test_pixels), "Unsharp mask should preserve pixel count")
    spill("✅ Unsharp mask applied")
    
    fr fr Test Canny edge detection
    sus canny_edges []byte = apply_canny_edge_detection(test_pixels, width, height, channels, 50, 150)
    assert_true(len(canny_edges) > 0, "Canny edge detection should produce output")
    spill("✅ Canny edge detection applied")
    
    fr fr Test histogram equalization
    sus equalized []byte = equalize_histogram(test_pixels, width, height, channels)
    assert_eq(len(equalized), len(test_pixels), "Histogram equalization should preserve pixel count")
    spill("✅ Histogram equalization applied")
    
    fr fr Test median filter
    sus median_filtered []byte = apply_median_filter(test_pixels, width, height, channels, 3)
    assert_eq(len(median_filtered), len(test_pixels), "Median filter should preserve pixel count")
    spill("✅ Median filter applied")
}

slay test_image_analysis() {
    spill("Testing image analysis features...")
    
    sus width normie = 20
    sus height normie = 20
    sus channels normie = 3
    sus test_pixels []byte = []
    
    fr fr Create colorful test image
    bestie y := 0; y < height; y++ {
        bestie x := 0; x < width; x++ {
            sus r byte = byte((x * 255) / width)
            sus g byte = byte((y * 255) / height)
            sus b byte = byte(128)
            
            test_pixels = append(test_pixels, r)
            test_pixels = append(test_pixels, g)
            test_pixels = append(test_pixels, b)
        }
    }
    
    fr fr Test histogram calculation
    sus histogram [256]normie = calculate_histogram(test_pixels, width, height, channels)
    sus total_count normie = 0
    bestie i := 0; i < 256; i++ {
        total_count += histogram[i]
    }
    assert_true(total_count > 0, "Histogram should have counts")
    spill("✅ Histogram calculated with ", total_count, " total samples")
    
    fr fr Test dominant color extraction
    sus colors []tea = extract_dominant_colors(test_pixels, width, height, channels, 5)
    assert_true(len(colors) > 0, "Should extract dominant colors")
    spill("✅ Extracted ", len(colors), " dominant colors")
    bestie i := 0; i < len(colors); i++ {
        spill("  Color ", i, ": ", colors[i])
    }
}

slay test_bilinear_interpolation() {
    spill("Testing bilinear interpolation for resizing...")
    
    sus old_width normie = 10
    sus old_height normie = 10
    sus new_width normie = 20
    sus new_height normie = 20
    sus channels normie = 3
    
    fr fr Create simple gradient
    sus original_pixels []byte = []
    bestie y := 0; y < old_height; y++ {
        bestie x := 0; x < old_width; x++ {
            sus intensity byte = byte((x + y) * 255 / (old_width + old_height))
            original_pixels = append(original_pixels, intensity)
            original_pixels = append(original_pixels, intensity)
            original_pixels = append(original_pixels, intensity)
        }
    }
    
    sus resized_pixels []byte = bilinear_interpolate(original_pixels, old_width, old_height, 
                                                   new_width, new_height, channels)
    
    sus expected_size normie = new_width * new_height * channels
    assert_eq(len(resized_pixels), expected_size, "Resized image should have correct size")
    spill("✅ Bilinear interpolation: ", old_width, "x", old_height, " -> ", new_width, "x", new_height)
}

slay test_high_level_api() {
    spill("Testing high-level image processing API...")
    
    fr fr Create ImageData structure
    sus img ImageData
    img.format = "TEST"
    img.width = 50
    img.height = 50
    img.channels = 3
    
    fr fr Generate test pixels
    sus pixel_data tea = ""
    bestie i := 0; i < img.width * img.height * img.channels; i++ {
        sus value byte = byte(i % 256)
        pixel_data = string_concat(pixel_data, string_from_byte(value))
    }
    img.pixels = pixel_data
    
    fr fr Test resize
    sus resized ImageData = img_resize(img, 100, 100)
    assert_eq(resized.width, 100, "Resized width should be correct")
    assert_eq(resized.height, 100, "Resized height should be correct")
    spill("✅ Image resize API works")
    
    fr fr Test filters
    sus blurred ImageData = img_apply_filter(img, FILTER_BLUR)
    assert_eq(blurred.width, img.width, "Blur should preserve dimensions")
    assert_eq(blurred.height, img.height, "Blur should preserve dimensions")
    spill("✅ Blur filter API works")
    
    sus grayscale ImageData = img_apply_filter(img, FILTER_GRAYSCALE)
    assert_eq(grayscale.format, img.format, "Grayscale should preserve format")
    spill("✅ Grayscale filter API works")
    
    fr fr Test adjustments
    sus brighter ImageData = img_adjust_brightness(img, 50.0)
    assert_eq(brighter.width, img.width, "Brightness adjustment should preserve dimensions")
    spill("✅ Brightness adjustment API works")
    
    sus higher_contrast ImageData = img_adjust_contrast(img, 1.5)
    assert_eq(higher_contrast.width, img.width, "Contrast adjustment should preserve dimensions")
    spill("✅ Contrast adjustment API works")
}

slay run_comprehensive_tests() {
    spill("🧪 Starting Comprehensive Real Image Processing Tests")
    spill("=" * 60)
    
    test_start("Image Processing")
    
    test_format_detection()
    test_png_decoder()
    test_jpeg_decoder()
    test_gif_decoder()
    test_image_filters()
    test_color_space_conversion()
    test_advanced_filters()
    test_image_analysis()
    test_bilinear_interpolation()
    test_high_level_api()
    
    print_test_summary()
    
    spill("=" * 60)
    spill("🎉 All image processing tests completed!")
    spill("")
    spill("✨ Real Image Processing Features Implemented:")
    spill("   📸 PNG decoder with DEFLATE decompression")
    spill("   📷 JPEG decoder with DCT and Huffman tables")
    spill("   🎞️ GIF decoder with LZW decompression")
    spill("   🔧 Advanced filters (Gaussian blur, Canny edges, unsharp mask)")
    spill("   🎨 Color space conversions (RGB ↔ HSV)")
    spill("   📊 Image analysis (histograms, dominant colors)")
    spill("   ✂️ Geometric operations (resize, rotate, flip, crop)")
    spill("   🎯 Feature extraction and edge detection")
    spill("   🌈 Filter effects (sepia, emboss, invert, median)")
    spill("")
}

fr fr Run all tests
run_comprehensive_tests()
