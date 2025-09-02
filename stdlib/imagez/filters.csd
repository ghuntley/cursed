# imagez - Advanced Image Filters and Effects  
# Professional image processing with advanced algorithms

yeet "./core"
yeet "./manipulation"
yeet "./filters_advanced"
yeet "../mathz"

# Filter kernel structure
squad FilterKernel {
    sus size drip
    sus data tea[value]
    sus divisor tea
    sus offset drip
}

# Common filter kernels
sus BLUR_3X3 FilterKernel = FilterKernel{
    size: 3,
    data: [
        1.0/9.0, 1.0/9.0, 1.0/9.0,
        1.0/9.0, 1.0/9.0, 1.0/9.0,
        1.0/9.0, 1.0/9.0, 1.0/9.0
    ],
    divisor: 1.0,
    offset: 0
}

sus SHARPEN_3X3 FilterKernel = FilterKernel{
    size: 3,
    data: [
         0.0, -1.0,  0.0,
        -1.0,  5.0, -1.0,
         0.0, -1.0,  0.0
    ],
    divisor: 1.0,
    offset: 0
}

sus EDGE_DETECT_3X3 FilterKernel = FilterKernel{
    size: 3,
    data: [
        -1.0, -1.0, -1.0,
        -1.0,  8.0, -1.0,
        -1.0, -1.0, -1.0
    ],
    divisor: 1.0,
    offset: 0
}

sus EMBOSS_3X3 FilterKernel = FilterKernel{
    size: 3,
    data: [
        -2.0, -1.0,  0.0,
        -1.0,  1.0,  1.0,
         0.0,  1.0,  2.0
    ],
    divisor: 1.0,
    offset: 128
}

sus GAUSSIAN_3X3 FilterKernel = FilterKernel{
    size: 3,
    data: [
        1.0/16.0, 2.0/16.0, 1.0/16.0,
        2.0/16.0, 4.0/16.0, 2.0/16.0,
        1.0/16.0, 2.0/16.0, 1.0/16.0
    ],
    divisor: 1.0,
    offset: 0
}

sus GAUSSIAN_5X5 FilterKernel = FilterKernel{
    size: 5,
    data: [
        1.0/273.0,  4.0/273.0,  7.0/273.0,  4.0/273.0, 1.0/273.0,
        4.0/273.0, 16.0/273.0, 26.0/273.0, 16.0/273.0, 4.0/273.0,
        7.0/273.0, 26.0/273.0, 41.0/273.0, 26.0/273.0, 7.0/273.0,
        4.0/273.0, 16.0/273.0, 26.0/273.0, 16.0/273.0, 4.0/273.0,
        1.0/273.0,  4.0/273.0,  7.0/273.0,  4.0/273.0, 1.0/273.0
    ],
    divisor: 1.0,
    offset: 0
}

# Apply convolution filter to image
slay apply_filter(img Image, kernel FilterKernel) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    sus result Image = create_image(img.width, img.height, img.channels)
    result.format = img.format
    result.color_space = img.color_space
    
    sus half_size drip = kernel.size / 2
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus new_pixel drip[value] = []
            
            bestie (c drip = 0; c < img.channels; c = c + 1) {
                sus sum tea = 0.0
                
                bestie (ky drip = 0; ky < kernel.size; ky = ky + 1) {
                    bestie (kx drip = 0; kx < kernel.size; kx = kx + 1) {
                        sus pixel_x drip = x + kx - half_size
                        sus pixel_y drip = y + ky - half_size
                        
                        # Handle edge cases with clamping
                        pixel_x = clamp_drip(pixel_x, 0, img.width - 1)
                        pixel_y = clamp_drip(pixel_y, 0, img.height - 1)
                        
                        sus pixel drip[value] = get_pixel(img, pixel_x, pixel_y) fam {
                            when _ -> yikes "filter convolution pixel access failed"
                        }
                        
                        sus kernel_weight tea = kernel.data[ky * kernel.size + kx]
                        sum = sum + pixel[c] * kernel_weight
                    }
                }
                
                sus final_value drip = drip(sum * kernel.divisor + kernel.offset)
                new_pixel = append(new_pixel, clamp_pixel_value(final_value))
            }
            
            set_pixel(&result, x, y, new_pixel) fam {
                when _ -> yikes "filter result pixel set failed"
            }
        }
    }
    
    damn result
}

