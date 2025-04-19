#!/usr/bin/env python3
# String processing benchmark for the Cursed programming language comparison

import time
import resource
import re
import random
import string

def generate_random_string(length):
    """Generate a random string of given length"""
    return ''.join(random.choice(string.ascii_letters + string.digits + string.punctuation) for _ in range(length))

def string_concatenation(iterations, base_str):
    """Benchmark string concatenation"""
    result = ""
    for i in range(iterations):
        result += base_str + str(i)
    return result

def string_interpolation(iterations, base_str):
    """Benchmark string interpolation"""
    result = ""
    for i in range(iterations):
        result += f"{base_str}{i}"
    return result

def string_splitting(iterations, text, delimiter):
    """Benchmark string splitting"""
    result = []
    for _ in range(iterations):
        result = text.split(delimiter)
    return result

def string_joining(iterations, parts, delimiter):
    """Benchmark string joining"""
    result = ""
    for _ in range(iterations):
        result = delimiter.join(parts)
    return result

def regex_matching(iterations, text, pattern):
    """Benchmark regex matching"""
    compiled_regex = re.compile(pattern)
    matches = 0
    for _ in range(iterations):
        if compiled_regex.search(text):
            matches += 1
    return matches

def regex_replace(iterations, text, pattern, replacement):
    """Benchmark regex replacement"""
    compiled_regex = re.compile(pattern)
    result = text
    for _ in range(iterations):
        result = compiled_regex.sub(replacement, result)
    return result

def main():
    start_time = time.time()
    
    # Parameters
    iterations = 10000
    
    # Generate test data
    base_str = "Python string benchmark test"
    long_text = generate_random_string(10000)
    words = [generate_random_string(random.randint(3, 15)) for _ in range(1000)]
    
    # Benchmarks
    print("Running string processing benchmarks...")
    
    # 1. String concatenation
    concat_start = time.time()
    concat_result = string_concatenation(iterations, base_str)
    concat_time = time.time() - concat_start
    print(f"Concatenation: {concat_time:.6f} seconds, result length: {len(concat_result)}")
    
    # 2. String interpolation
    interp_start = time.time()
    interp_result = string_interpolation(iterations, base_str)
    interp_time = time.time() - interp_start
    print(f"Interpolation: {interp_time:.6f} seconds, result length: {len(interp_result)}")
    
    # 3. String splitting
    split_start = time.time()
    split_result = string_splitting(iterations // 100, long_text, " ")
    split_time = time.time() - split_start
    print(f"Splitting: {split_time:.6f} seconds, parts count: {len(split_result)}")
    
    # 4. String joining
    join_start = time.time()
    join_result = string_joining(iterations // 10, words, " ")
    join_time = time.time() - join_start
    print(f"Joining: {join_time:.6f} seconds, result length: {len(join_result)}")
    
    # 5. Regex matching
    regex_match_start = time.time()
    match_count = regex_matching(iterations, long_text, r"[a-z]{5,10}")
    regex_match_time = time.time() - regex_match_start
    print(f"Regex matching: {regex_match_time:.6f} seconds, matches: {match_count}")
    
    # 6. Regex replacement
    regex_replace_start = time.time()
    replace_result = regex_replace(iterations // 100, long_text, r"[aeiou]", "X")
    regex_replace_time = time.time() - regex_replace_start
    print(f"Regex replacement: {regex_replace_time:.6f} seconds, result length: {len(replace_result)}")
    
    # Total time
    total_time = (time.time() - start_time) * 1000
    print(f"\nTotal time taken: {total_time:.2f} ms")
    
    # Get memory usage
    mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    print(f"Memory used: {mem_usage / 1024:.2f} MB")

if __name__ == "__main__":
    main()