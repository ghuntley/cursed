yeet "testz"
yeet "image_processing/mod"

fr fr Test BMP decoding functionality
slay test_bmp_decoding() {
    test_start("BMP Decoding Tests")
    
    fr fr Create minimal 2x2 24-bit BMP test data
    fr fr BMP file header (14 bytes)
    sus bmp_header byte[value] = [
        0x42, 0x4D,             fr fr "BM" signature
        0x36, 0x00, 0x00, 0x00, fr fr File size (54 bytes total)
        0x00, 0x00,             fr fr Reserved 1
        0x00, 0x00,             fr fr Reserved 2
        0x36, 0x00, 0x00, 0x00  fr fr Pixel data offset (54 bytes)
    ]
    
    fr fr BMP info header (40 bytes)
    sus bmp_info byte[value] = [
        0x28, 0x00, 0x00, 0x00, fr fr Header size (40)
        0x02, 0x00, 0x00, 0x00, fr fr Width (2 pixels)
        0x02, 0x00, 0x00, 0x00, fr fr Height (2 pixels)
        0x01, 0x00,             fr fr Planes (1)
        0x18, 0x00,             fr fr Bits per pixel (24)
        0x00, 0x00, 0x00, 0x00, fr fr Compression (0 = none)
        0x00, 0x00, 0x00, 0x00, fr fr Image size (0 = uncompressed)
        0x00, 0x00, 0x00, 0x00, fr fr X pixels per meter (0)
        0x00, 0x00, 0x00, 0x00, fr fr Y pixels per meter (0)
        0x00, 0x00, 0x00, 0x00, fr fr Colors used (0)
        0x00, 0x00, 0x00, 0x00  fr fr Important colors (0)
    ]
    
    fr fr Pixel data: 2x2 image with padding
    fr fr Row 1 (bottom): Red (BGR: 00,00,FF), Green (BGR: 00,FF,00) + 2 padding bytes
    fr fr Row 2 (top): Blue (BGR: FF,00,00), White (BGR: FF,FF,FF) + 2 padding bytes
    sus pixel_data byte[value] = [
        0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, fr fr Red, Green + padding
        0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00  fr fr Blue, White + padding
    ]
    
    fr fr Combine all data
    sus bmp_data byte[value] = []
    sus i normie = 0
    bestie (i < len(bmp_header)) {
        bmp_data = append(bmp_data, bmp_header[i])
        i = i + 1
    }
    i = 0
    bestie (i < len(bmp_info)) {
        bmp_data = append(bmp_data, bmp_info[i])
        i = i + 1
    }
    i = 0
    bestie (i < len(pixel_data)) {
        bmp_data = append(bmp_data, pixel_data[i])
        i = i + 1
    }
    
    fr fr Test BMP decoding
    sus width, height, pixels normie, normie, byte[value] = decode_bmp_basic(bmp_data)
    
    assert_eq_int(width, 2)
    assert_eq_int(height, 2)
    assert_eq_int(len(pixels), 12) fr fr 2x2 pixels * 3 channels = 12 bytes
    
    fr fr Test pixel values (remember BMP is stored bottom-to-top)
    fr fr First pixel should be Blue (from top row)
    assert_eq_int(pixels[0], 0x00) fr fr R
    assert_eq_int(pixels[1], 0x00) fr fr G
    assert_eq_int(pixels[2], 0xFF) fr fr B
    
    fr fr Second pixel should be White
    assert_eq_int(pixels[3], 0xFF) fr fr R
    assert_eq_int(pixels[4], 0xFF) fr fr G
    assert_eq_int(pixels[5], 0xFF) fr fr B
    
    print_test_summary()
}

fr fr Test invalid BMP data handling
slay test_bmp_error_handling() {
    test_start("BMP Error Handling Tests")
    
    fr fr Test empty data
    sus empty_data byte[value] = []
    sus w1, h1, p1 normie, normie, byte[value] = decode_bmp_basic(empty_data)
    assert_eq_int(w1, 0)
    assert_eq_int(h1, 0)
    assert_eq_int(len(p1), 0)
    
    fr fr Test invalid signature
    sus invalid_sig byte[value] = [0x41, 0x41, 0x00, 0x00, 0x00, 0x00] fr fr "AA" instead of "BM"
    sus w2, h2, p2 normie, normie, byte[value] = decode_bmp_basic(invalid_sig)
    assert_eq_int(w2, 0)
    assert_eq_int(h2, 0)
    assert_eq_int(len(p2), 0)
    
    fr fr Test truncated header
    sus truncated byte[value] = [0x42, 0x4D, 0x00, 0x00] fr fr Only 4 bytes
    sus w3, h3, p3 normie, normie, byte[value] = decode_bmp_basic(truncated)
    assert_eq_int(w3, 0)
    assert_eq_int(h3, 0)
    assert_eq_int(len(p3), 0)
    
    print_test_summary()
}

fr fr Test utility functions
slay test_utility_functions() {
    test_start("Utility Functions Tests")
    
    fr fr Test read_uint16_le
    sus test_data16 byte[value] = [0x34, 0x12] fr fr 0x1234 in little-endian
    sus result16 normie = read_uint16_le(test_data16, 0)
    assert_eq_int(result16, 0x1234)
    
    fr fr Test read_uint32_le
    sus test_data32 byte[value] = [0x78, 0x56, 0x34, 0x12] fr fr 0x12345678 in little-endian
    sus result32 normie = read_uint32_le(test_data32, 0)
    assert_eq_int(result32, 0x12345678)
    
    fr fr Test make_byte_array
    sus byte_array byte[value] = make_byte_array(5)
    assert_eq_int(len(byte_array), 5)
    assert_eq_int(byte_array[0], 0)
    assert_eq_int(byte_array[4], 0)
    
    print_test_summary()
}

fr fr Test image manipulation functions
slay test_image_manipulation() {
    test_start("Image Manipulation Tests")
    
    fr fr Create a 2x2 RGB test image (6 bytes total)
    sus test_pixels byte[value] = [
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, fr fr Red, Green
        0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF  fr fr Blue, White
    ]
    
    fr fr Test cropping
    sus cropped byte[value] = crop_image(test_pixels, 2, 2, 3, 0, 0, 1, 1)
    assert_eq_int(len(cropped), 3) fr fr 1x1 * 3 channels
    assert_eq_int(cropped[0], 0xFF) fr fr Red pixel
    assert_eq_int(cropped[1], 0x00)
    assert_eq_int(cropped[2], 0x00)
    
    fr fr Test horizontal flip
    sus flipped byte[value] = flip_horizontal(test_pixels, 2, 2, 3)
    assert_eq_int(len(flipped), 12) fr fr Same size as original
    fr fr First pixel should now be Green (was Red)
    assert_eq_int(flipped[0], 0x00) fr fr R
    assert_eq_int(flipped[1], 0xFF) fr fr G
    assert_eq_int(flipped[2], 0x00) fr fr B
    
    print_test_summary()
}

fr fr Main test runner
slay main() {
    vibez.spill("Running Image Processing Tests...")
    
    test_bmp_decoding()
    test_bmp_error_handling()
    test_utility_functions()
    test_image_manipulation()
    
    vibez.spill("All image processing tests completed!")
}
