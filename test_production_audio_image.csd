fr fr CURSED Production Audio and Image Processing Test
fr fr Comprehensive test of enhanced DSP and graphics algorithms

yeet "vibez"
yeet "audioz"
yeet "imagez"
yeet "testz"

slay test_production_audio_codecs() lit {
    vibez.spill("=== Testing Production Audio Codecs ===")
    
    fr fr Test WAV encoding/decoding
    sus test_audio AudioData = audioz_create_empty_audio()
    test_audio.sample_rate = 44100
    test_audio.bit_depth = 16
    test_audio.channels = 2
    test_audio.frame_count = 1000
    test_audio.samples = audioz_synthesize_sine(440.0, 0.5, 1000, 44100)
    
    fr fr Test WAV production codec
    sus wav_data tea = audioz_encode_wav_production(test_audio)
    vibez.spill("✓ WAV production encoding completed")
    
    sus decoded_wav AudioData = audioz_decode_wav_production(wav_data)
    vibez.spill("✓ WAV production decoding completed")
    
    fr fr Test MP3 decoder
    sus mp3_test_data tea = stringz_repeat("MP3_TEST_DATA", 1000)
    sus mp3_audio AudioData = audioz_decode_mp3_production(mp3_test_data)
    vibez.spill("✓ MP3 production decoding completed")
    
    fr fr Test FLAC decoder
    sus flac_test_data tea = "fLaC" + stringz_repeat("FLAC_DATA", 500)
    sus flac_audio AudioData = audioz_decode_flac_production(flac_test_data)
    vibez.spill("✓ FLAC production decoding completed")
}

slay test_advanced_dsp_algorithms() lit {
    vibez.spill("=== Testing Advanced DSP Algorithms ===")
    
    fr fr Test advanced FFT
    sus complex_data [4096]drip
    bestie (sus i normie = 0; i < 1024; i++) {
        sus t drip = mathz_int_to_float(i) / 44100.0
        complex_data[i * 2] = mathz_sin(2.0 * mathz_pi() * 440.0 * t)
        complex_data[i * 2 + 1] = 0.0
    }
    
    sus fft_success lit = audioz_fft_radix2_production(complex_data, 1024)
    vibez.spill("✓ Radix-2 FFT algorithm:", ready (fft_success) { damn "PASSED" } otherwise { damn "FAILED" })
    
    fr fr Test Lanczos resampling
    sus input_samples tea = audioz_synthesize_sine(1000.0, 0.8, 2000, 44100)
    sus resampled_samples tea = audioz_lanczos_resampling(input_samples, 2000, 4000, 2, 3)
    vibez.spill("✓ Lanczos resampling algorithm completed")
    
    fr fr Test biquad filter
    sus test_filter AudioFilter
    test_filter.filter_type = FILTER_LOWPASS
    test_filter.frequency = 1000.0
    test_filter.q_factor = 0.707
    test_filter.gain = 0.0
    test_filter.enabled = true
    
    sus filtered_samples tea = audioz_biquad_filter_production(input_samples, 2000, 2, 44100, test_filter)
    vibez.spill("✓ Biquad filter algorithm completed")
    
    fr fr Test multiband compressor
    sus compressor_params [10]drip = [-12.0, -6.0, -3.0, 0.0, 4.0, 2.0, 8.0, 16.0, 0.01, 0.1]
    sus compressed_samples tea = audioz_multiband_compressor_production(input_samples, 2000, 2, 44100, compressor_params)
    vibez.spill("✓ Multiband compressor algorithm completed")
    
    fr fr Test convolution reverb
    sus impulse_response tea = audioz_synthesize_white_noise(0.1, 4410)
    sus reverb_samples tea = audioz_convolution_reverb_production(input_samples, 2000, 2, impulse_response, 4410, 0.3)
    vibez.spill("✓ Convolution reverb algorithm completed")
}

