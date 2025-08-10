# CURSED ImageZ Module - Professional Image Processing

## Overview

The ImageZ module provides comprehensive image processing capabilities for CURSED applications, enabling professional-grade image manipulation, format conversion, and analysis. This module supports multiple image formats and offers both CPU and GPU-accelerated processing.

## Features

### Supported Image Formats
- **PNG** - Portable Network Graphics with transparency support
- **JPEG** - Joint Photographic Experts Group format with quality control
- **GIF** - Graphics Interchange Format with animation support
- **BMP** - Bitmap format for uncompressed images
- **WebP** - Google's WebP format with modern compression
- **TIFF** - Tagged Image File Format for high-quality images
- **ICO** - Icon format for applications

### Core Functionality
- **Loading and Saving** - Read/write images from files or memory
- **Format Conversion** - Convert between different image formats
- **Resizing and Scaling** - Multiple interpolation algorithms
- **Transformations** - Rotate, flip, crop, and geometric operations
- **Filters and Effects** - Blur, sharpen, edge detection, vintage effects
- **Color Manipulation** - Brightness, contrast, levels, curves adjustment
- **Image Composition** - Blending, masking, and layering operations
- **Analysis Tools** - Histograms, feature detection, similarity comparison

### Hardware Acceleration
- **GPU Support** - OpenGL and Vulkan compute shaders
- **Performance Optimization** - Multi-threaded processing
- **Memory Efficiency** - Zero-copy operations where possible

## Quick Start

```cursed
yeet "imagez"

# Load an image
sus img imagez.ImageData = imagez_load_from_file("photo.jpg")

# Apply filters
img = imagez_resize(img, 1024, 768, imagez.INTERPOLATION_BILINEAR)
img = imagez_apply_filter(img, imagez.FILTER_SHARPEN, 1.5)

# Save the result
imagez_save_to_file(img, "processed.png", 100)
```

## API Reference

### Core Functions

#### Loading and Saving
```cursed
slay imagez_load_from_file(filepath tea) ImageData
slay imagez_load_from_memory(data tea, format tea) ImageData
slay imagez_save_to_file(img ImageData, filepath tea, quality normie) lit
slay imagez_save_to_memory(img ImageData, format tea, quality normie) tea
```

#### Format Detection
```cursed
slay imagez_detect_format_from_file(filepath tea) tea
slay imagez_detect_format_from_signature(data tea) tea
```

#### Image Transformations
```cursed
slay imagez_resize(img ImageData, new_width normie, new_height normie, interpolation normie) ImageData
slay imagez_scale(img ImageData, scale_x drip, scale_y drip, interpolation normie) ImageData
slay imagez_crop(img ImageData, x normie, y normie, width normie, height normie) ImageData
slay imagez_rotate(img ImageData, angle drip, background_color normie) ImageData
slay imagez_flip_horizontal(img ImageData) ImageData
slay imagez_flip_vertical(img ImageData) ImageData
```

#### Filters and Effects
```cursed
slay imagez_apply_filter(img ImageData, filter_type normie, intensity drip) ImageData
slay imagez_apply_color_matrix(img ImageData, matrix ColorMatrix) ImageData
slay imagez_adjust_levels(img ImageData, input_min normie, input_max normie, gamma drip, output_min normie, output_max normie) ImageData
slay imagez_adjust_curves(img ImageData, curve_points [256]normie) ImageData
```

#### Image Composition
```cursed
slay imagez_blend(base ImageData, overlay ImageData, x normie, y normie, blend_mode normie, opacity drip) ImageData
slay imagez_alpha_composite(base ImageData, overlay ImageData, x normie, y normie) ImageData
slay imagez_create_mask(img ImageData, color normie, tolerance drip) ImageData
slay imagez_apply_mask(img ImageData, mask ImageData) ImageData
```

#### Analysis Functions
```cursed
slay imagez_calculate_histogram(img ImageData) ImageHistogram
slay imagez_calculate_similarity(img1 ImageData, img2 ImageData) drip
slay imagez_detect_features(img ImageData, threshold drip) tea
slay imagez_find_contours(img ImageData, threshold drip) tea
```

#### Hardware Acceleration
```cursed
slay imagez_enable_gpu_acceleration() lit
slay imagez_disable_gpu_acceleration() lit
slay imagez_is_gpu_available() lit
```

### Data Structures

#### ImageData
```cursed
be_like ImageData = struct {
    width normie,
    height normie,
    channels normie,
    format tea,
    pixels tea,
    color_space tea,
    dpi normie,
    has_alpha lit,
    compression_level normie
}
```

#### ImageMetadata
```cursed
be_like ImageMetadata = struct {
    format tea,
    width normie,
    height normie,
    color_depth normie,
    compression tea,
    created_at tea,
    author tea,
    description tea,
    camera_make tea,
    camera_model tea,
    exposure_time tea,
    iso_speed normie,
    gps_latitude drip,
    gps_longitude drip
}
```

#### ImageHistogram
```cursed
be_like ImageHistogram = struct {
    red [256]normie,
    green [256]normie,
    blue [256]normie,
    alpha [256]normie,
    luminance [256]normie
}
```

### Constants

#### Interpolation Methods
```cursed
facts INTERPOLATION_NEAREST normie = 0
facts INTERPOLATION_BILINEAR normie = 1
facts INTERPOLATION_BICUBIC normie = 2
facts INTERPOLATION_LANCZOS normie = 3
```

