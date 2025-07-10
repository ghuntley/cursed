yeet "testz"
yeet "image_processing"

// Test Image Loading
test_start("image_load_jpeg")
sus result lit = image_load("test.jpg")
assert_true(result)
assert_true(image_is_loaded())
assert_eq_string(image_get_format(), "jpeg")
assert_eq_int(image_get_width(), 1920)
assert_eq_int(image_get_height(), 1080)

test_start("image_load_png")
image_clear()
sus result2 lit = image_load("test.png")
assert_true(result2)
assert_eq_string(image_get_format(), "png")

test_start("image_load_gif")
image_clear()
sus result3 lit = image_load("test.gif")
assert_true(result3)
assert_eq_string(image_get_format(), "gif")

test_start("image_load_unsupported")
image_clear()
sus result4 lit = image_load("test.bmp")
assert_false(result4)

// Test Image Manipulation
test_start("image_resize")
image_load("test.jpg")
sus resize_result lit = image_resize(800, 600)
assert_true(resize_result)
assert_eq_int(image_get_width(), 800)
assert_eq_int(image_get_height(), 600)

test_start("image_crop")
image_load("test.jpg")
sus crop_result lit = image_crop(100, 100, 400, 300)
assert_true(crop_result)
assert_eq_int(image_get_width(), 400)
assert_eq_int(image_get_height(), 300)

test_start("image_rotate_90")
image_load("test.jpg")
sus original_width normie = image_get_width()
sus original_height normie = image_get_height()
sus rotate_result lit = image_rotate(90)
assert_true(rotate_result)
assert_eq_int(image_get_width(), original_height)
assert_eq_int(image_get_height(), original_width)

test_start("image_flip_horizontal")
image_load("test.jpg")
sus flip_h_result lit = image_flip_horizontal()
assert_true(flip_h_result)

test_start("image_flip_vertical")
image_load("test.jpg")
sus flip_v_result lit = image_flip_vertical()
assert_true(flip_v_result)

// Test Color Manipulation
test_start("image_adjust_brightness")
image_load("test.jpg")
sus brightness_result lit = image_adjust_brightness(1.2)
assert_true(brightness_result)

test_start("image_adjust_contrast")
image_load("test.jpg")
sus contrast_result lit = image_adjust_contrast(1.5)
assert_true(contrast_result)

test_start("image_adjust_saturation")
image_load("test.jpg")
sus saturation_result lit = image_adjust_saturation(0.8)
assert_true(saturation_result)

test_start("image_convert_to_grayscale")
image_load("test.jpg")
sus grayscale_result lit = image_convert_to_grayscale()
assert_true(grayscale_result)

test_start("image_convert_to_sepia")
image_load("test.jpg")
sus sepia_result lit = image_convert_to_sepia()
assert_true(sepia_result)

// Test Filter Functions
test_start("image_apply_blur")
image_load("test.jpg")
sus blur_result lit = image_apply_blur(5)
assert_true(blur_result)

test_start("image_apply_sharpen")
image_load("test.jpg")
sus sharpen_result lit = image_apply_sharpen(3)
assert_true(sharpen_result)

test_start("image_apply_edge_detection")
image_load("test.jpg")
sus edge_result lit = image_apply_edge_detection()
assert_true(edge_result)

test_start("image_apply_emboss")
image_load("test.jpg")
sus emboss_result lit = image_apply_emboss()
assert_true(emboss_result)

// Test Format Conversion
test_start("image_convert_format_jpeg_to_png")
image_load("test.jpg")
sus convert_result lit = image_convert_format("png")
assert_true(convert_result)
assert_eq_string(image_get_format(), "png")

test_start("image_convert_format_invalid")
image_load("test.jpg")
sus convert_invalid lit = image_convert_format("invalid")
assert_false(convert_invalid)

// Test Metadata
test_start("image_get_metadata")
image_load("test.jpg")
sus metadata tea = image_get_metadata()
assert_true(metadata.contains("width:"))
assert_true(metadata.contains("height:"))
assert_true(metadata.contains("format:"))

test_start("image_set_metadata")
image_load("test.jpg")
sus metadata_result lit = image_set_metadata("author", "test_user")
assert_true(metadata_result)

// Test Batch Processing
test_start("image_batch_resize")
sus batch_resize_count normie = image_batch_resize("file1.jpg,file2.jpg,file3.jpg", 800, 600)
assert_eq_int(batch_resize_count, 3)

test_start("image_batch_convert")
sus batch_convert_count normie = image_batch_convert("file1.jpg,file2.jpg", "png")
assert_eq_int(batch_convert_count, 2)

// Test Histogram
test_start("image_calculate_histogram")
image_load("test.jpg")
sus histogram tea = image_calculate_histogram()
assert_true(histogram.contains("red:"))
assert_true(histogram.contains("green:"))
assert_true(histogram.contains("blue:"))

test_start("image_equalize_histogram")
image_load("test.jpg")
sus equalize_result lit = image_equalize_histogram()
assert_true(equalize_result)

// Test Compression
test_start("image_compress")
image_load("test.jpg")
sus compress_result lit = image_compress(80)
assert_true(compress_result)

test_start("image_get_file_size")
image_load("test.jpg")
sus file_size normie = image_get_file_size()
assert_true(file_size > 0)

// Test Utility Functions
test_start("image_create_thumbnail")
image_load("test.jpg")
sus thumbnail_result lit = image_create_thumbnail(200)
assert_true(thumbnail_result)
assert_true(image_get_width() <= 200)
assert_true(image_get_height() <= 200)

test_start("image_validate_format_valid")
assert_true(image_validate_format("test.jpg"))
assert_true(image_validate_format("test.jpeg"))
assert_true(image_validate_format("test.png"))
assert_true(image_validate_format("test.gif"))

test_start("image_validate_format_invalid")
assert_false(image_validate_format("test.bmp"))
assert_false(image_validate_format("test.txt"))

test_start("image_clear")
image_load("test.jpg")
assert_true(image_is_loaded())
image_clear()
assert_false(image_is_loaded())
assert_eq_int(image_get_width(), 0)
assert_eq_int(image_get_height(), 0)

// Test Error Handling
test_start("operations_without_loaded_image")
image_clear()
assert_false(image_resize(100, 100))
assert_false(image_crop(0, 0, 100, 100))
assert_false(image_rotate(90))
assert_false(image_flip_horizontal())
assert_false(image_adjust_brightness(1.0))
assert_false(image_convert_to_grayscale())
assert_false(image_apply_blur(5))
assert_false(image_convert_format("png"))
assert_false(image_compress(80))
assert_false(image_equalize_histogram())
assert_false(image_create_thumbnail(200))

test_start("image_save")
image_load("test.jpg")
sus save_result lit = image_save("output.jpg", 90)
assert_true(save_result)

test_start("image_save_without_loaded")
image_clear()
sus save_no_image lit = image_save("output.jpg", 90)
assert_false(save_no_image)

print_test_summary()