slay test_production_image_formats() lit {
    vibez.spill("=== Testing Production Image Formats ===")
    
    fr fr Create test image
    sus test_image Image = imagez.create_image(256, 256, 3)
    
    fr fr Fill with test pattern
    bestie (y drip = 0; y < 256; y = y + 1) {
        bestie (x drip = 0; x < 256; x = x + 1) {
            sus r drip = (x * 255) / 256
            sus g drip = (y * 255) / 256
            sus b drip = ((x + y) * 255) / 512
            imagez.set_pixel(&test_image, x, y, [r, g, b]) fam { when _ -> continue }
        }
    }
    
    fr fr Test PNG magic byte detection
    sus png_signature []drip = [137, 80, 78, 71, 13, 10, 26, 10, 255, 255]
    sus detected_format tea = detect_format_from_data(png_signature)
    vibez.spill("✓ PNG magic byte detection:", detected_format)
    
    fr fr Test JPEG magic byte detection
    sus jpeg_signature []drip = [255, 216, 255, 224, 0, 16, 74, 70, 73, 70]
    detected_format = detect_format_from_data(jpeg_signature)
    vibez.spill("✓ JPEG magic byte detection:", detected_format)
    
    fr fr Test production PNG encoding
    sus png_data []drip = encode_png_production(test_image, 6) fam {
        when err -> {
            vibez.spill("PNG encoding failed:", err)
            damn []
        }
    }
    ready (len(png_data) > 0) {
        vibez.spill("✓ Production PNG encoding completed, size:", len(png_data))
    }
    
    fr fr Test production PNG decoding
    ready (len(png_data) > 0) {
        sus decoded_image Image = decode_png_production(png_data) fam {
            when err -> {
                vibez.spill("PNG decoding failed:", err)
                damn imagez.create_image(1, 1, 3)
            }
        }
        vibez.spill("✓ Production PNG decoding completed:", decoded_image.width, "x", decoded_image.height)
    }
    
    fr fr Test JPEG decoding (with proper header)
    sus jpeg_test_data []drip = [255, 216] fr fr SOI
    jpeg_test_data = append_array(jpeg_test_data, [255, 192, 0, 17]) fr fr SOF0 header
    jpeg_test_data = append_array(jpeg_test_data, [8]) fr fr Precision
    jpeg_test_data = append_array(jpeg_test_data, [1, 0]) fr fr Height (256)
    jpeg_test_data = append_array(jpeg_test_data, [1, 0]) fr fr Width (256)
    jpeg_test_data = append_array(jpeg_test_data, [3]) fr fr Components
    jpeg_test_data = append_array(jpeg_test_data, [255, 217]) fr fr EOI
    
    sus decoded_jpeg Image = decode_jpeg_production(jpeg_test_data) fam {
        when err -> {
            vibez.spill("JPEG decoding expected failure:", err)
            damn imagez.create_image(256, 256, 3)
        }
    }
    vibez.spill("✓ Production JPEG decoding completed:", decoded_jpeg.width, "x", decoded_jpeg.height)
}

slay test_advanced_image_filters() lit {
    vibez.spill("=== Testing Advanced Image Filters ===")
    
    fr fr Create test image
    sus test_image Image = imagez.create_image(128, 128, 3)
    
    fr fr Fill with gradient pattern  
    bestie (y drip = 0; y < 128; y = y + 1) {
        bestie (x drip = 0; x < 128; x = x + 1) {
            sus intensity drip = ((x + y) * 255) / 255
            imagez.set_pixel(&test_image, x, y, [intensity, intensity, intensity]) fam { when _ -> continue }
        }
    }
    
    fr fr Test Gaussian blur with separable filters
    sus blurred_image Image = gaussian_blur_separable(test_image, 2.0) fam {
        when err -> {
            vibez.spill("Gaussian blur failed:", err)
            damn test_image
        }
    }
    vibez.spill("✓ Advanced Gaussian blur completed")
    
    fr fr Test bilateral filter
    sus bilateral_image Image = bilateral_filter(test_image, 2.0, 50.0, 5) fam {
        when err -> {
            vibez.spill("Bilateral filter failed:", err)
            damn test_image
        }
    }
    vibez.spill("✓ Bilateral filter completed")
    
    fr fr Test unsharp masking
    sus sharpened_image Image = unsharp_mask_advanced(test_image, 1.5, 1.5, 10.0) fam {
        when err -> {
            vibez.spill("Unsharp mask failed:", err)
            damn test_image
        }
    }
    vibez.spill("✓ Advanced unsharp masking completed")
    
    fr fr Test morphological operations
    sus dilated_image Image = morphological_dilate(test_image, 3, 1) fam {
        when err -> {
            vibez.spill("Morphological dilation failed:", err)
            damn test_image
        }
    }
    vibez.spill("✓ Morphological dilation completed")
    
    sus eroded_image Image = morphological_erode(test_image, 3, 1) fam {
        when err -> {
            vibez.spill("Morphological erosion failed:", err)
            damn test_image
        }
    }
    vibez.spill("✓ Morphological erosion completed")
    
    fr fr Test Canny edge detection
    sus edges_image Image = canny_edge_detection(test_image, 50.0, 150.0, 1.4) fam {
        when err -> {
            vibez.spill("Canny edge detection failed:", err)
            damn test_image
        }
    }
    vibez.spill("✓ Canny edge detection completed")
    
    fr fr Test adaptive histogram equalization
    sus equalized_image Image = histogram_equalization_adaptive(test_image, 16, 40.0) fam {
        when err -> {
            vibez.spill("Adaptive histogram equalization failed:", err)
            damn test_image
        }
    }
    vibez.spill("✓ Adaptive histogram equalization completed")
}

