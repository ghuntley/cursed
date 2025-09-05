# image_processing Module

The `image_processing` module provides comprehensive image manipulation and processing capabilities for the CURSED programming language. This FFI-free module supports multiple image formats, filters, transformations, and analysis operations essential for graphics programming and visual applications.

## Features

### Image Format Support
- **PNG**: Portable Network Graphics with DEFLATE compression
- **JPEG**: Joint Photographic Experts Group with DCT compression
- **GIF**: Graphics Interchange Format with LZW compression
- **BMP**: Bitmap with no compression
- **WEBP**: Google's modern image format

### Image Loading & Saving
- Load images from files and byte arrays
- Save images to files and byte arrays
- Automatic format detection from file extensions
- Format conversion between different image types
- Binary data handling for raw pixel manipulation

### Image Transformations
- **Resize**: Scale images to specific dimensions with bilinear interpolation
- **Scale**: Proportionally resize images by scale factors
- **Crop**: Extract rectangular regions from images
- **Rotate**: Rotate images by arbitrary angles with automatic dimension calculation
- **Flip**: Horizontal and vertical image flipping

### Image Filters
- **Blur**: Gaussian-style blur for smoothing effects
- **Sharpen**: Edge enhancement and detail sharpening
- **Edge Detection**: Sobel edge detection algorithms
- **Emboss**: 3D embossed effect filter
- **Grayscale**: Luminance-based color to grayscale conversion
- **Sepia**: Vintage sepia tone effect
- **Invert**: Color inversion and negative effects
- **Custom Filters**: Apply custom convolution kernels

### Color Manipulation
- **Brightness Adjustment**: Increase or decrease image brightness
- **Contrast Adjustment**: Modify image contrast levels
- **Color Replacement**: Replace specific colors with tolerance
- **Pixel Access**: Get and set individual pixel colors
- **Color Histogram**: Calculate color distribution statistics

### Image Analysis
- **Similarity Calculation**: Compare images using mean squared error
- **Edge Detection**: Advanced edge detection with thresholds
- **Contour Tracing**: Find and trace image contours
- **Metadata Extraction**: Read and write image metadata

### Image Composition
- **Overlay**: Composite images with alpha blending
- **Multi-layer Composition**: Blend multiple images with different modes
- **Alpha Blending**: Transparent overlay operations

## Data Structures

### ImageData Structure
```cursed
be_like ImageData = struct {
    width normie,      # Image width in pixels
    height normie,     # Image height in pixels
    channels normie,   # Number of color channels (3=RGB, 4=RGBA)
    format tea,        # Image format (PNG, JPEG, GIF, BMP, WEBP)
    pixels tea         # Raw pixel data as string
}
```

### ImageMetadata Structure
```cursed
be_like ImageMetadata = struct {
    format tea,        # Image format
    width normie,      # Image dimensions
    height normie,
    color_depth normie,# Bits per pixel
    compression tea,   # Compression method
    created_at tea,    # Creation timestamp
    author tea         # Author information
}
```

## Functions

### Image Loading Functions
```cursed
slay img_load_from_file(filepath tea) ImageData
slay img_load_from_bytes(data tea, format tea) ImageData
slay img_detect_format(filepath tea) tea
slay img_decode_format(data tea, format tea) ImageData
```

### Format-Specific Decoders
```cursed
slay img_decode_png(data tea) ImageData
slay img_decode_jpeg(data tea) ImageData
slay img_decode_gif(data tea) ImageData
slay img_decode_bmp(data tea) ImageData
```

### Image Saving Functions
```cursed
slay img_save_to_file(img ImageData, filepath tea) lit
slay img_save_to_bytes(img ImageData, format tea) tea
slay img_encode_format(img ImageData, format tea) tea
```

### Format-Specific Encoders
```cursed
slay img_encode_png(img ImageData) tea
slay img_encode_jpeg(img ImageData) tea
slay img_encode_gif(img ImageData) tea
slay img_encode_bmp(img ImageData) tea
```

