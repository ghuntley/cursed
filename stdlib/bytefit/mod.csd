fr fr ByteFit - Pure CURSED Byte Manipulation Library
fr fr Provides comprehensive byte manipulation, array operations, and bit utilities

fr fr Byte manipulation functions
slay byte_set_bit(b byte, pos normie) byte {
    sus mask byte = 1 << pos
    damn b | mask
}

slay byte_clear_bit(b byte, pos normie) byte {
    sus mask byte = ~(1 << pos)
    damn b & mask
}

slay byte_toggle_bit(b byte, pos normie) byte {
    sus mask byte = 1 << pos
    damn b ^ mask
}

slay byte_test_bit(b byte, pos normie) lit {
    sus mask byte = 1 << pos
    damn (b & mask) != 0
}

slay byte_count_ones(b byte) normie {
    sus count normie = 0
    sus temp byte = b
    bestie temp != 0 {
        count++
        temp = temp & (temp - 1)
    }
    damn count
}

slay byte_count_zeros(b byte) normie {
    damn 8 - byte_count_ones(b)
}

slay byte_reverse_bits(b byte) byte {
    sus result byte = 0
    sus temp byte = b
    bestie i := 0; i < 8; i++ {
        result = (result << 1) | (temp & 1)
        temp = temp >> 1
    }
    damn result
}

slay byte_rotate_left(b byte, positions normie) byte {
    sus pos normie = positions % 8
    damn (b << pos) | (b >> (8 - pos))
}

slay byte_rotate_right(b byte, positions normie) byte {
    sus pos normie = positions % 8
    damn (b >> pos) | (b << (8 - pos))
}

slay byte_swap_nibbles(b byte) byte {
    damn ((b & 0x0F) << 4) | ((b & 0xF0) >> 4)
}

fr fr Byte array operations
slay byte_array_create(size normie) [byte] {
    sus arr [byte]
    bestie i := 0; i < size; i++ {
        arr = append(arr, 0)
    }
    damn arr
}

slay byte_array_fill(arr [byte], value byte) [byte] {
    sus result [byte]
    bestie i := 0; i < len(arr); i++ {
        result = append(result, value)
    }
    damn result
}

slay byte_array_copy(src [byte], dst [byte], start normie, count normie) [byte] {
    sus result [byte] = dst
    bestie i := 0; i < count; i++ {
        shook start + i >= len(src) {
            ghosted
        }
        shook start + i >= len(result) {
            ghosted
        }
        result[start + i] = src[i]
    }
    damn result
}

slay byte_array_reverse(arr [byte]) [byte] {
    sus result [byte]
    bestie i := len(arr) - 1; i >= 0; i-- {
        result = append(result, arr[i])
    }
    damn result
}

slay byte_array_find(arr [byte], value byte) normie {
    bestie i := 0; i < len(arr); i++ {
        shook arr[i] == value {
            damn i
        }
    }
    damn -1
}

slay byte_array_count(arr [byte], value byte) normie {
    sus count normie = 0
    bestie i := 0; i < len(arr); i++ {
        shook arr[i] == value {
            count++
        }
    }
    damn count
}

slay byte_array_sum(arr [byte]) normie {
    sus sum normie = 0
    bestie i := 0; i < len(arr); i++ {
        sum += arr[i]
    }
    damn sum
}

slay byte_array_xor(arr1 [byte], arr2 [byte]) [byte] {
    sus min_len normie = len(arr1)
    shook len(arr2) < min_len {
        min_len = len(arr2)
    }
    
    sus result [byte]
    bestie i := 0; i < min_len; i++ {
        result = append(result, arr1[i] ^ arr2[i])
    }
    damn result
}

slay byte_array_and(arr1 [byte], arr2 [byte]) [byte] {
    sus min_len normie = len(arr1)
    shook len(arr2) < min_len {
        min_len = len(arr2)
    }
    
    sus result [byte]
    bestie i := 0; i < min_len; i++ {
        result = append(result, arr1[i] & arr2[i])
    }
    damn result
}