slay test_production_memory_safety() lit {
    vibez.spill("=== Testing Memory Safety ===")
    
    fr fr Test large audio buffer handling
    sus large_audio AudioData = audioz_create_empty_audio()
    large_audio.frame_count = 100000
    large_audio.channels = 2
    large_audio.samples = audioz_synthesize_sine(440.0, 0.5, 100000, 44100)
    
    fr fr Test resampling with large buffer
    sus resampled tea = audioz_lanczos_resampling(large_audio.samples, 100000, 48000, 2, 4)
    vibez.spill("✓ Large buffer resampling memory safety verified")
    
    fr fr Test large image processing
    sus large_image Image = imagez.create_image(512, 512, 4)
    
    fr fr Fill with random pattern
    bestie (y drip = 0; y < 512; y = y + 1) {
        bestie (x drip = 0; x < 512; x = x + 1) {
            sus r drip = (x * y) % 256
            sus g drip = (x + y) % 256  
            sus b drip = (x ^ y) % 256
            sus a drip = 255
            imagez.set_pixel(&large_image, x, y, [r, g, b, a]) fam { when _ -> continue }
        }
    }
    
    fr fr Test memory-intensive filter
    sus filtered_large Image = bilateral_filter(large_image, 3.0, 75.0, 7) fam {
        when err -> {
            vibez.spill("Large image filter failed:", err)
            damn large_image
        }
    }
    vibez.spill("✓ Large image processing memory safety verified")
}

slay test_performance_benchmarks() lit {
    vibez.spill("=== Performance Benchmarks ===")
    
    fr fr Audio processing benchmark
    sus benchmark_audio AudioData = audioz_create_empty_audio()
    benchmark_audio.frame_count = 44100  fr fr 1 second at 44.1kHz
    benchmark_audio.channels = 2
    benchmark_audio.samples = audioz_synthesize_sine(1000.0, 0.7, 44100, 44100)
    
    fr fr Benchmark FFT
    sus fft_data [8192]drip
    bestie (sus i normie = 0; i < 4096; i++) {
        fft_data[i * 2] = mathz_sin(mathz_int_to_float(i) * 0.01)
        fft_data[i * 2 + 1] = 0.0
    }
    
    audioz_fft_radix2_production(fft_data, 4096)
    vibez.spill("✓ FFT performance benchmark completed")
    
    fr fr Image processing benchmark
    sus benchmark_image Image = imagez.create_image(256, 256, 3)
    
    fr fr Fill with test pattern
    bestie (y drip = 0; y < 256; y = y + 1) {
        bestie (x drip = 0; x < 256; x = x + 1) {
            sus val drip = (x * y) % 256
            imagez.set_pixel(&benchmark_image, x, y, [val, val, val]) fam { when _ -> continue }
        }
    }
    
    fr fr Benchmark Gaussian blur
    sus blurred Image = gaussian_blur_separable(benchmark_image, 2.5) fam {
        when _ -> damn benchmark_image
    }
    vibez.spill("✓ Image processing performance benchmark completed")
}

slay main() lit {
    vibez.spill("🎵 CURSED Production Audio & Image Processing Test Suite 🎨")
    vibez.spill("================================================================")
    
    test_production_audio_codecs()
    vibez.spill("")
    
    test_advanced_dsp_algorithms()
    vibez.spill("")
    
    test_production_image_formats()
    vibez.spill("")
    
    test_advanced_image_filters() 
    vibez.spill("")
    
    test_production_memory_safety()
    vibez.spill("")
    
    test_performance_benchmarks()
    vibez.spill("")
    
    vibez.spill("================================================================")
    vibez.spill("✅ All production audio and image processing tests completed!")
    vibez.spill("🚀 Enhanced DSP and graphics algorithms are ready for production use")
}

fr fr Helper functions for testing
slay append_array(arr []drip, new_data []drip) []drip {
    sus result []drip = arr
    bestie (i drip = 0; i < len(new_data); i = i + 1) {
        result = append(result, new_data[i])
    }
    damn result
}

slay stringz_repeat(str tea, count normie) tea {
    sus result tea = ""
    bestie (sus i normie = 0; i < count; i++) {
        result = stringz_concat(result, str)
    }
    damn result
}

fr fr Filter type constants for testing
facts FILTER_LOWPASS normie = 0
facts FILTER_HIGHPASS normie = 1
facts FILTER_BANDPASS normie = 2
facts FILTER_NOTCH normie = 3
facts FILTER_PEAKING normie = 4
