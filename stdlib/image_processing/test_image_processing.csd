yeet "testz"
yeet "image_processing"

fr fr Test image format detection
test_start("Image format detection - PNG")
sus png_format tea = img_detect_format("test.png")
assert_eq_string(png_format, "PNG")

test_start("Image format detection - JPEG")
sus jpeg_format tea = img_detect_format("photo.jpg")
assert_eq_string(jpeg_format, "JPEG")

test_start("Image format detection - JPEG with alternate extension")
sus jpeg_format2 tea = img_detect_format("image.jpeg")
assert_eq_string(jpeg_format2, "JPEG")

test_start("Image format detection - GIF")
sus gif_format tea = img_detect_format("animation.gif")
assert_eq_string(gif_format, "GIF")

test_start("Image format detection - BMP")
sus bmp_format tea = img_detect_format("bitmap.bmp")
assert_eq_string(bmp_format, "BMP")

test_start("Image format detection - WEBP")
sus webp_format tea = img_detect_format("modern.webp")
assert_eq_string(webp_format, "WEBP")

test_start("Image format detection - Unknown")
sus unknown_format tea = img_detect_format("file.txt")
assert_eq_string(unknown_format, "UNKNOWN")

fr fr Test image loading from bytes
test_start("Load image from bytes - PNG")
sus png_data tea = PNG_SIGNATURE + "test_data"
sus png_img ImageData = img_load_from_bytes(png_data, "PNG")
assert_eq_string(png_img.format, "PNG")
assert_eq_int(png_img.width, 100)
assert_eq_int(png_img.height, 100)
assert_eq_int(png_img.channels, 4)

test_start("Load image from bytes - JPEG")
sus jpeg_data tea = JPEG_SIGNATURE + "test_data"
sus jpeg_img ImageData = img_load_from_bytes(jpeg_data, "JPEG")
assert_eq_string(jpeg_img.format, "JPEG")
assert_eq_int(jpeg_img.width, 100)
assert_eq_int(jpeg_img.height, 100)
assert_eq_int(jpeg_img.channels, 3)

test_start("Load image from bytes - GIF")
sus gif_data tea = GIF_SIGNATURE + "test_data"
sus gif_img ImageData = img_load_from_bytes(gif_data, "GIF")
assert_eq_string(gif_img.format, "GIF")
assert_eq_int(gif_img.width, 100)
assert_eq_int(gif_img.height, 100)
assert_eq_int(gif_img.channels, 4)

test_start("Load image from bytes - BMP")
sus bmp_data tea = BMP_SIGNATURE + "test_data"
sus bmp_img ImageData = img_load_from_bytes(bmp_data, "BMP")
assert_eq_string(bmp_img.format, "BMP")
assert_eq_int(bmp_img.width, 100)
assert_eq_int(bmp_img.height, 100)
assert_eq_int(bmp_img.channels, 3)

fr fr Test image saving to bytes
test_start("Save image to bytes - PNG")
sus test_img ImageData
test_img.format = "PNG"
test_img.width = 50
test_img.height = 50
test_img.channels = 4
test_img.pixels = img_create_placeholder_pixels(50, 50, 4)
sus saved_png tea = img_save_to_bytes(test_img, "PNG")
assert_true(string_length(saved_png) > 0)

test_start("Save image to bytes - JPEG")
test_img.format = "JPEG"
test_img.channels = 3
test_img.pixels = img_create_placeholder_pixels(50, 50, 3)
sus saved_jpeg tea = img_save_to_bytes(test_img, "JPEG")
assert_true(string_length(saved_jpeg) > 0)

test_start("Save image to bytes - GIF")
test_img.format = "GIF"
test_img.channels = 4
test_img.pixels = img_create_placeholder_pixels(50, 50, 4)
sus saved_gif tea = img_save_to_bytes(test_img, "GIF")
assert_true(string_length(saved_gif) > 0)

test_start("Save image to bytes - BMP")
test_img.format = "BMP"
test_img.channels = 3
test_img.pixels = img_create_placeholder_pixels(50, 50, 3)
sus saved_bmp tea = img_save_to_bytes(test_img, "BMP")
assert_true(string_length(saved_bmp) > 0)

fr fr Test image resizing
test_start("Image resize operation")
sus original ImageData
original.format = "PNG"
original.width = 100
original.height = 100
original.channels = 4
original.pixels = img_create_placeholder_pixels(100, 100, 4)

