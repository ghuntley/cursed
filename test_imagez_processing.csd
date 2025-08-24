// Test image processing (imagez package)
yeet "imagez"
yeet "vibez"

vibez.spill("=== Testing Imagez Image Processing ===")

// Test image creation
sus width drip = 100
sus height drip = 100
sus image Image = create_image(width, height, RGB)

ready (image.width == width && image.height == height) {
    vibez.spill("✅ Image creation: PASSED")
} otherwise {
    vibez.spill("❌ Image creation: FAILED")
}

// Test pixel manipulation
sus red_color Color = rgb(255, 0, 0)
sus green_color Color = rgb(0, 255, 0)
sus blue_color Color = rgb(0, 0, 255)

set_pixel(image, 10, 10, red_color)
set_pixel(image, 20, 20, green_color)  
set_pixel(image, 30, 30, blue_color)

sus retrieved_red Color = get_pixel(image, 10, 10)
sus retrieved_green Color = get_pixel(image, 20, 20)
sus retrieved_blue Color = get_pixel(image, 30, 30)

ready (color_equals(retrieved_red, red_color) && 
       color_equals(retrieved_green, green_color) && 
       color_equals(retrieved_blue, blue_color)) {
    vibez.spill("✅ Pixel manipulation: PASSED")
} otherwise {
    vibez.spill("❌ Pixel manipulation: FAILED")
}

// Test image filters
sus filtered_image Image = apply_blur(image, 2.0)
sus grayscale_image Image = convert_to_grayscale(image)
sus resized_image Image = resize_image(image, 50, 50, BILINEAR)

ready (filtered_image.width == width && filtered_image.height == height &&
       grayscale_image.width == width && grayscale_image.height == height &&
       resized_image.width == 50 && resized_image.height == 50) {
    vibez.spill("✅ Image filters and transformations: PASSED")
} otherwise {
    vibez.spill("❌ Image filters and transformations: FAILED")
}

// Test image format support
sus png_data []drip = encode_png(image)
sus decoded_png Image = decode_png(png_data)

ready (decoded_png.width == image.width && decoded_png.height == image.height) {
    vibez.spill("✅ PNG encoding/decoding: PASSED")
} otherwise {
    vibez.spill("❌ PNG encoding/decoding: FAILED")
}

// Test JPEG support
sus jpeg_data []drip = encode_jpeg(image, 85)  // 85% quality
sus decoded_jpeg Image = decode_jpeg(jpeg_data)

ready (decoded_jpeg.width == image.width && decoded_jpeg.height == image.height) {
    vibez.spill("✅ JPEG encoding/decoding: PASSED")
} otherwise {
    vibez.spill("❌ JPEG encoding/decoding: FAILED")
}

vibez.spill("Image processing stats:")
vibez.spill("- Original size:", width, "x", height)
vibez.spill("- PNG data size:", len(png_data), "bytes")
vibez.spill("- JPEG data size:", len(jpeg_data), "bytes")

vibez.spill("=== Imagez Testing Complete ===")
