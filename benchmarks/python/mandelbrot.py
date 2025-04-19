#!/usr/bin/env python3
# Mandelbrot benchmark adapted from The Computer Language Benchmarks Game

import time
import resource
import sys
import multiprocessing as mp

def calculate_row(y, w, h, max_iter):
    result = bytearray(w // 8 + (1 if w % 8 else 0))
    bit_num = 0
    byte_pos = 0
    byte_acc = 0
    
    for x in range(w):
        Zr, Zi, Tr, Ti = 0.0, 0.0, 0.0, 0.0
        Cr = 2.0 * x / w - 1.5
        Ci = 2.0 * y / h - 1.0
        
        i = 0
        while i < max_iter and Tr + Ti <= 4.0:
            Zi = 2.0 * Zr * Zi + Ci
            Zr = Tr - Ti + Cr
            Tr = Zr * Zr
            Ti = Zi * Zi
            i += 1
        
        byte_acc = (byte_acc << 1) | (i == max_iter)
        bit_num += 1
        
        if bit_num == 8:
            result[byte_pos] = byte_acc
            byte_acc = 0
            bit_num = 0
            byte_pos += 1
    
    # Handle remaining bits if any
    if bit_num > 0:
        byte_acc <<= (8 - bit_num)
        result[byte_pos] = byte_acc
    
    return result

def main():
    start_time = time.time()
    
    # Set parameters
    size = 4000  # default size
    max_iter = 50
    
    # Calculate the image
    with mp.Pool() as pool:
        rows = pool.starmap(calculate_row, 
                           [(y, size, size, max_iter) for y in range(size)])
    
    # Output PBM format header (commented out for benchmark)
    # print(f"P4\n{size} {size}")
    
    # Write the result (commented out for benchmark)
    # sys.stdout.buffer.write(b''.join(rows))
    
    elapsed = (time.time() - start_time) * 1000
    print(f"Time taken: {elapsed:.2f} ms")
    
    # Get memory usage
    mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    print(f"Memory used: {mem_usage / 1024:.2f} MB")

if __name__ == "__main__":
    main()