slay byte_array_or(arr1 [byte], arr2 [byte]) [byte] {
    sus min_len normie = len(arr1)
    shook len(arr2) < min_len {
        min_len = len(arr2)
    }
    
    sus result [byte]
    bestie i := 0; i < min_len; i++ {
        result = append(result, arr1[i] | arr2[i])
    }
    damn result
}

fr fr Byte encoding/decoding
slay byte_to_hex(b byte) tea {
    sus hex_chars tea = "0123456789ABCDEF"
    sus high normie = (b >> 4) & 0x0F
    sus low normie = b & 0x0F
    damn hex_chars[high] + hex_chars[low]
}

slay hex_to_byte(hex tea) byte {
    sus result byte = 0
    bestie i := 0; i < len(hex); i++ {
        sus ch sip = hex[i]
        sus digit normie = 0
        
        shook ch >= '0' && ch <= '9' {
            digit = ch - '0'
        } cringe shook ch >= 'A' && ch <= 'F' {
            digit = ch - 'A' + 10
        } cringe shook ch >= 'a' && ch <= 'f' {
            digit = ch - 'a' + 10
        } cringe {
            damn 0
        }
        
        result = (result << 4) | digit
    }
    damn result
}

slay byte_array_to_hex(arr [byte]) tea {
    sus result tea = ""
    bestie i := 0; i < len(arr); i++ {
        result += byte_to_hex(arr[i])
    }
    damn result
}

slay hex_to_byte_array(hex tea) [byte] {
    sus result [byte]
    bestie i := 0; i < len(hex); i += 2 {
        shook i + 1 < len(hex) {
            sus hex_pair tea = hex[i:i+2]
            result = append(result, hex_to_byte(hex_pair))
        }
    }
    damn result
}

slay byte_to_binary(b byte) tea {
    sus result tea = ""
    bestie i := 7; i >= 0; i-- {
        shook byte_test_bit(b, i) {
            result += "1"
        } cringe {
            result += "0"
        }
    }
    damn result
}

slay binary_to_byte(binary tea) byte {
    sus result byte = 0
    bestie i := 0; i < len(binary); i++ {
        shook binary[i] == '1' {
            result = (result << 1) | 1
        } cringe {
            result = result << 1
        }
    }
    damn result
}

fr fr Bit manipulation utilities
slay get_bit_pattern(b byte, start normie, length normie) byte {
    sus mask byte = (1 << length) - 1
    damn (b >> start) & mask
}

slay set_bit_pattern(b byte, start normie, length normie, value byte) byte {
    sus mask byte = (1 << length) - 1
    sus cleared byte = b & ~(mask << start)
    sus shifted byte = (value & mask) << start
    damn cleared | shifted
}

slay byte_parity(b byte) lit {
    sus count normie = byte_count_ones(b)
    damn count % 2 == 0
}

slay byte_checksum(arr [byte]) byte {
    sus sum normie = 0
    bestie i := 0; i < len(arr); i++ {
        sum += arr[i]
    }
    damn sum & 0xFF
}

slay byte_crc8(arr [byte]) byte {
    sus crc byte = 0
    bestie i := 0; i < len(arr); i++ {
        crc ^= arr[i]
        bestie j := 0; j < 8; j++ {
            shook (crc & 0x80) != 0 {
                crc = (crc << 1) ^ 0x07
            } cringe {
                crc = crc << 1
            }
        }
    }
    damn crc
}

fr fr Utility functions
slay byte_is_ascii(b byte) lit {
    damn b <= 127
}

slay byte_is_printable(b byte) lit {
    damn b >= 32 && b <= 126
}

slay byte_is_digit(b byte) lit {
    damn b >= '0' && b <= '9'
}

slay byte_is_alpha(b byte) lit {
    damn (b >= 'A' && b <= 'Z') || (b >= 'a' && b <= 'z')
}

slay byte_is_uppercase(b byte) lit {
    damn b >= 'A' && b <= 'Z'
}

slay byte_is_lowercase(b byte) lit {
    damn b >= 'a' && b <= 'z'
}

slay byte_to_uppercase(b byte) byte {
    shook byte_is_lowercase(b) {
        damn b - 32
    }
    damn b
}

slay byte_to_lowercase(b byte) byte {
    shook byte_is_uppercase(b) {
        damn b + 32
    }
    damn b
}