#### Filter Types
```cursed
facts FILTER_BLUR normie = 1
facts FILTER_SHARPEN normie = 2
facts FILTER_EDGE_DETECT normie = 3
facts FILTER_EMBOSS normie = 4
facts FILTER_GRAYSCALE normie = 5
facts FILTER_SEPIA normie = 6
facts FILTER_INVERT normie = 7
facts FILTER_GAUSSIAN_BLUR normie = 12
facts FILTER_MOTION_BLUR normie = 13
facts FILTER_VINTAGE normie = 15
```

#### Blend Modes
```cursed
facts BLEND_NORMAL normie = 0
facts BLEND_MULTIPLY normie = 1
facts BLEND_SCREEN normie = 2
facts BLEND_OVERLAY normie = 3
facts BLEND_SOFT_LIGHT normie = 4
facts BLEND_HARD_LIGHT normie = 5
```

## Usage Examples

### Basic Image Processing
```cursed
yeet "imagez"

# Load and process an image
sus img imagez.ImageData = imagez_load_from_file("input.jpg")

# Resize with high-quality interpolation
img = imagez_resize(img, 1920, 1080, imagez.INTERPOLATION_LANCZOS)

# Apply sharpening filter
img = imagez_apply_filter(img, imagez.FILTER_SHARPEN, 2.0)

# Adjust brightness and contrast
img = imagez_adjust_levels(img, 10, 245, 1.2, 0, 255)

# Save as high-quality PNG
imagez_save_to_file(img, "output.png", 100)
```

### Image Composition
```cursed
# Load base and overlay images
sus background imagez.ImageData = imagez_load_from_file("background.jpg")
sus logo imagez.ImageData = imagez_load_from_file("logo.png")

# Resize logo to fit
logo = imagez_resize(logo, 200, 100, imagez.INTERPOLATION_BILINEAR)

# Blend with transparency
sus result imagez.ImageData = imagez_blend(background, logo, 50, 50, imagez.BLEND_NORMAL, 0.8)

# Save composite image
imagez_save_to_file(result, "composite.jpg", 95)
```

### Batch Processing
```cursed
# Process multiple images
sus files [10]tea = ["img1.jpg", "img2.jpg", ...]

sus i normie = 0
bestie (i < 10) {
    sus img imagez.ImageData = imagez_load_from_file(files[i])
    
    # Standard processing pipeline
    img = imagez_resize(img, 1024, 768, imagez.INTERPOLATION_BILINEAR)
    img = imagez_apply_filter(img, imagez.FILTER_SHARPEN, 1.2)
    img = imagez_adjust_levels(img, 5, 250, 1.1, 0, 255)
    
    # Save processed image
    sus output tea = stringz_concat("processed_", files[i])
    imagez_save_to_file(img, output, 90)
    
    i = i + 1
}
```

### Advanced Color Manipulation
```cursed
sus img imagez.ImageData = imagez_load_from_file("colorful.jpg")

# Create color matrix for sepia effect
sus sepia_matrix imagez.ColorMatrix
sepia_matrix.matrix[0] = 0.393  # Red component
sepia_matrix.matrix[1] = 0.769
sepia_matrix.matrix[2] = 0.189
sepia_matrix.matrix[3] = 0.0
sepia_matrix.matrix[4] = 0.349  # Green component
sepia_matrix.matrix[5] = 0.686
sepia_matrix.matrix[6] = 0.168
sepia_matrix.matrix[7] = 0.0
sepia_matrix.matrix[8] = 0.272  # Blue component
sepia_matrix.matrix[9] = 0.534
sepia_matrix.matrix[10] = 0.131
sepia_matrix.matrix[11] = 0.0

sus sepia_img imagez.ImageData = imagez_apply_color_matrix(img, sepia_matrix)
imagez_save_to_file(sepia_img, "sepia.jpg", 95)
```

### GPU Acceleration
```cursed
# Check GPU availability and enable acceleration
ready (imagez_is_gpu_available()) {
    imagez_enable_gpu_acceleration()
    vibez.spill("GPU acceleration enabled")
    
    # GPU-accelerated operations
    sus img imagez.ImageData = imagez_load_from_file("large_image.tiff")
    img = imagez_resize(img, 4096, 3072, imagez.INTERPOLATION_BICUBIC)
    img = imagez_apply_filter(img, imagez.FILTER_GAUSSIAN_BLUR, 5.0)
    
    imagez_disable_gpu_acceleration()
} otherwise {
    vibez.spill("Using CPU processing")
}
```

## Performance Considerations

### Memory Usage
- Large images consume significant memory (width × height × channels × bytes_per_channel)
- Use streaming processing for very large images
- Consider downsampling for preview operations

### Processing Speed
- GPU acceleration provides 5-10x speedup for large images
- Bilinear interpolation is faster than bicubic or Lanczos
- Avoid unnecessary format conversions

### Quality vs Speed
- Use appropriate interpolation methods for the use case
- Higher quality settings increase processing time
- Consider progressive processing for real-time applications

## Dependencies

The ImageZ module depends on:
- `vibez` - For output and logging
- `mathz` - For mathematical operations
- `stringz` - For string manipulation
- `memoryz` - For memory management
- `filez` - For file I/O operations

## Error Handling

The module follows CURSED error handling conventions:
- Functions return empty/invalid data structures on error
- Use validation functions to check results
- Error messages are logged through the vibez module

## Platform Support

ImageZ is designed to work across all CURSED-supported platforms:
- **Linux** - Full feature support including GPU acceleration
- **macOS** - Full feature support with Metal backend
- **Windows** - Full feature support with DirectX backend
- **WebAssembly** - Core features with software rendering

## License

This module is part of the CURSED standard library and follows the same licensing terms as the core language.
