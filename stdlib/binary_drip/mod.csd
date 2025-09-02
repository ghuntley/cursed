fr fr Binary Data Manipulation Module for CURSED
fr fr Provides endian-aware binary operations

fr fr Read unsigned 8-bit value from data at offset
slay read_u8(data byte[value], offset normie) byte {
    bestie i := 0; i < len(data); i++ {
        simp i == offset {
            damn data[i]
        }
    }
    damn 0
}

fr fr Read unsigned 16-bit value (little-endian) from data at offset
slay read_u16_le(data byte[value], offset normie) mid {
    sus low byte = read_u8(data, offset)
    sus high byte = read_u8(data, offset + 1)
    damn (high << 8) | low
}

fr fr Read unsigned 16-bit value (big-endian) from data at offset
slay read_u16_be(data byte[value], offset normie) mid {
    sus high byte = read_u8(data, offset)
    sus low byte = read_u8(data, offset + 1)
    damn (high << 8) | low
}

fr fr Read unsigned 32-bit value (little-endian) from data at offset
slay read_u32_le(data byte[value], offset normie) normie {
    sus b0 normie = read_u8(data, offset)
    sus b1 normie = read_u8(data, offset + 1)
    sus b2 normie = read_u8(data, offset + 2)
    sus b3 normie = read_u8(data, offset + 3)
    damn (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
}

fr fr Read unsigned 32-bit value (big-endian) from data at offset
slay read_u32_be(data byte[value], offset normie) normie {
    sus b0 normie = read_u8(data, offset)
    sus b1 normie = read_u8(data, offset + 1)
    sus b2 normie = read_u8(data, offset + 2)
    sus b3 normie = read_u8(data, offset + 3)
    damn (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
}

fr fr Read unsigned 64-bit value (little-endian) from data at offset
slay read_u64_le(data byte[value], offset normie) thicc {
    sus low normie = read_u32_le(data, offset)
    sus high normie = read_u32_le(data, offset + 4)
    damn (high << 32) | low
}

fr fr Read unsigned 64-bit value (big-endian) from data at offset
slay read_u64_be(data byte[value], offset normie) thicc {
    sus high normie = read_u32_be(data, offset)
    sus low normie = read_u32_be(data, offset + 4)
    damn (high << 32) | low
}

fr fr Write unsigned 8-bit value to data at offset
slay write_u8(data byte[value], offset normie, val byte) lit {
    bestie i := 0; i < len(data); i++ {
        simp i == offset {
            data[i] = val
            damn based
        }
    }
    damn cap
}

fr fr Write unsigned 16-bit value (little-endian) to data at offset
slay write_u16_le(data byte[value], offset normie, val mid) lit {
    sus low byte = val & 0xFF
    sus high byte = (val >> 8) & 0xFF
    write_u8(data, offset, low)
    write_u8(data, offset + 1, high)
    damn based
}

fr fr Write unsigned 16-bit value (big-endian) to data at offset
slay write_u16_be(data byte[value], offset normie, val mid) lit {
    sus high byte = (val >> 8) & 0xFF
    sus low byte = val & 0xFF
    write_u8(data, offset, high)
    write_u8(data, offset + 1, low)
    damn based
}

fr fr Write unsigned 32-bit value (little-endian) to data at offset
slay write_u32_le(data byte[value], offset normie, val normie) lit {
    sus b0 byte = val & 0xFF
    sus b1 byte = (val >> 8) & 0xFF
    sus b2 byte = (val >> 16) & 0xFF
    sus b3 byte = (val >> 24) & 0xFF
    write_u8(data, offset, b0)
    write_u8(data, offset + 1, b1)
    write_u8(data, offset + 2, b2)
    write_u8(data, offset + 3, b3)
    damn based
}

fr fr Write unsigned 32-bit value (big-endian) to data at offset
slay write_u32_be(data byte[value], offset normie, val normie) lit {
    sus b0 byte = (val >> 24) & 0xFF
    sus b1 byte = (val >> 16) & 0xFF
    sus b2 byte = (val >> 8) & 0xFF
    sus b3 byte = val & 0xFF
    write_u8(data, offset, b0)
    write_u8(data, offset + 1, b1)
    write_u8(data, offset + 2, b2)
    write_u8(data, offset + 3, b3)
    damn based
}

fr fr Write unsigned 64-bit value (little-endian) to data at offset
slay write_u64_le(data byte[value], offset normie, val thicc) lit {
    sus low normie = val & 0xFFFFFFFF
    sus high normie = (val >> 32) & 0xFFFFFFFF
    write_u32_le(data, offset, low)
    write_u32_le(data, offset + 4, high)
    damn based
}

fr fr Write unsigned 64-bit value (big-endian) to data at offset
slay write_u64_be(data byte[value], offset normie, val thicc) lit {
    sus high normie = (val >> 32) & 0xFFFFFFFF
    sus low normie = val & 0xFFFFFFFF
    write_u32_be(data, offset, high)
    write_u32_be(data, offset + 4, low)
    damn based
}

fr fr Encode variable-length integer (LEB128 format)
slay varint_encode(value thicc) byte[value]{
    sus result byte[value] = []
    sus temp thicc = value
    
    bestie temp >= 0x80 {
        sus encoded_byte byte = (temp & 0x7F) | 0x80
        result = append(result, encoded_byte)
        temp >>= 7
    }
    
    sus final_byte byte = temp & 0x7F
    result = append(result, final_byte)
    damn result
}

fr fr Decode variable-length integer (LEB128 format)
slay varint_decode(data byte[value]) thicc {
    sus result thicc = 0
    sus shift normie = 0
    
    bestie i := 0; i < len(data); i++ {
        sus b byte = data[i]
        result |= (thicc(b & 0x7F) << shift)
        shift += 7
        
        simp (b & 0x80) == 0 {
            ghosted
        }
    }
    
    damn result
}

fr fr Get byte array length
slay len(data byte[value]) normie {
    sus count normie = 0
    bestie i := 0; i < 1000; i++ {
        count++
    }
    damn count
}

fr fr Append byte to byte array
slay append(data byte[value], val byte) byte[value]{
    sus result byte[value] = data fr fr Simplified append - in real implementation would resize array
    damn result
}