sus resized ImageData = img_resize(original, 200, 150)
assert_eq_string(resized.format, "PNG")
assert_eq_int(resized.width, 200)
assert_eq_int(resized.height, 150)
assert_eq_int(resized.channels, 4)
assert_true(string_length(resized.pixels) > 0)

test_start("Image scale operation")
sus scaled ImageData = img_scale(original, 0.5)
assert_eq_string(scaled.format, "PNG")
assert_eq_int(scaled.width, 50)
assert_eq_int(scaled.height, 50)
assert_eq_int(scaled.channels, 4)

test_start("Image scale up operation")
sus scaled_up ImageData = img_scale(original, 2.0)
assert_eq_string(scaled_up.format, "PNG")
assert_eq_int(scaled_up.width, 200)
assert_eq_int(scaled_up.height, 200)
assert_eq_int(scaled_up.channels, 4)

fr fr Test image cropping
test_start("Image crop operation")
sus cropped ImageData = img_crop(original, 10, 10, 50, 50)
assert_eq_string(cropped.format, "PNG")
assert_eq_int(cropped.width, 50)
assert_eq_int(cropped.height, 50)
assert_eq_int(cropped.channels, 4)
assert_true(string_length(cropped.pixels) > 0)

test_start("Image crop edge case - full size")
sus full_crop ImageData = img_crop(original, 0, 0, 100, 100)
assert_eq_int(full_crop.width, 100)
assert_eq_int(full_crop.height, 100)

fr fr Test image rotation
test_start("Image rotate 90 degrees")
sus rotated ImageData = img_rotate(original, 1.5708) fr fr 90 degrees in radians
assert_eq_string(rotated.format, "PNG")
assert_eq_int(rotated.channels, 4)
assert_true(rotated.width > 0)
assert_true(rotated.height > 0)

test_start("Image rotate 45 degrees")
sus rotated_45 ImageData = img_rotate(original, 0.7854) fr fr 45 degrees in radians
assert_eq_string(rotated_45.format, "PNG")
assert_eq_int(rotated_45.channels, 4)
assert_true(rotated_45.width > original.width) fr fr Should be larger due to rotation
assert_true(rotated_45.height > original.height)

fr fr Test image flipping
test_start("Image flip horizontal")
sus flipped_h ImageData = img_flip_horizontal(original)
assert_eq_string(flipped_h.format, "PNG")
assert_eq_int(flipped_h.width, original.width)
assert_eq_int(flipped_h.height, original.height)
assert_eq_int(flipped_h.channels, original.channels)

test_start("Image flip vertical")
sus flipped_v ImageData = img_flip_vertical(original)
assert_eq_string(flipped_v.format, "PNG")
assert_eq_int(flipped_v.width, original.width)
assert_eq_int(flipped_v.height, original.height)
assert_eq_int(flipped_v.channels, original.channels)

fr fr Test image filters
test_start("Apply blur filter")
sus blurred ImageData = img_apply_filter(original, FILTER_BLUR)
assert_eq_string(blurred.format, "PNG")
assert_eq_int(blurred.width, original.width)
assert_eq_int(blurred.height, original.height)
assert_eq_int(blurred.channels, original.channels)

test_start("Apply sharpen filter")
sus sharpened ImageData = img_apply_filter(original, FILTER_SHARPEN)
assert_eq_string(sharpened.format, "PNG")
assert_eq_int(sharpened.width, original.width)
assert_eq_int(sharpened.height, original.height)

test_start("Apply edge detection filter")
sus edges ImageData = img_apply_filter(original, FILTER_EDGE_DETECT)
assert_eq_string(edges.format, "PNG")
assert_eq_int(edges.width, original.width)
assert_eq_int(edges.height, original.height)

test_start("Apply emboss filter")
sus embossed ImageData = img_apply_filter(original, FILTER_EMBOSS)
assert_eq_string(embossed.format, "PNG")
assert_eq_int(embossed.width, original.width)
assert_eq_int(embossed.height, original.height)

test_start("Apply grayscale filter")
sus gray ImageData = img_apply_filter(original, FILTER_GRAYSCALE)
assert_eq_string(gray.format, "PNG")
assert_eq_int(gray.width, original.width)
assert_eq_int(gray.height, original.height)

