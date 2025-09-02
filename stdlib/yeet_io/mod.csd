fr fr YeetIO (io package)
fr fr Pure CURSED implementation of I/O operations with Gen Z twist

fr fr Error constants
sus ErrYoinkBruh tea = "no more to yoink, bruh"

fr fr Yeeter interface (equivalent to io.Writer)
fr fr Yeets (writes) data to a destination
collab Yeeter {
    Yeet(p byte[value]) (n normie, err tea)
}

fr fr Yoink interface (equivalent to io.Reader)  
fr fr Yoinks (reads) data from a source
collab Yoink {
    Yoink(p byte[value]) (n normie, err tea)
}

fr fr YoinkYeeter interface (equivalent to io.ReadWriter)
fr fr Combines Yoink and Yeeter interfaces
collab YoinkYeeter {
    Yoink(p byte[value]) (n normie, err tea)
    Yeet(p byte[value]) (n normie, err tea)
}

fr fr StringYeeter - A simple string-based writer implementation
struct StringYeeter {
    data tea
}

fr fr Create a new StringYeeter
slay new_string_yeeter() StringYeeter {
    damn StringYeeter{data: ""}
}

fr fr Implement Yeeter interface for StringYeeter
slay (sy *StringYeeter) Yeet(p byte[value]) (n normie, err tea) { fr fr Convert bytes to string and append
    sus str_data tea = string(p)
    sy.data = sy.data + str_data
    damn len(p), ""
}

fr fr Get the accumulated data from StringYeeter
slay (sy *StringYeeter) get_data() tea {
    damn sy.data
}

fr fr ByteYoink - A simple byte slice reader implementation
struct ByteYoink {
    data byte[value]
    pos normie
}

fr fr Create a new ByteYoink from string
slay new_byte_yoink(content tea) ByteYoink {
    damn ByteYoink{data: byte[value](content), pos: 0}
}

fr fr Implement Yoink interface for ByteYoink
slay (by *ByteYoink) Yoink(p byte[value]) (n normie, err tea) { fr fr Check if we've reached the end
    if by.pos >= len(by.data) {
        damn 0, ErrYoinkBruh
    } fr fr Calculate how much we can read
    sus available normie = len(by.data) - by.pos
    sus to_read normie = len(p)
    
    if available < to_read {
        to_read = available
    } fr fr Copy data
    bestie i := 0; i < to_read; i++ {
        p[i] = by.data[by.pos + i]
    }
    
    by.pos = by.pos + to_read fr fr Return EOF if we've reached the end
    if by.pos >= len(by.data) {
        damn to_read, ErrYoinkBruh
    }
    
    damn to_read, ""
}

fr fr LimitedYoink - A reader that stops after n bytes
struct LimitedYoink {
    reader Yoink
    limit thicc
    remaining thicc
}

fr fr Create a LimitedYoink
slay LimitedYoink(r Yoink, n thicc) LimitedYoink {
    damn LimitedYoink{reader: r, limit: n, remaining: n}
}

fr fr Implement Yoink interface for LimitedYoink
slay (lr *LimitedYoink) Yoink(p byte[value]) (n normie, err tea) {
    if lr.remaining <= 0 {
        damn 0, ErrYoinkBruh
    } fr fr Limit the read size
    sus max_read normie = len(p)
    if thicc(max_read) > lr.remaining {
        max_read = normie(lr.remaining)
    } fr fr Create a smaller buffer if needed
    sus limited_buf byte[value] = make(byte[value], max_read)
    sus read_count normie, read_err tea = lr.reader.Yoink(limited_buf) fr fr Copy the data
    bestie i := 0; i < read_count; i++ {
        p[i] = limited_buf[i]
    }
    
    lr.remaining = lr.remaining - thicc(read_count)
    
    if lr.remaining <= 0 {
        damn read_count, ErrYoinkBruh
    }
    
    damn read_count, read_err
}

fr fr YeetAll - Copies all data from a Yoink to a Yeeter (like io.Copy)
slay YeetAll(dst Yeeter, src Yoink) (written thicc, err tea) {
    sus buf byte[value] = make(byte[value], 1024) fr fr 1KB buffer
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

fr fr Utility functions for common operations

fr fr YeetString - Write a string to a Yeeter
slay YeetString(dst Yeeter, content tea) (n normie, err tea) {
    sus data byte[value] = byte[value](content)
    damn dst.Yeet(data)
}

fr fr YoinkAll - Read all data from a Yoink until EOF
slay YoinkAll(src Yoink) (content tea, err tea) {
    sus result tea = ""
    sus buf byte[value] = make(byte[value], 1024)
    
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

fr fr YeetLine - Write a line with newline
slay YeetLine(dst Yeeter, content tea) (n normie, err tea) {
    damn YeetString(dst, content + "\n")
}

fr fr IsEOF - Check if error is EOF
slay IsEOF(err tea) lit {
    damn err == ErrYoinkBruh
}

fr fr MultiYeeter - Write to multiple Yeeters at once
struct MultiYeeter {
    yeeters Yeeter[value]
}

fr fr Create a new MultiYeeter
slay new_multi_yeeter(yeeters ...Yeeter) MultiYeeter {
    damn MultiYeeter{yeeters: yeeters}
}

fr fr Implement Yeeter interface for MultiYeeter
slay (my *MultiYeeter) Yeet(p byte[value]) (n normie, err tea) {
    bestie _, yeeter := range my.yeeters {
        sus written normie, write_err tea = yeeter.Yeet(p)
        
        if write_err != "" {
            damn written, write_err
        }
    }
    
    damn len(p), ""
}

fr fr BufferedYoink - Buffer reads for better performance
struct BufferedYoink {
    reader Yoink
    buffer byte[value]
    pos normie
    size normie
}

fr fr Create a new BufferedYoink
slay new_buffered_yoink(reader Yoink, buffer_size normie) BufferedYoink {
    damn BufferedYoink{
        reader: reader,
        buffer: make(byte[value], buffer_size),
        pos: 0,
        size: 0
    }
}

fr fr Implement Yoink interface for BufferedYoink
slay (br *BufferedYoink) Yoink(p byte[value]) (n normie, err tea) {
    sus read_count normie = 0
    sus max_read normie = len(p)
    
    bestie read_count < max_read { fr fr If buffer is empty, refill it
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
        } fr fr Copy from buffer
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