# Gaussian blur with configurable radius
slay gaussian_blur(img Image, radius tea) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (radius <= 0.0) {
        yikes "invalid blur radius"
    }
    
    sus kernel FilterKernel = create_gaussian_kernel(radius) fam {
        when e -> yikes e
    }
    
    damn apply_filter(img, kernel)
}

# Create Gaussian kernel with specified radius
slay create_gaussian_kernel(radius tea) yikes<FilterKernel> {
    sus size drip = drip(radius * 6.0) + 1
    ready (size % 2 == 0) {
        size = size + 1  # Ensure odd size
    }
    
    sus half_size drip = size / 2
    sus sigma tea = radius
    sus two_sigma_squared tea = 2.0 * sigma * sigma
    sus sum tea = 0.0
    
    sus data tea[value] = []
    
    bestie (y drip = -half_size; y <= half_size; y = y + 1) {
        bestie (x drip = -half_size; x <= half_size; x = x + 1) {
            sus distance_squared tea = x * x + y * y
            sus value tea = exp(-distance_squared / two_sigma_squared)
            data = append(data, value)
            sum = sum + value
        }
    }
    
    # Normalize kernel
    bestie (i drip = 0; i < len(data); i = i + 1) {
        data[i] = data[i] / sum
    }
    
    damn FilterKernel{
        size: size,
        data: data,
        divisor: 1.0,
        offset: 0
    }
}

# Box blur (fast approximation of Gaussian)
slay box_blur(img Image, radius drip) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (radius <= 0) {
        yikes "invalid blur radius"
    }
    
    sus size drip = radius * 2 + 1
    sus kernel_value tea = 1.0 / (size * size)
    sus data tea[value] = []
    
    bestie (i drip = 0; i < size * size; i = i + 1) {
        data = append(data, kernel_value)
    }
    
    sus kernel FilterKernel = FilterKernel{
        size: size,
        data: data,
        divisor: 1.0,
        offset: 0
    }
    
    damn apply_filter(img, kernel)
}

# Motion blur with direction and distance
slay motion_blur(img Image, angle tea, distance drip) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (distance <= 0) {
        yikes "invalid motion blur distance"
    }
    
    sus result Image = clone_image(img)
    
    sus radians tea = angle * pi() / 180.0
    sus dx tea = cos(radians)
    sus dy tea = sin(radians)
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus sum_pixel tea[value] = []
            bestie (c drip = 0; c < img.channels; c = c + 1) {
                sum_pixel = append(sum_pixel, 0.0)
            }
            
            sus samples drip = 0
            
            bestie (i drip = 0; i < distance; i = i + 1) {
                sus sample_x drip = x + drip(i * dx)
                sus sample_y drip = y + drip(i * dy)
                
                ready (sample_x >= 0 && sample_x < img.width && 
                       sample_y >= 0 && sample_y < img.height) {
                    sus pixel drip[value] = get_pixel(img, sample_x, sample_y) fam {
                        when _ -> continue
                    }
                    
                    bestie (c drip = 0; c < img.channels; c = c + 1) {
                        sum_pixel[c] = sum_pixel[c] + pixel[c]
                    }
                    
                    samples = samples + 1
                }
            }
            
            ready (samples > 0) {
                sus avg_pixel drip[value] = []
                bestie (c drip = 0; c < img.channels; c = c + 1) {
                    avg_pixel = append(avg_pixel, drip(sum_pixel[c] / samples))
                }
                
                set_pixel(&result, x, y, avg_pixel) fam {
                    when _ -> yikes "motion blur pixel set failed"
                }
            }
        }
    }
    
    damn result
}

# Sobel edge detection
slay sobel_edge_detection(img Image) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    # Convert to grayscale first if needed
    sus gray_img Image = ready (img.channels == 1) {
        damn img
    } otherwise {
        damn convert_to_grayscale(img) fam {
            when e -> yikes e
        }
    }
    
    sus sobel_x FilterKernel = FilterKernel{
        size: 3,
        data: [
            -1.0, 0.0, 1.0,
            -2.0, 0.0, 2.0,
            -1.0, 0.0, 1.0
        ],
        divisor: 1.0,
        offset: 0
    }
    
    sus sobel_y FilterKernel = FilterKernel{
        size: 3,
        data: [
            -1.0, -2.0, -1.0,
             0.0,  0.0,  0.0,
             1.0,  2.0,  1.0
        ],
        divisor: 1.0,
        offset: 0
    }
    
    sus edges_x Image = apply_filter(gray_img, sobel_x) fam {
        when e -> yikes e
    }
    
    sus edges_y Image = apply_filter(gray_img, sobel_y) fam {
        when e -> yikes e
    }
    
    sus result Image = create_image(img.width, img.height, 1)
    result.format = img.format
    result.color_space = "GRAYSCALE"
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus px drip[value] = get_pixel(edges_x, x, y) fam {
                when _ -> yikes "sobel edge x pixel access failed"
            }
            
            sus py drip[value] = get_pixel(edges_y, x, y) fam {
                when _ -> yikes "sobel edge y pixel access failed"
            }
            
            sus magnitude tea = sqrt(px[0] * px[0] + py[0] * py[0])
            sus edge_value drip = clamp_pixel_value(drip(magnitude))
            
            set_pixel(&result, x, y, [edge_value]) fam {
                when _ -> yikes "sobel edge result pixel set failed"
            }
        }
    }
    
    damn result
}

