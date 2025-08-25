yeet "vibez"

fr fr Simple BMP decoder validation
slay read_uint16_le(data []byte, offset normie) normie {
    vibe_check offset + 1 >= len(data) {
        damn 0
    }
    damn data[offset] | (data[offset + 1] << 8)
}

slay read_uint32_le(data []byte, offset normie) normie {
    vibe_check offset + 3 >= len(data) {
        damn 0
    }
    damn data[offset] | (data[offset + 1] << 8) | (data[offset + 2] << 16) | (data[offset + 3] << 24)
}

slay make_byte_array(size normie) []byte {
    sus result []byte = []
    sus i normie = 0
    bestie (i < size) {
        result = append(result, 0)
        i = i + 1
    }
    damn result
}

slay decode_bmp_basic(data []byte) (normie, normie, []byte) {
    vibe_check len(data) < 54 {
        vibez.spill("BMP data too small")
        damn 0, 0, []
    }
    
    vibe_check data[0] != 0x42 || data[1] != 0x4D {
        vibez.spill("Invalid BMP signature")
        damn 0, 0, []
    }
    
    sus width normie = read_uint32_le(data, 18)
    sus height normie = read_uint32_le(data, 22)
    sus bits_per_pixel normie = read_uint16_le(data, 28)
    
    vibe_check bits_per_pixel != 24 && bits_per_pixel != 32 {
        vibez.spill("Unsupported BMP format:", bits_per_pixel, "bits per pixel")
        damn 0, 0, []
    }
    
    vibez.spill("BMP decoded successfully:", width, "x", height, "pixels")
    
    sus bytes_per_pixel normie = bits_per_pixel / 8
    sus pixels []byte = make_byte_array(width * height * 3)
    
    damn width, height, pixels
}

slay main() {
    vibez.spill("Testing BMP decoder functionality...")
    
    fr fr Create minimal BMP header for testing
    sus test_bmp []byte = [
        0x42, 0x4D,             fr fr "BM" signature
        0x36, 0x00, 0x00, 0x00, fr fr File size
        0x00, 0x00, 0x00, 0x00, fr fr Reserved
        0x36, 0x00, 0x00, 0x00, fr fr Pixel offset
        0x28, 0x00, 0x00, 0x00, fr fr Header size
        0x02, 0x00, 0x00, 0x00, fr fr Width (2)
        0x02, 0x00, 0x00, 0x00, fr fr Height (2)
        0x01, 0x00,             fr fr Planes
        0x18, 0x00,             fr fr 24 bits per pixel
        0x00, 0x00, 0x00, 0x00, fr fr Compression
        0x00, 0x00, 0x00, 0x00, fr fr Image size
        0x00, 0x00, 0x00, 0x00, fr fr X pixels per meter
        0x00, 0x00, 0x00, 0x00, fr fr Y pixels per meter
        0x00, 0x00, 0x00, 0x00, fr fr Colors used
        0x00, 0x00, 0x00, 0x00, fr fr Important colors
        0xFF, 0x00, 0x00, 0x00, fr fr Pixel data
        0xFF, 0x00, 0x00, 0x00
    ]
    
    sus width, height, pixels normie, normie, []byte = decode_bmp_basic(test_bmp)
    
    vibez.spill("Width:", width)
    vibez.spill("Height:", height)
    vibez.spill("Pixel array length:", len(pixels))
    
    vibe_check width == 2 && height == 2 {
        vibez.spill("✓ BMP decoding SUCCESS!")
    } damn {
        vibez.spill("✗ BMP decoding FAILED!")
    }
    
    fr fr Test error cases
    sus empty_data []byte = []
    sus w2, h2, p2 normie, normie, []byte = decode_bmp_basic(empty_data)
    vibe_check w2 == 0 && h2 == 0 {
        vibez.spill("✓ Error handling SUCCESS!")
    } damn {
        vibez.spill("✗ Error handling FAILED!")
    }
    
    vibez.spill("BMP decoder validation completed!")
}
