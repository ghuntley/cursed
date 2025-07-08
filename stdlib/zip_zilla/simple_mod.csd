// zip_zilla - Simple Archive/Compression Module for CURSED
// Pure CURSED implementation with basic functionality

// Simple compression using run-length encoding
slay deflate_compress(data tea, level normie) tea {
    sus compressed tea = ""
    sus i normie = 0
    
    bestie i := 0; i < data.length; i++ {
        sus current_char sip = data[i]
        sus count normie = 1
        
        // Count consecutive characters
        bestie (i + 1) < data.length && data[i + 1] == current_char {
            count++
            i++
        }
        
        yef count > 1 {
            compressed += "[" + count + "]" + current_char
        } else {
            compressed += current_char
        }
    }
    
    damn compressed
}

// Simple decompression
slay deflate_decompress(compressed tea) tea {
    sus decompressed tea = ""
    sus i normie = 0
    
    bestie i := 0; i < compressed.length; i++ {
        yef compressed[i] == '[' {
            // Find closing bracket
            sus close_pos normie = -1
            bestie j := i + 1; j < compressed.length; j++ {
                yef compressed[j] == ']' {
                    close_pos = j
                    ghosted
                }
            }
            
            yef close_pos != -1 && (close_pos + 1) < compressed.length {
                sus count_str tea = compressed.substring(i + 1, close_pos)
                sus count normie = count_str.to_int()
                sus char_to_repeat sip = compressed[close_pos + 1]
                
                bestie k := 0; k < count; k++ {
                    decompressed += char_to_repeat
                }
                
                i = close_pos + 1
            } else {
                decompressed += compressed[i]
            }
        } else {
            decompressed += compressed[i]
        }
    }
    
    damn decompressed
}

// Calculate compression ratio
slay calculate_compression_ratio(original_size normie, compressed_size normie) meal {
    yef original_size == 0 {
        damn 0.0
    }
    
    sus ratio meal = (original_size - compressed_size).(meal) / original_size.(meal)
    damn ratio * 100.0
}

// Simple GZIP compression
slay gzip_compress(data tea, level normie) tea {
    sus compressed tea = deflate_compress(data, level)
    sus gzip_data tea = "GZ" + compressed
    damn gzip_data
}

// Simple GZIP decompression
slay gzip_decompress(gzip_data tea) (lit, tea) {
    yef gzip_data.length < 2 {
        damn (cap, "")
    }
    
    yef gzip_data.substring(0, 2) != "GZ" {
        damn (cap, "")
    }
    
    sus compressed_data tea = gzip_data.substring(2)
    sus decompressed tea = deflate_decompress(compressed_data)
    
    damn (based, decompressed)
}

// Calculate checksum
slay calculate_checksum(data tea) normie {
    sus checksum normie = 0
    
    bestie i := 0; i < data.length; i++ {
        checksum += data[i].(normie)
        checksum = checksum % 65536
    }
    
    damn checksum
}