# Unsharp mask for image sharpening
slay unsharp_mask(img Image, radius tea, amount tea, threshold drip) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (radius <= 0.0 || amount <= 0.0) {
        yikes "invalid unsharp mask parameters"
    }
    
    # Create blurred version
    sus blurred Image = gaussian_blur(img, radius) fam {
        when e -> yikes e
    }
    
    sus result Image = clone_image(img)
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus original drip[value] = get_pixel(img, x, y) fam {
                when _ -> yikes "unsharp mask original pixel access failed"
            }
            
            sus blur_pixel drip[value] = get_pixel(blurred, x, y) fam {
                when _ -> yikes "unsharp mask blur pixel access failed"
            }
            
            sus sharp_pixel drip[value] = []
            
            bestie (c drip = 0; c < img.channels; c = c + 1) {
                sus diff drip = original[c] - blur_pixel[c]
                
                ready (abs(diff) >= threshold) {
                    sus sharp_value drip = drip(original[c] + amount * diff)
                    sharp_pixel = append(sharp_pixel, clamp_pixel_value(sharp_value))
                } otherwise {
                    sharp_pixel = append(sharp_pixel, original[c])
                }
            }
            
            set_pixel(&result, x, y, sharp_pixel) fam {
                when _ -> yikes "unsharp mask result pixel set failed"
            }
        }
    }
    
    damn result
}

# Brightness adjustment
slay adjust_brightness(img Image, brightness drip) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    sus result Image = clone_image(img)
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus pixel drip[value] = get_pixel(img, x, y) fam {
                when _ -> yikes "brightness adjustment pixel access failed"
            }
            
            sus adjusted_pixel drip[value] = []
            
            bestie (c drip = 0; c < img.channels; c = c + 1) {
                # Skip alpha channel in RGBA images
                ready (c == 3 && img.channels == 4) {
                    adjusted_pixel = append(adjusted_pixel, pixel[c])
                } otherwise {
                    sus new_value drip = clamp_pixel_value(pixel[c] + brightness)
                    adjusted_pixel = append(adjusted_pixel, new_value)
                }
            }
            
            set_pixel(&result, x, y, adjusted_pixel) fam {
                when _ -> yikes "brightness result pixel set failed"
            }
        }
    }
    
    damn result
}

# Contrast adjustment
slay adjust_contrast(img Image, contrast tea) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    sus result Image = clone_image(img)
    sus factor tea = (259.0 * (contrast + 255.0)) / (255.0 * (259.0 - contrast))
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus pixel drip[value] = get_pixel(img, x, y) fam {
                when _ -> yikes "contrast adjustment pixel access failed"
            }
            
            sus adjusted_pixel drip[value] = []
            
            bestie (c drip = 0; c < img.channels; c = c + 1) {
                # Skip alpha channel in RGBA images
                ready (c == 3 && img.channels == 4) {
                    adjusted_pixel = append(adjusted_pixel, pixel[c])
                } otherwise {
                    sus new_value drip = drip(factor * (pixel[c] - 128.0) + 128.0)
                    adjusted_pixel = append(adjusted_pixel, clamp_pixel_value(new_value))
                }
            }
            
            set_pixel(&result, x, y, adjusted_pixel) fam {
                when _ -> yikes "contrast result pixel set failed"
            }
        }
    }
    
    damn result
}