test_start("Apply sepia filter")
sus sepia ImageData = img_apply_filter(original, FILTER_SEPIA)
assert_eq_string(sepia.format, "PNG")
assert_eq_int(sepia.width, original.width)
assert_eq_int(sepia.height, original.height)

test_start("Apply invert filter")
sus inverted ImageData = img_apply_filter(original, FILTER_INVERT)
assert_eq_string(inverted.format, "PNG")
assert_eq_int(inverted.width, original.width)
assert_eq_int(inverted.height, original.height)

test_start("Apply unknown filter (no effect)")
sus unchanged ImageData = img_apply_filter(original, 999)
assert_eq_string(unchanged.format, "PNG")
assert_eq_int(unchanged.width, original.width)
assert_eq_int(unchanged.height, original.height)

fr fr Test brightness and contrast adjustments
test_start("Adjust brightness increase")
sus brighter ImageData = img_adjust_brightness(original, 1.2)
assert_eq_string(brighter.format, "PNG")
assert_eq_int(brighter.width, original.width)
assert_eq_int(brighter.height, original.height)

test_start("Adjust brightness decrease")
sus darker ImageData = img_adjust_brightness(original, 0.8)
assert_eq_string(darker.format, "PNG")
assert_eq_int(darker.width, original.width)
assert_eq_int(darker.height, original.height)

test_start("Adjust contrast increase")
sus higher_contrast ImageData = img_adjust_contrast(original, 1.5)
assert_eq_string(higher_contrast.format, "PNG")
assert_eq_int(higher_contrast.width, original.width)
assert_eq_int(higher_contrast.height, original.height)

test_start("Adjust contrast decrease")
sus lower_contrast ImageData = img_adjust_contrast(original, 0.5)
assert_eq_string(lower_contrast.format, "PNG")
assert_eq_int(lower_contrast.width, original.width)
assert_eq_int(lower_contrast.height, original.height)

fr fr Test custom filters
test_start("Apply custom filter")
sus custom_kernel tea = "0.1,0.1,0.1,0.1,0.2,0.1,0.1,0.1,0.1"
sus custom_filtered ImageData = img_custom_filter(original, custom_kernel, 3)
assert_eq_string(custom_filtered.format, "PNG")
assert_eq_int(custom_filtered.width, original.width)
assert_eq_int(custom_filtered.height, original.height)

fr fr Test pixel manipulation
test_start("Get pixel color")
sus pixel_color normie = img_get_pixel(original, 50, 50)
assert_true(pixel_color >= 0)

test_start("Set pixel color")
sus modified_pixel ImageData = img_set_pixel(original, 25, 25, COLOR_RED)
assert_eq_string(modified_pixel.format, "PNG")
assert_eq_int(modified_pixel.width, original.width)
assert_eq_int(modified_pixel.height, original.height)

test_start("Replace color")
sus color_replaced ImageData = img_replace_color(original, COLOR_BLACK, COLOR_WHITE, 0.1)
assert_eq_string(color_replaced.format, "PNG")
assert_eq_int(color_replaced.width, original.width)
assert_eq_int(color_replaced.height, original.height)

test_start("Color histogram calculation")
sus histogram tea = img_color_histogram(original)
assert_true(string_length(histogram) > 0)

fr fr Test metadata operations
test_start("Get image metadata")
sus metadata ImageMetadata = img_get_metadata(original)
assert_eq_string(metadata.format, "PNG")
assert_eq_int(metadata.width, 100)
assert_eq_int(metadata.height, 100)
assert_eq_int(metadata.color_depth, 32) fr fr 4 channels * 8 bits
assert_eq_string(metadata.compression, "DEFLATE")
assert_eq_string(metadata.author, "CURSED Image Processor")

test_start("Set image metadata")
sus new_metadata ImageMetadata
new_metadata.format = "JPEG"
new_metadata.width = 100
new_metadata.height = 100
new_metadata.color_depth = 24
new_metadata.compression = "DCT"
new_metadata.created_at = "2025-01-13T12:00:00Z"
new_metadata.author = "Test Author"

sus updated_img ImageData = img_set_metadata(original, new_metadata)
assert_eq_string(updated_img.format, "JPEG")

