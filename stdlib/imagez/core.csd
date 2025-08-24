# imagez - Core Image Processing Types and Structures
# Pure CURSED implementation with optimized format support

# Core image data structure
squad Image {
    sus width drip
    sus height drip
    sus channels drip
    sus data []drip
    sus format tea
    sus color_space tea
}

# Color structures
squad RGB {
    sus r drip
    sus g drip
    sus b drip
}

squad RGBA {
    sus r drip
    sus g drip
    sus b drip  
    sus a drip
}

squad HSV {
    sus h tea  # Hue in degrees
    sus s tea  # Saturation 0-1
    sus v tea  # Value 0-1
}

squad LAB {
    sus l tea  # Lightness 0-100
    sus a tea  # A component -128 to 127
    sus b tea  # B component -128 to 127
}

# Image format enumeration
squad ImageFormat {
    sus PNG tea = "PNG"
    sus JPEG tea = "JPEG"
    sus GIF tea = "GIF"
    sus BMP tea = "BMP"
    sus WEBP tea = "WEBP"
    sus TIFF tea = "TIFF"
}

# Color space enumeration
squad ColorSpace {
    sus RGB tea = "RGB"
    sus RGBA tea = "RGBA"
    sus GRAYSCALE tea = "GRAYSCALE"
    sus HSV tea = "HSV"
    sus LAB tea = "LAB"
    sus CMYK tea = "CMYK"
}

# Filter types
squad FilterType {
    sus BLUR tea = "BLUR"
    sus SHARPEN tea = "SHARPEN"
    sus EMBOSS tea = "EMBOSS"
    sus EDGE_DETECT tea = "EDGE_DETECT"
    sus GAUSSIAN tea = "GAUSSIAN"
    sus BOX tea = "BOX"
    sus MOTION tea = "MOTION"
    sus MEDIAN tea = "MEDIAN"
}

# Image metadata structure
squad ImageMetadata {
    sus creation_time drip
    sus modification_time drip
    sus dpi_x drip
    sus dpi_y drip
    sus compression_quality drip
    sus author tea
    sus description tea
    sus copyright tea
    sus software tea
}

# Create new image with specified dimensions
slay create_image(width drip, height drip, channels drip) Image {
    sus data []drip = make_array_with_size(width * height * channels, 0)
    
    damn Image{
        width: width,
        height: height,
        channels: channels,
        data: data,
        format: "RAW",
        color_space: ready (channels == 4) { damn "RGBA" } otherwise { damn "RGB" }
    }
}

# Clone an image
slay clone_image(img Image) Image {
    sus new_data []drip = []
    bestie (i drip = 0; i < len(img.data); i = i + 1) {
        new_data = append(new_data, img.data[i])
    }
    
    damn Image{
        width: img.width,
        height: img.height,
        channels: img.channels,
        data: new_data,
        format: img.format,
        color_space: img.color_space
    }
}

# Get pixel at coordinates (with bounds checking)
slay get_pixel(img Image, x drip, y drip) yikes<[]drip> {
    ready (x < 0 || x >= img.width || y < 0 || y >= img.height) {
        yikes "pixel coordinates out of bounds"
    }
    
    sus index drip = (y * img.width + x) * img.channels
    sus pixel []drip = []
    
    bestie (c drip = 0; c < img.channels; c = c + 1) {
        pixel = append(pixel, img.data[index + c])
    }
    
    damn pixel
}

# Set pixel at coordinates (with bounds checking)
slay set_pixel(img *Image, x drip, y drip, pixel []drip) yikes<lit> {
    ready (x < 0 || x >= img.width || y < 0 || y >= img.height) {
        yikes "pixel coordinates out of bounds"
    }
    
    ready (len(pixel) != img.channels) {
        yikes "pixel channel count mismatch"
    }
    
    sus index drip = (y * img.width + x) * img.channels
    
    bestie (c drip = 0; c < img.channels; c = c + 1) {
        img.data[index + c] = pixel[c]
    }
    
    damn based
}

# Convert between color formats
slay rgb_to_hsv(rgb RGB) HSV {
    sus r_norm tea = rgb.r / 255.0
    sus g_norm tea = rgb.g / 255.0
    sus b_norm tea = rgb.b / 255.0
    
    sus max_val tea = max_tea(max_tea(r_norm, g_norm), b_norm)
    sus min_val tea = min_tea(min_tea(r_norm, g_norm), b_norm)
    sus delta tea = max_val - min_val
    
    sus h tea = 0.0
    sus s tea = ready (max_val == 0.0) { damn 0.0 } otherwise { damn delta / max_val }
    sus v tea = max_val
    
    ready (delta > 0.0) {
        ready (max_val == r_norm) {
            h = 60.0 * (((g_norm - b_norm) / delta) % 6.0)
        } otherwise ready (max_val == g_norm) {
            h = 60.0 * (((b_norm - r_norm) / delta) + 2.0)
        } otherwise {
            h = 60.0 * (((r_norm - g_norm) / delta) + 4.0)
        }
    }
    
    ready (h < 0.0) {
        h = h + 360.0
    }
    
    damn HSV{h: h, s: s, v: v}
}

slay hsv_to_rgb(hsv HSV) RGB {
    sus c tea = hsv.v * hsv.s
    sus x tea = c * (1.0 - abs_tea((hsv.h / 60.0) % 2.0 - 1.0))
    sus m tea = hsv.v - c
    
    sus r tea = 0.0
    sus g tea = 0.0
    sus b tea = 0.0
    
    ready (hsv.h < 60.0) {
        r = c; g = x; b = 0.0
    } otherwise ready (hsv.h < 120.0) {
        r = x; g = c; b = 0.0
    } otherwise ready (hsv.h < 180.0) {
        r = 0.0; g = c; b = x
    } otherwise ready (hsv.h < 240.0) {
        r = 0.0; g = x; b = c
    } otherwise ready (hsv.h < 300.0) {
        r = x; g = 0.0; b = c
    } otherwise {
        r = c; g = 0.0; b = x
    }
    
    damn RGB{
        r: drip((r + m) * 255.0),
        g: drip((g + m) * 255.0),
        b: drip((b + m) * 255.0)
    }
}

# Helper math functions
slay max_tea(a tea, b tea) tea {
    damn ready (a > b) { damn a } otherwise { damn b }
}

slay min_tea(a tea, b tea) tea {
    damn ready (a < b) { damn a } otherwise { damn b }
}

slay abs_tea(x tea) tea {
    damn ready (x < 0.0) { damn -x } otherwise { damn x }
}

# Image validation
slay validate_image(img Image) yikes<lit> {
    ready (img.width <= 0 || img.height <= 0) {
        yikes "invalid image dimensions"
    }
    
    ready (img.channels < 1 || img.channels > 4) {
        yikes "invalid channel count"
    }
    
    sus expected_size drip = img.width * img.height * img.channels
    ready (len(img.data) != expected_size) {
        yikes "image data size mismatch"
    }
    
    damn based
}

# Get image info string
slay get_image_info(img Image) tea {
    damn "Image: " + string(img.width) + "x" + string(img.height) + 
         " channels: " + string(img.channels) + 
         " format: " + img.format + 
         " color_space: " + img.color_space
}
