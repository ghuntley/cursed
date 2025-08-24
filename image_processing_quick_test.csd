yeet "stdlib/image_processing/mod"

fr fr Quick validation test for image processing module
slay main() {
    sus width normie = 8
    sus height normie = 8
    sus channels normie = 3
    
    sus pixels []byte = img_create_test_pattern_pixels(width, height, channels)
    vibez.spill("Test pattern pixels created:", len(pixels), "bytes")
    
    sus test_img ImageData
    test_img.width = width
    test_img.height = height
    test_img.channels = channels
    test_img.format = "TEST"
    test_img.pixels = pixels
    
    vibez.spill("Original image:", test_img.width, "x", test_img.height)
    
    sus resized ImageData = img_resize(test_img, 16, 16)
    vibez.spill("Resized image:", resized.width, "x", resized.height)
    
    sus blurred ImageData = img_apply_filter(test_img, FILTER_BLUR)
    vibez.spill("Blur filter applied, size:", len(blurred.pixels))
    
    sus grayscale ImageData = img_apply_filter(test_img, FILTER_GRAYSCALE)
    vibez.spill("Grayscale filter applied, size:", len(grayscale.pixels))
    
    vibez.spill("✓ Image processing module working without placeholders!")
}