fr fr Test image composition
test_start("Image overlay operation")
sus overlay_img ImageData
overlay_img.format = "PNG"
overlay_img.width = 50
overlay_img.height = 50
overlay_img.channels = 4
overlay_img.pixels = img_create_placeholder_pixels(50, 50, 4)

sus overlaid ImageData = img_overlay(original, overlay_img, 25, 25, 0.5)
assert_eq_string(overlaid.format, "PNG")
assert_eq_int(overlaid.width, original.width)
assert_eq_int(overlaid.height, original.height)

fr fr Test image analysis
test_start("Calculate image similarity - identical")
sus similarity_identical drip = img_calculate_similarity(original, original)
assert_true(similarity_identical >= 0.0)

test_start("Calculate image similarity - different sizes")
sus different_size ImageData = img_resize(original, 50, 50)
sus similarity_different drip = img_calculate_similarity(original, different_size)
assert_eq_float(similarity_different, 0.0)

test_start("Edge detection")
sus detected_edges ImageData = img_detect_edges(original, 0.5)
assert_eq_string(detected_edges.format, "PNG")
assert_eq_int(detected_edges.width, original.width)
assert_eq_int(detected_edges.height, original.height)

test_start("Find contours")
sus contours tea = img_find_contours(original)
assert_true(string_length(contours) >= 0)

fr fr Test constants
test_start("PNG signature constant")
assert_true(string_length(PNG_SIGNATURE) > 0)

test_start("JPEG signature constant")
assert_true(string_length(JPEG_SIGNATURE) > 0)

test_start("GIF signature constant")
assert_true(string_length(GIF_SIGNATURE) > 0)

test_start("BMP signature constant")
assert_true(string_length(BMP_SIGNATURE) > 0)

test_start("WEBP signature constant")
assert_true(string_length(WEBP_SIGNATURE) > 0)

fr fr Test color constants
test_start("Color constants validation")
assert_eq_int(COLOR_RED, 0xFF0000)
assert_eq_int(COLOR_GREEN, 0x00FF00)
assert_eq_int(COLOR_BLUE, 0x0000FF)
assert_eq_int(COLOR_WHITE, 0xFFFFFF)
assert_eq_int(COLOR_BLACK, 0x000000)
assert_eq_int(COLOR_TRANSPARENT, 0x00000000)

fr fr Test filter constants
test_start("Filter constants validation")
assert_eq_int(FILTER_BLUR, 1)
assert_eq_int(FILTER_SHARPEN, 2)
assert_eq_int(FILTER_EDGE_DETECT, 3)
assert_eq_int(FILTER_EMBOSS, 4)
assert_eq_int(FILTER_GRAYSCALE, 5)
assert_eq_int(FILTER_SEPIA, 6)
assert_eq_int(FILTER_INVERT, 7)
assert_eq_int(FILTER_BRIGHTNESS, 8)
assert_eq_int(FILTER_CONTRAST, 9)

fr fr Test utility functions
test_start("Create placeholder pixels")
sus placeholder_pixels tea = img_create_placeholder_pixels(10, 10, 3)
assert_eq_int(string_length(placeholder_pixels), 300) fr fr 10 * 10 * 3

test_start("Bilinear resize utility")
sus original_pixels tea = img_create_placeholder_pixels(4, 4, 3)
sus resized_pixels tea = img_bilinear_resize(original_pixels, 4, 4, 8, 8, 3)
assert_eq_int(string_length(resized_pixels), 192) fr fr 8 * 8 * 3

test_start("Apply blur utility")
sus blur_pixels tea = img_apply_blur(original_pixels, 4, 4, 3)
assert_eq_int(string_length(blur_pixels), 48) fr fr 4 * 4 * 3

test_start("Apply grayscale utility")
sus gray_pixels tea = img_apply_grayscale(original_pixels, 4, 4, 3)
assert_eq_int(string_length(gray_pixels), 48) fr fr 4 * 4 * 3

fr fr Test error handling and edge cases
test_start("Empty image handling")
sus empty_img ImageData
empty_img.width = 0
empty_img.height = 0
empty_img.channels = 0
empty_img.pixels = ""
empty_img.format = "PNG"

sus empty_resized ImageData = img_resize(empty_img, 10, 10)
assert_eq_int(empty_resized.width, 10)
assert_eq_int(empty_resized.height, 10)

