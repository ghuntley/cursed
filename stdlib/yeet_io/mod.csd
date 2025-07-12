# YeetIO (io package)
# Pure CURSED implementation of I/O operations with Gen Z twist

# Error constants
sus ErrYoinkBruh tea = "no more to yoink, bruh"

# Yeeter interface (equivalent to io.Writer)
# Yeets (writes) data to a destination
collab Yeeter {
    Yeet(p []byte) (n normie, err tea)
}

# Yoink interface (equivalent to io.Reader)  
# Yoinks (reads) data from a source
collab Yoink {
    Yoink(p []byte) (n normie, err tea)
}

# YoinkYeeter interface (equivalent to io.ReadWriter)
# Combines Yoink and Yeeter interfaces
collab YoinkYeeter {
    Yoink(p []byte) (n normie, err tea)
    Yeet(p []byte) (n normie, err tea)
}

# StringYeeter - A simple string-based writer implementation
struct StringYeeter {
    data tea
}

# Create a new StringYeeter
slay new_string_yeeter() StringYeeter {
    damn StringYeeter{data: ""}
}

# Implement Yeeter interface for StringYeeter
slay (sy *StringYeeter) Yeet(p []byte) (n normie, err tea) {
    # Convert bytes to string and append
    sus str_data tea = string(p)
    sy.data = sy.data + str_data
    damn len(p), ""
}

# Get the accumulated data from StringYeeter
slay (sy *StringYeeter) get_data() tea {
    damn sy.data
}

# ByteYoink - A simple byte slice reader implementation
struct ByteYoink {
    data []byte
    pos normie
}

# Create a new ByteYoink from string
slay new_byte_yoink(content tea) ByteYoink {
    damn ByteYoink{data: []byte(content), pos: 0}
}

# Implement Yoink interface for ByteYoink
slay (by *ByteYoink) Yoink(p []byte) (n normie, err tea) {
    # Check if we've reached the end
    if by.pos >= len(by.data) {
        damn 0, ErrYoinkBruh
    }
    
    # Calculate how much we can read
    sus available normie = len(by.data) - by.pos
    sus to_read normie = len(p)
    
    if available < to_read {
        to_read = available
    }
    
    # Copy data
    bestie i := 0; i < to_read; i++ {
        p[i] = by.data[by.pos + i]
    }
    
    by.pos = by.pos + to_read
    
    # Return EOF if we've reached the end
    if by.pos >= len(by.data) {
        damn to_read, ErrYoinkBruh
    }
    
    damn to_read, ""
}

# LimitedYoink - A reader that stops after n bytes
struct LimitedYoink {
    reader Yoink
    limit thicc
    remaining thicc
}

# Create a LimitedYoink
slay LimitedYoink(r Yoink, n thicc) LimitedYoink {
    damn LimitedYoink{reader: r, limit: n, remaining: n}
}

# Implement Yoink interface for LimitedYoink
slay (lr *LimitedYoink) Yoink(p []byte) (n normie, err tea) {
    if lr.remaining <= 0 {
        damn 0, ErrYoinkBruh
    }
    
    # Limit the read size
    sus max_read normie = len(p)
    if thicc(max_read) > lr.remaining {
        max_read = normie(lr.remaining)
    }
    
    # Create a smaller buffer if needed
    sus limited_buf []byte = make([]byte, max_read)
    sus read_count normie, read_err tea = lr.reader.Yoink(limited_buf)
    
    # Copy the data
    bestie i := 0; i < read_count; i++ {
        p[i] = limited_buf[i]
    }
    
    lr.remaining = lr.remaining - thicc(read_count)
    
    if lr.remaining <= 0 {
        damn read_count, ErrYoinkBruh
    }
    
    damn read_count, read_err
}

# YeetAll - Copies all data from a Yoink to a Yeeter (like io.Copy)
slay YeetAll(dst Yeeter, src Yoink) (written thicc, err tea) {
    sus buf []byte = make([]byte, 1024)  # 1KB buffer
    sus total_written thicc = 0
    
    bestie {
        sus n normie, read_err tea = src.Yoink(buf)
        
        if n > 0 {
            sus written_bytes normie, write_err tea = dst.Yeet(buf[:n])
            total_written = total_written + thicc(written_bytes)
            
            if write_err != "" {
                damn total_written, write_err
            }
        }
        
        if read_err == ErrYoinkBruh {
            ghosted
        }
        
        if read_err != "" {
            damn total_written, read_err
        }
    }
    
    damn total_written, ""
}

# Utility functions for common operations

# YeetString - Write a string to a Yeeter
slay YeetString(dst Yeeter, content tea) (n normie, err tea) {
    sus data []byte = []byte(content)
    damn dst.Yeet(data)
}

# YoinkAll - Read all data from a Yoink until EOF
slay YoinkAll(src Yoink) (content tea, err tea) {
    sus result tea = ""
    sus buf []byte = make([]byte, 1024)
    
    bestie {
        sus n normie, read_err tea = src.Yoink(buf)
        
        if n > 0 {
            result = result + string(buf[:n])
        }
        
        if read_err == ErrYoinkBruh {
            ghosted
        }
        
        if read_err != "" {
            damn result, read_err
        }
    }
    
    damn result, ""
}

# YeetLine - Write a line with newline
slay YeetLine(dst Yeeter, content tea) (n normie, err tea) {
    damn YeetString(dst, content + "\n")
}

# IsEOF - Check if error is EOF
slay IsEOF(err tea) lit {
    damn err == ErrYoinkBruh
}

# MultiYeeter - Write to multiple Yeeters at once
struct MultiYeeter {
    yeeters []Yeeter
}

# Create a new MultiYeeter
slay new_multi_yeeter(yeeters ...Yeeter) MultiYeeter {
    damn MultiYeeter{yeeters: yeeters}
}

# Implement Yeeter interface for MultiYeeter
slay (my *MultiYeeter) Yeet(p []byte) (n normie, err tea) {
    bestie _, yeeter := range my.yeeters {
        sus written normie, write_err tea = yeeter.Yeet(p)
        
        if write_err != "" {
            damn written, write_err
        }
    }
    
    damn len(p), ""
}

# BufferedYoink - Buffer reads for better performance
struct BufferedYoink {
    reader Yoink
    buffer []byte
    pos normie
    size normie
}

# Create a new BufferedYoink
slay new_buffered_yoink(reader Yoink, buffer_size normie) BufferedYoink {
    damn BufferedYoink{
        reader: reader,
        buffer: make([]byte, buffer_size),
        pos: 0,
        size: 0
    }
}

# Implement Yoink interface for BufferedYoink
slay (br *BufferedYoink) Yoink(p []byte) (n normie, err tea) {
    sus read_count normie = 0
    sus max_read normie = len(p)
    
    bestie read_count < max_read {
        # If buffer is empty, refill it
        if br.pos >= br.size {
            sus new_size normie, read_err tea = br.reader.Yoink(br.buffer)
            br.size = new_size
            br.pos = 0
            
            if read_err != "" && read_err != ErrYoinkBruh {
                damn read_count, read_err
            }
            
            if br.size == 0 {
                if read_count == 0 {
                    damn 0, ErrYoinkBruh
                }
                damn read_count, ""
            }
        }
        
        # Copy from buffer
        sus available normie = br.size - br.pos
        sus to_copy normie = max_read - read_count
        
        if available < to_copy {
            to_copy = available
        }
        
        bestie i := 0; i < to_copy; i++ {
            p[read_count + i] = br.buffer[br.pos + i]
        }
        
        br.pos = br.pos + to_copy
        read_count = read_count + to_copy
    }
    
    damn read_count, ""
}
