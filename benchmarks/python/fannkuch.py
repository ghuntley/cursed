#!/usr/bin/env python3
# Fannkuch redux benchmark adapted from The Computer Language Benchmarks Game

import time
import resource
import sys
from multiprocessing import Pool

def count_flips(perm):
    first = perm[0]
    if first == 0:
        return 0
    flips = 0
    new_perm = perm.copy()
    
    while True:
        flips += 1
        new_first = new_perm[first]
        
        # Reverse the first (first+1) elements of the permutation
        for i in range((first + 1) >> 1):
            j = first - i
            new_perm[i], new_perm[j] = new_perm[j], new_perm[i]
        
        new_perm[first] = first
        first = new_first
        
        if first == 0:
            return flips

def fannkuch_task(n, start_idx, count):
    p = list(range(n))
    perm = p.copy()
    count_array = [0] * n
    max_flips = 0
    checksum = 0
    
    # Initialize permutations to the specified starting index
    idx = start_idx
    for i in range(n-1, 0, -1):
        d = idx // i
        count_array[i] = d
        idx %= i
        
        # Rotate p[0:i+1] left by d+1 steps
        p[:i+1] = p[d:i+1] + p[:d]
        
        perm = p.copy()
    
    # Now iterate through the permutations
    for _ in range(count):
        # Count flips for this permutation
        flips = count_flips(perm.copy())
        max_flips = max(max_flips, flips)
        
        # Update checksum (alternating signs)
        if start_idx % 2 == 0:
            checksum += flips
        else:
            checksum -= flips
        
        # Generate next permutation
        if perm[0] != 0:
            # Rotate first element to the end
            first = perm[0]
            for i in range(1, first+1):
                perm[i-1] = perm[i]
            perm[first] = first
        else:
            # First element is 0, need to increment indices
            i = 1
            while count_array[i] >= i:
                count_array[i] = 0
                i += 1
            
            count_array[i] += 1
            
            # Rotate according to count_array
            first = perm[1]
            perm[1] = perm[0]
            
            for j in range(2, i+1):
                perm[j-1] = perm[j]
            perm[i] = first
            
        start_idx += 1
    
    return max_flips, checksum, start_idx

def main():
    start_time = time.time()
    
    n = 12  # Default parameter
    
    factorial = 1
    for i in range(2, n+1):
        factorial *= i
    
    # Number of tasks to split the work into
    num_tasks = min(factorial, 16)
    chunk_size = factorial // num_tasks
    remainder = factorial % num_tasks
    
    tasks = []
    start_idx = 0
    
    for i in range(num_tasks):
        this_chunk = chunk_size + (1 if i < remainder else 0)
        tasks.append((n, start_idx, this_chunk))
        start_idx += this_chunk
    
    # Process the tasks in parallel
    with Pool() as pool:
        results = pool.starmap(fannkuch_task, tasks)
    
    # Combine results
    max_flips = 0
    checksum = 0
    
    for flips, chk, _ in results:
        max_flips = max(max_flips, flips)
        checksum += chk
    
    print(f"Checksum: {checksum}\nPfannkuchen({n}): {max_flips}")
    
    elapsed = (time.time() - start_time) * 1000
    print(f"Time taken: {elapsed:.2f} ms")
    
    # Get memory usage
    mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    print(f"Memory used: {mem_usage / 1024:.2f} MB")

if __name__ == "__main__":
    main()