#!/usr/bin/env python3
# Binary trees benchmark adapted from The Computer Language Benchmarks Game

import sys
import time
import gc
import resource

class TreeNode:
    def __init__(self, item, depth):
        self.item = item
        if depth > 0:
            depth -= 1
            self.left = TreeNode(item * 2 - 1, depth)
            self.right = TreeNode(item * 2, depth)
        else:
            self.left = None
            self.right = None
    
    def check(self):
        if self.left is None:
            return self.item
        return self.item + self.left.check() - self.right.check()

def main():
    min_depth = 4
    max_depth = 12
    stretch_depth = max_depth + 1
    
    start_time = time.time()
    
    # Stretch memory tree
    stretch_tree = TreeNode(0, stretch_depth)
    print(f"stretch tree of depth {stretch_depth} check: {stretch_tree.check()}")
    
    # Allow garbage collection
    stretch_tree = None
    gc.collect()
    
    # Create long-lived tree
    long_lived_tree = TreeNode(0, max_depth)
    
    # Create, check, and destroy multiple trees
    for depth in range(min_depth, max_depth + 1, 2):
        iterations = 1 << (max_depth - depth + min_depth)
        result = 0
        
        for i in range(iterations):
            a = TreeNode(i, depth)
            b = TreeNode(-i, depth)
            result += a.check() + b.check()
            
        print(f"{iterations*2} trees of depth {depth} check: {result}")
    
    # Check the long-lived tree
    print(f"long lived tree of depth {max_depth} check: {long_lived_tree.check()}")
    
    elapsed = (time.time() - start_time) * 1000
    print(f"Time taken: {elapsed:.2f} ms")
    
    # Get memory usage
    mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    print(f"Memory used: {mem_usage / 1024:.2f} MB")

if __name__ == "__main__":
    main()