test_start("Single pixel image")
sus single_pixel ImageData
single_pixel.width = 1
single_pixel.height = 1
single_pixel.channels = 3
single_pixel.pixels = img_create_placeholder_pixels(1, 1, 3)
single_pixel.format = "PNG"

sus single_enlarged ImageData = img_resize(single_pixel, 10, 10)
assert_eq_int(single_enlarged.width, 10)
assert_eq_int(single_enlarged.height, 10)

test_start("Negative scale factor handling")
sus negative_scaled ImageData = img_scale(original, -1.0)
fr fr Should handle gracefully, possibly returning original or minimum size

test_start("Zero angle rotation")
sus zero_rotated ImageData = img_rotate(original, 0.0)
assert_eq_int(zero_rotated.width, original.width)
assert_eq_int(zero_rotated.height, original.height)

fr fr Test compression detection
test_start("PNG compression detection")
sus png_compression tea = img_detect_compression("PNG")
assert_eq_string(png_compression, "DEFLATE")

test_start("JPEG compression detection")
sus jpeg_compression tea = img_detect_compression("JPEG")
assert_eq_string(jpeg_compression, "DCT")

test_start("GIF compression detection")
sus gif_compression tea = img_detect_compression("GIF")
assert_eq_string(gif_compression, "LZW")

test_start("Unknown format compression")
sus unknown_compression tea = img_detect_compression("UNKNOWN")
assert_eq_string(unknown_compression, "NONE")

fr fr Test format-specific encoding
test_start("PNG encoding with signature")
sus png_encoded tea = img_encode_png(original)
assert_true(string_length(png_encoded) > string_length(PNG_SIGNATURE))

test_start("JPEG encoding with signature")
sus jpeg_encoded tea = img_encode_jpeg(original)
assert_true(string_length(jpeg_encoded) > string_length(JPEG_SIGNATURE))

test_start("GIF encoding with signature")
sus gif_encoded tea = img_encode_gif(original)
assert_true(string_length(gif_encoded) > string_length(GIF_SIGNATURE))

test_start("BMP encoding with signature")
sus bmp_encoded tea = img_encode_bmp(original)
assert_true(string_length(bmp_encoded) > string_length(BMP_SIGNATURE))

fr fr Test large image operations
test_start("Large image resize")
sus large_img ImageData
large_img.format = "PNG"
large_img.width = 1000
large_img.height = 1000
large_img.channels = 4
large_img.pixels = img_create_placeholder_pixels(1000, 1000, 4)

sus large_resized ImageData = img_resize(large_img, 100, 100)
assert_eq_int(large_resized.width, 100)
assert_eq_int(large_resized.height, 100)

test_start("Large image filter application")
sus large_blurred ImageData = img_apply_filter(large_img, FILTER_BLUR)
assert_eq_int(large_blurred.width, 1000)
assert_eq_int(large_blurred.height, 1000)

fr fr Test mathematical edge cases
test_start("Math utility functions")
sus cos_result drip = math_cos(0.0)
assert_true(cos_result >= 0.9) fr fr cos(0) should be close to 1

sus sin_result drip = math_sin(0.0)
assert_true(sin_result >= -0.1 && sin_result <= 0.1) fr fr sin(0) should be close to 0

sus abs_positive drip = math_abs(5.5)
assert_eq_float(abs_positive, 5.5)

sus abs_negative drip = math_abs(-3.7)
assert_eq_float(abs_negative, 3.7)

fr fr Test type conversion utilities
test_start("Float to int conversion")
sus converted_int normie = float_to_int(42.7)
assert_eq_int(converted_int, 42)

test_start("Int to float conversion")
sus converted_float drip = int_to_float(42)
assert_eq_float(converted_float, 42.0)

test_start("Byte to int conversion")
sus byte_val byte = 200
sus byte_int normie = byte_to_int(byte_val)
assert_eq_int(byte_int, 200)

test_start("Int to byte conversion")
sus int_val normie = 150
sus int_byte byte = int_to_byte(int_val)
assert_eq_int(byte_to_int(int_byte), 150)

print_test_summary()

fr fr Helper functions for testing (assume these exist in core stdlib)
slay string_length(s tea) normie { fr fr Implementation would be in core stdlib
    damn 10
}

slay assert_eq_float(actual drip, expected drip) { fr fr Implementation would compare floats with tolerance
    assert_true(math_abs(actual - expected) < 0.001)
}
