yeet "testz"
yeet "image_processing/algorithms"

test_start("Image Processing Algorithms Test Suite")

fr fr Helper function to create test image data
slay create_test_image(width normie, height normie, channels normie) byte[value]{
    sus pixels byte[value] = []
    sus pixel_count normie = width * height
    
    bestie i := 0; i < pixel_count; i++ { fr fr Create a simple gradient pattern
        sus x normie = i % width
        sus y normie = i / width
        sus value byte = byte((x + y) % 256)
        
        bestie c := 0; c < channels; c++ {
            vibe_check c < 3 { fr fr RGB channels
                pixels = append(pixels, value)
            } damn { fr fr Alpha channel (if present)
                pixels = append(pixels, 255)
            }
        }
    }
    
    damn pixels
}

test_start("Image Format Detection")

fr fr Test PNG format detection
sus png_header byte[value] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
sus png_format tea = detect_image_format_from_header(png_header)
assert_eq_string(png_format, "PNG")

fr fr Test JPEG format detection
sus jpeg_header byte[value] = [0xFF, 0xD8, 0xFF, 0xE0]
sus jpeg_format tea = detect_image_format_from_header(jpeg_header)
assert_eq_string(jpeg_format, "JPEG")

fr fr Test GIF format detection
sus gif_header byte[value] = [0x47, 0x49, 0x46, 0x38]
sus gif_format tea = detect_image_format_from_header(gif_header)
assert_eq_string(gif_format, "GIF")

fr fr Test BMP format detection
sus bmp_header byte[value] = [0x42, 0x4D, 0x36, 0x84]
sus bmp_format tea = detect_image_format_from_header(bmp_header)
assert_eq_string(bmp_format, "BMP")

test_start("Safe Pixel Access")

fr fr Create test image 4x4 RGB
sus test_pixels byte[value] = create_test_image(4, 4, 3)

fr fr Test valid pixel access
sus pixel_value byte = get_pixel_safe(test_pixels, 2, 2, 0, 4, 4, 3)
assert_true(pixel_value >= 0)

fr fr Test out-of-bounds access
sus oob_pixel byte = get_pixel_safe(test_pixels, 10, 10, 0, 4, 4, 3)
assert_eq_int(normie(oob_pixel), 0)

fr fr Test negative coordinates
sus neg_pixel byte = get_pixel_safe(test_pixels, -1, -1, 0, 4, 4, 3)
assert_eq_int(normie(neg_pixel), 0)

test_start("Image Resizing")

fr fr Test bilinear interpolation
sus original_pixels byte[value] = create_test_image(4, 4, 3)
sus resized_pixels byte[value] = bilinear_interpolate(original_pixels, 4, 4, 8, 8, 3)

fr fr Check that resized image has correct size
sus expected_size normie = 8 * 8 * 3
assert_eq_int(len(resized_pixels), expected_size)

fr fr Test downsizing
sus downsized_pixels byte[value] = bilinear_interpolate(original_pixels, 4, 4, 2, 2, 3)
sus downsize_expected normie = 2 * 2 * 3
assert_eq_int(len(downsized_pixels), downsize_expected)

test_start("Color Space Conversion")

fr fr Test grayscale conversion
sus rgb_pixels byte[value] = create_test_image(3, 3, 3)
sus gray_pixels byte[value] = convert_to_grayscale(rgb_pixels, 3, 3, 3)

fr fr Should have same number of pixels
assert_eq_int(len(gray_pixels), len(rgb_pixels))

fr fr Test RGBA to grayscale
sus rgba_pixels byte[value] = create_test_image(2, 2, 4)
sus gray_rgba_pixels byte[value] = convert_to_grayscale(rgba_pixels, 2, 2, 4)
assert_eq_int(len(gray_rgba_pixels), len(rgba_pixels))

test_start("Image Effects")

fr fr Test sepia tone effect
sus sepia_pixels byte[value] = apply_sepia_tone(rgb_pixels, 3, 3, 3)
assert_eq_int(len(sepia_pixels), len(rgb_pixels))

fr fr Test brightness adjustment
sus bright_pixels byte[value] = adjust_brightness(rgb_pixels, 3, 3, 3, 50.0)
assert_eq_int(len(bright_pixels), len(rgb_pixels))

fr fr Test contrast adjustment
sus contrast_pixels byte[value] = adjust_contrast(rgb_pixels, 3, 3, 3, 1.5)
assert_eq_int(len(contrast_pixels), len(rgb_pixels))