### Transformation Functions
```cursed
slay img_resize(img ImageData, new_width normie, new_height normie) ImageData
slay img_scale(img ImageData, scale_factor drip) ImageData
slay img_crop(img ImageData, x normie, y normie, width normie, height normie) ImageData
slay img_rotate(img ImageData, angle drip) ImageData
slay img_flip_horizontal(img ImageData) ImageData
slay img_flip_vertical(img ImageData) ImageData
```

### Filter Functions
```cursed
slay img_apply_filter(img ImageData, filter_type normie) ImageData
slay img_adjust_brightness(img ImageData, brightness drip) ImageData
slay img_adjust_contrast(img ImageData, contrast drip) ImageData
slay img_custom_filter(img ImageData, kernel tea, kernel_size normie) ImageData
```

### Color Manipulation Functions
```cursed
slay img_get_pixel(img ImageData, x normie, y normie) normie
slay img_set_pixel(img ImageData, x normie, y normie, color normie) ImageData
slay img_replace_color(img ImageData, old_color normie, new_color normie, tolerance drip) ImageData
slay img_color_histogram(img ImageData) tea
```

### Metadata Functions
```cursed
slay img_get_metadata(img ImageData) ImageMetadata
slay img_set_metadata(img ImageData, metadata ImageMetadata) ImageData
```

### Composition Functions
```cursed
slay img_overlay(base ImageData, overlay ImageData, x normie, y normie, alpha drip) ImageData
slay img_composite(images [ImageData], blend_modes [normie]) ImageData
```

### Analysis Functions
```cursed
slay img_calculate_similarity(img1 ImageData, img2 ImageData) drip
slay img_detect_edges(img ImageData, threshold drip) ImageData
slay img_find_contours(img ImageData) tea
```

## Constants

### Image Format Signatures
```cursed
facts PNG_SIGNATURE tea = "\x89PNG\r\n\x1a\n"
facts JPEG_SIGNATURE tea = "\xFF\xD8\xFF"
facts GIF_SIGNATURE tea = "GIF"
facts BMP_SIGNATURE tea = "BM"
facts WEBP_SIGNATURE tea = "RIFF"
```

### Color Constants
```cursed
facts COLOR_RED normie = 0xFF0000
facts COLOR_GREEN normie = 0x00FF00
facts COLOR_BLUE normie = 0x0000FF
facts COLOR_WHITE normie = 0xFFFFFF
facts COLOR_BLACK normie = 0x000000
facts COLOR_TRANSPARENT normie = 0x00000000
```

### Filter Constants
```cursed
facts FILTER_BLUR normie = 1
facts FILTER_SHARPEN normie = 2
facts FILTER_EDGE_DETECT normie = 3
facts FILTER_EMBOSS normie = 4
facts FILTER_GRAYSCALE normie = 5
facts FILTER_SEPIA normie = 6
facts FILTER_INVERT normie = 7
facts FILTER_BRIGHTNESS normie = 8
facts FILTER_CONTRAST normie = 9
```

## Usage Examples

### Basic Image Loading and Saving
```cursed
yeet "image_processing"

# Load an image from file
sus img ImageData = img_load_from_file("photo.jpg")
vibez.spill("Loaded image: " + img.width + "x" + img.height)

# Convert to PNG and save
sus png_saved lit = img_save_to_file(img, "converted.png")
vibez.spill("Saved as PNG: " + png_saved)
```

### Image Resizing and Scaling
```cursed
yeet "image_processing"

sus original ImageData = img_load_from_file("large_photo.jpg")

# Resize to specific dimensions
sus resized ImageData = img_resize(original, 800, 600)

# Scale proportionally
sus scaled ImageData = img_scale(original, 0.5)

# Save the results
img_save_to_file(resized, "resized_photo.jpg")
img_save_to_file(scaled, "scaled_photo.jpg")
```

