# imagez - Main Module Export
# Comprehensive image processing module for CURSED

# Export all core functionality
yeet "./core" as core
yeet "./formats" as formats  
yeet "./manipulation" as manipulation
yeet "./filters" as filters
yeet "./colorspace" as colorspace
yeet "./test" as test

# Re-export commonly used structures and functions for convenience
yeet core.{
    Image,
    RGB, RGBA, HSV, LAB,
    ImageFormat, ColorSpace, FilterType,
    ImageMetadata,
    create_image,
    clone_image,
    get_pixel,
    set_pixel,
    validate_image,
    get_image_info
}

yeet formats.{
    load_image,
    save_image,
    detect_format_from_extension
}

yeet manipulation.{
    resize_image,
    crop_image,
    rotate_image,
    flip_horizontal,
    flip_vertical,
    scale_image,
    extract_channel
}

yeet filters.{
    apply_filter,
    gaussian_blur,
    box_blur,
    motion_blur,
    sobel_edge_detection,
    unsharp_mask,
    adjust_brightness,
    adjust_contrast,
    adjust_gamma,
    convert_to_grayscale,
    median_filter,
    apply_preset_filter,
    # Filter kernels
    BLUR_3X3,
    SHARPEN_3X3,
    EDGE_DETECT_3X3,
    EMBOSS_3X3,
    GAUSSIAN_3X3,
    GAUSSIAN_5X5
}

yeet colorspace.{
    rgb_to_hsv_precise,
    hsv_to_rgb_precise,
    rgb_to_lab,
    lab_to_rgb,
    rgb_to_cmyk,
    cmyk_to_rgb,
    convert_image_colorspace,
    convert_to_rgb,
    convert_to_rgba,
    adjust_color_temperature,
    white_balance
}

# Module information
sus IMAGEZ_VERSION tea = "1.0.0"
sus IMAGEZ_DESCRIPTION tea = "Professional image processing library for CURSED"

# Quick module test function
slay test_imagez() lit {
    test.run_imagez_tests()
}

# Module initialization message
vibez.spill("📸 imagez v" + IMAGEZ_VERSION + " - Professional image processing loaded")