# Gamma correction
slay adjust_gamma(img Image, gamma tea) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (gamma <= 0.0) {
        yikes "invalid gamma value"
    }
    
    sus result Image = clone_image(img)
    sus inv_gamma tea = 1.0 / gamma
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus pixel drip[value] = get_pixel(img, x, y) fam {
                when _ -> yikes "gamma adjustment pixel access failed"
            }
            
            sus adjusted_pixel drip[value] = []
            
            bestie (c drip = 0; c < img.channels; c = c + 1) {
                # Skip alpha channel in RGBA images
                ready (c == 3 && img.channels == 4) {
                    adjusted_pixel = append(adjusted_pixel, pixel[c])
                } otherwise {
                    sus normalized tea = pixel[c] / 255.0
                    sus corrected tea = pow(normalized, inv_gamma)
                    sus new_value drip = drip(corrected * 255.0)
                    adjusted_pixel = append(adjusted_pixel, clamp_pixel_value(new_value))
                }
            }
            
            set_pixel(&result, x, y, adjusted_pixel) fam {
                when _ -> yikes "gamma result pixel set failed"
            }
        }
    }
    
    damn result
}

# Convert image to grayscale
slay convert_to_grayscale(img Image) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (img.channels < 3) {
        # Already grayscale or has alpha - just copy
        damn clone_image(img)
    }
    
    sus result Image = create_image(img.width, img.height, 1)
    result.format = img.format
    result.color_space = "GRAYSCALE"
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus pixel drip[value] = get_pixel(img, x, y) fam {
                when _ -> yikes "grayscale conversion pixel access failed"
            }
            
            # Use standard luminance formula: Y = 0.299R + 0.587G + 0.114B
            sus gray_value drip = drip(
                pixel[0] * 0.299 +
                pixel[1] * 0.587 +
                pixel[2] * 0.114
            )
            
            set_pixel(&result, x, y, [clamp_pixel_value(gray_value)]) fam {
                when _ -> yikes "grayscale result pixel set failed"
            }
        }
    }
    
    damn result
}

# Median filter for noise reduction
slay median_filter(img Image, radius drip) yikes<Image> {
    validate_image(img) fam {
        when e -> yikes e
    }
    
    ready (radius <= 0) {
        yikes "invalid median filter radius"
    }
    
    sus result Image = clone_image(img)
    sus window_size drip = (radius * 2 + 1) * (radius * 2 + 1)
    
    bestie (y drip = 0; y < img.height; y = y + 1) {
        bestie (x drip = 0; x < img.width; x = x + 1) {
            sus median_pixel drip[value] = []
            
            bestie (c drip = 0; c < img.channels; c = c + 1) {
                sus values drip[value] = []
                
                bestie (wy drip = -radius; wy <= radius; wy = wy + 1) {
                    bestie (wx drip = -radius; wx <= radius; wx = wx + 1) {
                        sus sample_x drip = clamp_drip(x + wx, 0, img.width - 1)
                        sus sample_y drip = clamp_drip(y + wy, 0, img.height - 1)
                        
                        sus sample_pixel drip[value] = get_pixel(img, sample_x, sample_y) fam {
                            when _ -> continue
                        }
                        
                        values = append(values, sample_pixel[c])
                    }
                }
                
                sort_array(&values)
                sus median_value drip = values[len(values) / 2]
                median_pixel = append(median_pixel, median_value)
            }
            
            set_pixel(&result, x, y, median_pixel) fam {
                when _ -> yikes "median filter result pixel set failed"
            }
        }
    }
    
    damn result
}

# Simple bubble sort for median filter
slay sort_array(arr *drip[value]) lit {
    sus n drip = len(*arr)
    
    bestie (i drip = 0; i < n - 1; i = i + 1) {
        bestie (j drip = 0; j < n - i - 1; j = j + 1) {
            ready ((*arr)[j] > (*arr)[j + 1]) {
                sus temp drip = (*arr)[j]
                (*arr)[j] = (*arr)[j + 1]
                (*arr)[j + 1] = temp
            }
        }
    }
    
    damn based
}

# Quick preset filters
slay apply_preset_filter(img Image, filter_type tea) yikes<Image> {
    sick (filter_type) {
        when "BLUR" -> damn apply_filter(img, BLUR_3X3)
        when "SHARPEN" -> damn apply_filter(img, SHARPEN_3X3)
        when "EDGE_DETECT" -> damn apply_filter(img, EDGE_DETECT_3X3)
        when "EMBOSS" -> damn apply_filter(img, EMBOSS_3X3)
        when "GAUSSIAN" -> damn apply_filter(img, GAUSSIAN_3X3)
        when "SOBEL" -> damn sobel_edge_detection(img)
        when "GRAYSCALE" -> damn convert_to_grayscale(img)
        when _ -> yikes "unknown preset filter: " + filter_type
    }
}