### Applying Filters
```cursed
yeet "image_processing"

sus photo ImageData = img_load_from_file("portrait.jpg")

# Apply various filters
sus blurred ImageData = img_apply_filter(photo, FILTER_BLUR)
sus grayscale ImageData = img_apply_filter(photo, FILTER_GRAYSCALE)
sus sepia ImageData = img_apply_filter(photo, FILTER_SEPIA)

# Adjust brightness and contrast
sus brighter ImageData = img_adjust_brightness(photo, 1.2)
sus higher_contrast ImageData = img_adjust_contrast(photo, 1.5)

# Save filtered images
img_save_to_file(blurred, "blurred_portrait.jpg")
img_save_to_file(grayscale, "grayscale_portrait.jpg")
img_save_to_file(sepia, "sepia_portrait.jpg")
```

### Image Transformation
```cursed
yeet "image_processing"

sus image ImageData = img_load_from_file("landscape.png")

# Rotate 45 degrees
sus rotated ImageData = img_rotate(image, 0.7854)

# Flip horizontally
sus flipped ImageData = img_flip_horizontal(image)

# Crop a section
sus cropped ImageData = img_crop(image, 100, 100, 400, 300)

img_save_to_file(rotated, "rotated_landscape.png")
img_save_to_file(flipped, "flipped_landscape.png")
img_save_to_file(cropped, "cropped_landscape.png")
```

### Color Manipulation
```cursed
yeet "image_processing"

sus img ImageData = img_load_from_file("colorful.png")

# Get pixel color at specific position
sus pixel_color normie = img_get_pixel(img, 100, 50)
vibez.spill("Pixel color: " + pixel_color)

# Set a red pixel
sus modified ImageData = img_set_pixel(img, 100, 50, COLOR_RED)

# Replace all black pixels with white
sus color_replaced ImageData = img_replace_color(img, COLOR_BLACK, COLOR_WHITE, 0.1)

# Generate color histogram
sus histogram tea = img_color_histogram(img)
vibez.spill("Color histogram: " + histogram)
```

### Image Composition
```cursed
yeet "image_processing"

sus background ImageData = img_load_from_file("background.png")
sus overlay ImageData = img_load_from_file("overlay.png")

# Overlay with 50% transparency at position (100, 50)
sus composited ImageData = img_overlay(background, overlay, 100, 50, 0.5)

img_save_to_file(composited, "composited_image.png")
```

### Metadata Operations
```cursed
yeet "image_processing"

sus image ImageData = img_load_from_file("photo.jpg")

# Get metadata
sus metadata ImageMetadata = img_get_metadata(image)
vibez.spill("Format: " + metadata.format)
vibez.spill("Size: " + metadata.width + "x" + metadata.height)
vibez.spill("Color depth: " + metadata.color_depth + " bits")
vibez.spill("Compression: " + metadata.compression)

# Modify metadata
metadata.author = "CURSED Photographer"
metadata.created_at = "2025-01-13T12:00:00Z"

sus updated ImageData = img_set_metadata(image, metadata)
```

### Image Analysis
```cursed
yeet "image_processing"

sus img1 ImageData = img_load_from_file("image1.jpg")
sus img2 ImageData = img_load_from_file("image2.jpg")

# Compare similarity
sus similarity drip = img_calculate_similarity(img1, img2)
vibez.spill("Image similarity: " + similarity)

# Detect edges
sus edges ImageData = img_detect_edges(img1, 0.5)
img_save_to_file(edges, "edges.png")

# Find contours
sus contours tea = img_find_contours(img1)
vibez.spill("Contours found: " + contours)
```

### Custom Filter Application
```cursed
yeet "image_processing"

sus image ImageData = img_load_from_file("input.png")

# Define a custom 3x3 sharpening kernel
sus sharpen_kernel tea = "-1,-1,-1,-1,9,-1,-1,-1,-1"

# Apply custom filter
sus sharpened ImageData = img_custom_filter(image, sharpen_kernel, 3)

img_save_to_file(sharpened, "custom_sharpened.png")
```

## Performance Characteristics

- **Pure CURSED Implementation**: No FFI dependencies for maximum portability
- **Memory Efficient**: Optimized pixel data handling and processing
- **Streaming Processing**: Efficient handling of large images
- **Algorithm Optimization**: Bilinear interpolation, Sobel operators, and convolution kernels
- **Format Optimization**: Efficient encoding/decoding for each image format

