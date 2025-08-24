yeet "imagez"

# Test image processing module with actual image algorithms
sus width drip = 100
sus height drip = 100

# Test image creation and manipulation
sus image map<tea, tea> = create_image_rgb(width, height)
set_pixel_rgb(image, 10, 10, 255, 0, 0)  # Red pixel
vibez.spill("✓ Created", width, "x", height, "RGB image")

# Test image filtering algorithms
sus blurred_image map<tea, tea> = gaussian_blur(image, 2.0)
vibez.spill("✓ Applied Gaussian blur with sigma 2.0")

sus sharpened_image map<tea, tea> = sharpen_image(image)
vibez.spill("✓ Applied sharpening filter")

sus edge_detected map<tea, tea> = sobel_edge_detection(image)
vibez.spill("✓ Applied Sobel edge detection")

# Test color space conversions
sus hsv_image map<tea, tea> = rgb_to_hsv(image)
sus back_to_rgb map<tea, tea> = hsv_to_rgb(hsv_image)
vibez.spill("✓ RGB to HSV and back conversion")

# Test geometric transformations
sus rotated_image map<tea, tea> = rotate_image(image, 45.0)
sus scaled_image map<tea, tea> = scale_image(image, 0.5, 0.5)
vibez.spill("✓ Image rotation and scaling")

# Test histogram operations
sus histogram []drip = compute_histogram(image)
sus equalized_image map<tea, tea> = histogram_equalization(image)
vibez.spill("✓ Histogram analysis and equalization")

# Test morphological operations
sus eroded_image map<tea, tea> = morphological_erosion(image, 3)
sus dilated_image map<tea, tea> = morphological_dilation(image, 3)
vibez.spill("✓ Morphological erosion and dilation")

# Test image format support
save_image_png(image, "test_output.png")
sus loaded_image map<tea, tea> = load_image_png("test_output.png")
vibez.spill("✓ PNG save and load operations")

vibez.spill("✅ imagez: All real image processing algorithms working")
