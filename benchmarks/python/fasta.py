#!/usr/bin/env python3
# FASTA benchmark adapted from The Computer Language Benchmarks Game

import time
import resource
import sys
import io
from array import array
from itertools import repeat
import random

def gen_random(limit, ia=3877, ic=29573, im=139968):
    random_array = array('f', [0]) * limit
    last = 42
    for i in range(limit):
        last = (last * ia + ic) % im
        random_array[i] = last / im
    return random_array

def make_cumulative(table):
    total = 0.0
    for i in range(len(table)):
        total += table[i][1]
        table[i] = (table[i][0], total)
    return table

def select_random_k(table, random_val):
    lo, hi = 0, len(table) - 1
    while hi > lo + 1:
        mid = (hi + lo) >> 1
        if random_val < table[mid][1]:
            hi = mid
        else:
            lo = mid
    return table[hi if random_val >= table[lo][1] else lo][0]

def write_fasta_sequence(out, header, sequence, num_chars):
    out.write(f'>{header}\n')
    total_len = len(sequence)
    
    for i in range(0, num_chars, 60):
        out.write(sequence[i % total_len:min(i % total_len + 60, total_len)])
        if i % total_len + 60 > total_len:
            # Complete the line with the beginning of the sequence
            remaining = 60 - (total_len - i % total_len)
            if remaining > 0:
                out.write(sequence[:remaining])
        out.write('\n')

def generate_random_fasta_sequence(out, header, table, random_vals, num_chars):
    out.write(f'>{header}\n')
    buf = bytearray()
    
    for i in range(0, num_chars, 60):
        line_length = min(60, num_chars - i)
        for j in range(line_length):
            index = (i + j) % len(random_vals)
            random_val = random_vals[index]
            c = select_random_k(table, random_val)
            buf.append(c)
        
        buf.append(ord('\n'))
        
        if len(buf) > 1024:
            out.write(buf.decode('ascii'))
            buf = bytearray()
    
    if buf:
        out.write(buf.decode('ascii'))

def main():
    start_time = time.time()
    
    # Parameters
    n = 25000000  # Default sequence length
    
    # Define the sequences
    alu = (
        "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGG"
        "GAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGA"
        "CCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAAT"
        "ACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCA"
        "GCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGG"
        "AGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCC"
        "AGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA"
    )
    
    iub = make_cumulative([
        (ord('a'), 0.27), (ord('c'), 0.12), (ord('g'), 0.12),
        (ord('t'), 0.27), (ord('B'), 0.02), (ord('D'), 0.02),
        (ord('H'), 0.02), (ord('K'), 0.02), (ord('M'), 0.02),
        (ord('N'), 0.02), (ord('R'), 0.02), (ord('S'), 0.02),
        (ord('V'), 0.02), (ord('W'), 0.02), (ord('Y'), 0.02)
    ])
    
    homosapiens = make_cumulative([
        (ord('a'), 0.3029549426680), (ord('c'), 0.1979883004921),
        (ord('g'), 0.1975473066391), (ord('t'), 0.3015094502008)
    ])
    
    # Generate random values
    random_vals = gen_random(16384)
    
    # Create an in-memory output buffer (we don't actually write to stdout for benchmarking)
    out = io.StringIO()
    
    # Generate sequences
    write_fasta_sequence(out, "ONE Homo sapiens alu", alu, n*2)
    generate_random_fasta_sequence(out, "TWO IUB ambiguity codes", iub, random_vals, n*3)
    generate_random_fasta_sequence(out, "THREE Homo sapiens frequency", homosapiens, random_vals, n*5)
    
    # Get result length for verification
    result_length = out.tell()
    print(f"Generated DNA sequence of length {result_length}")
    
    elapsed = (time.time() - start_time) * 1000
    print(f"Time taken: {elapsed:.2f} ms")
    
    # Get memory usage
    mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    print(f"Memory used: {mem_usage / 1024:.2f} MB")

if __name__ == "__main__":
    main()