## Self-Hosting Support

The `image_processing` module supports CURSED's self-hosting capabilities:

- **Compiler Assets**: Processing compiler icons, logos, and visual assets
- **Documentation Generation**: Creating diagrams and visual documentation
- **IDE Integration**: Image processing for syntax highlighting and UI elements
- **Build System**: Processing build artifacts and visual resources
- **Testing**: Generating visual test results and comparisons

## Testing

The module includes comprehensive tests covering:

- Image format detection and loading for all supported formats
- Image saving and encoding operations
- Transformation operations (resize, scale, crop, rotate, flip)
- Filter applications for all built-in filters
- Color manipulation and pixel access operations
- Metadata reading and writing
- Image composition and overlaying
- Analysis functions (similarity, edge detection, contours)
- Edge cases and error handling
- Mathematical utility functions
- Type conversion utilities

### Running Tests

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/image_processing/test_image_processing.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/image_processing/test_image_processing.💀
./test_image_processing

# Both-mode verification
test_both_modes() {
    cargo run --bin cursed stdlib/image_processing/test_image_processing.💀 > interp_output.txt
    cargo run --bin cursed -- compile stdlib/image_processing/test_image_processing.💀
    ./test_image_processing > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

## Error Handling

The module provides robust error handling:

- **Format Validation**: Automatic format detection and validation
- **Dimension Checking**: Safe handling of image dimensions and bounds
- **Memory Safety**: Bounds checking for pixel access and manipulation
- **Graceful Degradation**: Fallback to safe defaults for invalid operations
- **Input Validation**: Comprehensive validation of function parameters

## Algorithm Details

### Bilinear Interpolation
Used for image resizing to provide smooth scaling with anti-aliasing effects.

### Sobel Edge Detection
Implements Sobel operators for edge detection with configurable thresholds.

### Convolution Filters
Supports custom convolution kernels for advanced filtering effects.

### Color Space Conversion
Accurate RGB to grayscale conversion using luminance weighting (0.299R + 0.587G + 0.114B).

### Alpha Blending
Proper alpha compositing for transparent overlays and multi-layer composition.

## Standards Compliance

- **PNG**: Portable Network Graphics specification
- **JPEG**: JPEG File Interchange Format
- **GIF**: Graphics Interchange Format 89a
- **BMP**: Windows Bitmap format
- **Color Theory**: Standard RGB and RGBA color spaces

## Integration

The `image_processing` module integrates seamlessly with other CURSED stdlib modules:

- **fs/filesystem**: File I/O operations for image loading/saving
- **encode_mood**: Base64 encoding for web image embedding
- **math**: Mathematical operations for transformations and filters
- **time**: Timestamp generation for metadata
- **crypto**: Image watermarking and digital signatures

## Dependencies

**Pure CURSED Implementation** - No external dependencies

Core stdlib functions used:
- String manipulation and concatenation
- File I/O operations
- Mathematical functions (sin, cos, abs)
- Type conversion utilities
- Memory management functions

## Future Enhancements

Planned features for future versions:
- **Advanced Formats**: TIFF, RAW, HDR image support
- **Animation**: GIF animation frame processing
- **Color Profiles**: ICC color profile support
- **Compression**: Advanced compression algorithms
- **GPU Acceleration**: Hardware-accelerated operations
- **Machine Learning**: AI-powered image enhancement
- **Vector Graphics**: SVG support and vector operations
- **3D Graphics**: Depth maps and 3D image processing

## Performance Optimization

### Memory Management
- Efficient pixel buffer handling
- Minimal memory allocation for transformations
- Streaming processing for large images

### Algorithm Efficiency
- Optimized convolution kernels
- Fast bilinear interpolation
- Efficient color space conversions
- Parallel processing support

### Format Optimization
- Format-specific optimization strategies
- Compression-aware processing
- Progressive loading support

---

The `image_processing` module represents a comprehensive solution for image manipulation in CURSED, providing enterprise-grade functionality essential for graphics programming, visual applications, and modern software development workflows.