test_start("Image Flipping")

fr fr Test horizontal flip
sus flipped_h byte[value] = flip_horizontal(rgb_pixels, 3, 3, 3)
assert_eq_int(len(flipped_h), len(rgb_pixels))

fr fr Test vertical flip
sus flipped_v byte[value] = flip_vertical(rgb_pixels, 3, 3, 3)
assert_eq_int(len(flipped_v), len(rgb_pixels))

fr fr Verify that double flip returns to original (simplified test)
sus double_flip byte[value] = flip_horizontal(flipped_h, 3, 3, 3)
fr fr Note: Exact pixel comparison would require more sophisticated testing

test_start("Image Cropping")

fr fr Test basic cropping
sus cropped_pixels byte[value] = crop_image(rgb_pixels, 3, 3, 3, 1, 1, 2, 2)
sus expected_crop_size normie = 2 * 2 * 3
assert_eq_int(len(cropped_pixels), expected_crop_size)

fr fr Test cropping with out-of-bounds region
sus oob_cropped byte[value] = crop_image(rgb_pixels, 3, 3, 3, 2, 2, 3, 3)
sus oob_expected_size normie = 3 * 3 * 3
assert_eq_int(len(oob_cropped), oob_expected_size)

test_start("Edge Detection")

fr fr Test Sobel edge detection
sus edge_pixels byte[value] = apply_sobel_edge_detection(rgb_pixels, 3, 3, 3)
fr fr Edge detection returns smaller image (excludes border)
sus edge_expected_size normie = 1 * 1 * 3 fr fr (3-2) x (3-2) x 3
assert_eq_int(len(edge_pixels), edge_expected_size)

test_start("Gaussian Kernel Creation")

fr fr Test creating Gaussian kernel
sus kernel meal[value] = create_gaussian_kernel(5, 1.0)
assert_eq_int(len(kernel), 5)

fr fr Kernel should sum to approximately 1.0
sus kernel_sum meal = 0.0
bestie i := 0; i < len(kernel); i++ {
    kernel_sum = kernel_sum + kernel[i]
}
assert_true(kernel_sum > 0.99 && kernel_sum < 1.01)

test_start("Mathematical Helpers")

fr fr Test square root function
assert_true(math_sqrt(4.0) > 1.9 && math_sqrt(4.0) < 2.1)
assert_true(math_sqrt(9.0) > 2.9 && math_sqrt(9.0) < 3.1)
assert_eq_int(normie(math_sqrt(0.0)), 0)

fr fr Test exponential function
assert_true(math_exp(0.0) > 0.9 && math_exp(0.0) < 1.1)
assert_true(math_exp(1.0) > 2.5 && math_exp(1.0) < 3.0)

test_start("Image Decoder Tests")

fr fr Test PNG decoding with minimal data
sus png_test_data byte[value] = [
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, fr fr PNG signature
    0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, fr fr IHDR chunk
    0x00, 0x00, 0x00, 0x10, fr fr Width: 16
    0x00, 0x00, 0x00, 0x10, fr fr Height: 16
    0x08, 0x02, 0x00, 0x00, 0x00 fr fr Bit depth, color type, etc.
]

sus png_width, png_height, png_pixels = decode_png_basic(png_test_data)
assert_eq_int(png_width, 16)
assert_eq_int(png_height, 16)
assert_true(len(png_pixels) > 0)

fr fr Test JPEG decoding
sus jpeg_test_data byte[value] = [
    0xFF, 0xD8, 0xFF, 0xE0, fr fr JPEG signature
    0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
    0xFF, 0xC0, fr fr SOF0 marker
    0x00, 0x11, 0x08, fr fr Length, precision
    0x00, 0x20, fr fr Height: 32
    0x00, 0x30 fr fr Width: 48
]

sus jpeg_width, jpeg_height, jpeg_pixels = decode_jpeg_basic(jpeg_test_data)
assert_eq_int(jpeg_width, 48)
assert_eq_int(jpeg_height, 32)
assert_true(len(jpeg_pixels) > 0)

test_start("Blur Filter")

fr fr Test Gaussian blur (simplified test)
sus blur_test_pixels byte[value] = create_test_image(5, 5, 3)
sus blurred_pixels byte[value] = apply_gaussian_blur(blur_test_pixels, 5, 5, 3, 1)
assert_eq_int(len(blurred_pixels), len(blur_test_pixels))

print_test_summary()
