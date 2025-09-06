#!/usr/bin/env python3

"""
CURSED LeetCode Problem Generator
Generates LeetCode problems implemented in CURSED programming language
Using ඞ pointers and 💀 file extensions!
"""

import os
import json

# LeetCode problems database
PROBLEMS = [
    # Arrays & Hashing
    {"id": 1, "name": "Two Sum", "category": "arrays", "difficulty": "Easy"},
    {"id": 11, "name": "Container With Most Water", "category": "arrays", "difficulty": "Medium"},
    {"id": 15, "name": "3Sum", "category": "arrays", "difficulty": "Medium"},
    {"id": 16, "name": "3Sum Closest", "category": "arrays", "difficulty": "Medium"},
    {"id": 18, "name": "4Sum", "category": "arrays", "difficulty": "Medium"},
    {"id": 26, "name": "Remove Duplicates from Sorted Array", "category": "arrays", "difficulty": "Easy"},
    {"id": 27, "name": "Remove Element", "category": "arrays", "difficulty": "Easy"},
    {"id": 31, "name": "Next Permutation", "category": "arrays", "difficulty": "Medium"},
    {"id": 33, "name": "Search in Rotated Sorted Array", "category": "binary_search", "difficulty": "Medium"},
    {"id": 34, "name": "Find First and Last Position", "category": "binary_search", "difficulty": "Medium"},
    
    # Strings
    {"id": 3, "name": "Longest Substring Without Repeating Characters", "category": "strings", "difficulty": "Medium"},
    {"id": 5, "name": "Longest Palindromic Substring", "category": "strings", "difficulty": "Medium"},
    {"id": 6, "name": "Zigzag Conversion", "category": "strings", "difficulty": "Medium"},
    {"id": 8, "name": "String to Integer (atoi)", "category": "strings", "difficulty": "Medium"},
    {"id": 14, "name": "Longest Common Prefix", "category": "strings", "difficulty": "Easy"},
    {"id": 17, "name": "Letter Combinations of Phone Number", "category": "backtracking", "difficulty": "Medium"},
    {"id": 20, "name": "Valid Parentheses", "category": "strings", "difficulty": "Easy"},
    {"id": 22, "name": "Generate Parentheses", "category": "backtracking", "difficulty": "Medium"},
    {"id": 28, "name": "Find Index of First Occurrence", "category": "strings", "difficulty": "Easy"},
    
    # Linked Lists  
    {"id": 2, "name": "Add Two Numbers", "category": "linked_lists", "difficulty": "Medium"},
    {"id": 19, "name": "Remove Nth Node From End", "category": "linked_lists", "difficulty": "Medium"},
    {"id": 21, "name": "Merge Two Sorted Lists", "category": "linked_lists", "difficulty": "Easy"},
    {"id": 23, "name": "Merge k Sorted Lists", "category": "linked_lists", "difficulty": "Hard"},
    {"id": 24, "name": "Swap Nodes in Pairs", "category": "linked_lists", "difficulty": "Medium"},
    {"id": 25, "name": "Reverse Nodes in k-Group", "category": "linked_lists", "difficulty": "Hard"},
    
    # Trees
    {"id": 94, "name": "Binary Tree Inorder Traversal", "category": "trees", "difficulty": "Easy"},
    {"id": 95, "name": "Unique Binary Search Trees II", "category": "trees", "difficulty": "Medium"},
    {"id": 96, "name": "Unique Binary Search Trees", "category": "trees", "difficulty": "Medium"},
    {"id": 98, "name": "Validate Binary Search Tree", "category": "trees", "difficulty": "Medium"},
    {"id": 100, "name": "Same Tree", "category": "trees", "difficulty": "Easy"},
    {"id": 101, "name": "Symmetric Tree", "category": "trees", "difficulty": "Easy"},
    {"id": 102, "name": "Binary Tree Level Order Traversal", "category": "trees", "difficulty": "Medium"},
    {"id": 104, "name": "Maximum Depth of Binary Tree", "category": "trees", "difficulty": "Easy"},
    {"id": 105, "name": "Construct Binary Tree from Preorder and Inorder", "category": "trees", "difficulty": "Medium"},
    {"id": 106, "name": "Construct Binary Tree from Inorder and Postorder", "category": "trees", "difficulty": "Medium"},
    {"id": 107, "name": "Binary Tree Level Order Traversal II", "category": "trees", "difficulty": "Medium"},
    {"id": 108, "name": "Convert Sorted Array to Binary Search Tree", "category": "trees", "difficulty": "Easy"},
    {"id": 110, "name": "Balanced Binary Tree", "category": "trees", "difficulty": "Easy"},
    {"id": 111, "name": "Minimum Depth of Binary Tree", "category": "trees", "difficulty": "Easy"},
    {"id": 112, "name": "Path Sum", "category": "trees", "difficulty": "Easy"},
    {"id": 113, "name": "Path Sum II", "category": "trees", "difficulty": "Medium"},
    {"id": 114, "name": "Flatten Binary Tree to Linked List", "category": "trees", "difficulty": "Medium"},
    {"id": 116, "name": "Populating Next Right Pointers", "category": "trees", "difficulty": "Medium"},
    {"id": 124, "name": "Binary Tree Maximum Path Sum", "category": "trees", "difficulty": "Hard"},
    {"id": 144, "name": "Binary Tree Preorder Traversal", "category": "trees", "difficulty": "Easy"},
    {"id": 145, "name": "Binary Tree Postorder Traversal", "category": "trees", "difficulty": "Easy"},
    
    # Dynamic Programming
    {"id": 53, "name": "Maximum Subarray", "category": "dynamic_programming", "difficulty": "Medium"},
    {"id": 62, "name": "Unique Paths", "category": "dynamic_programming", "difficulty": "Medium"},
    {"id": 63, "name": "Unique Paths II", "category": "dynamic_programming", "difficulty": "Medium"},
    {"id": 64, "name": "Minimum Path Sum", "category": "dynamic_programming", "difficulty": "Medium"},
    {"id": 70, "name": "Climbing Stairs", "category": "dynamic_programming", "difficulty": "Easy"},
    {"id": 72, "name": "Edit Distance", "category": "dynamic_programming", "difficulty": "Hard"},
    {"id": 91, "name": "Decode Ways", "category": "dynamic_programming", "difficulty": "Medium"},
    {"id": 118, "name": "Pascal's Triangle", "category": "dynamic_programming", "difficulty": "Easy"},
    {"id": 119, "name": "Pascal's Triangle II", "category": "dynamic_programming", "difficulty": "Easy"},
    {"id": 120, "name": "Triangle", "category": "dynamic_programming", "difficulty": "Medium"},
    {"id": 121, "name": "Best Time to Buy and Sell Stock", "category": "dynamic_programming", "difficulty": "Easy"},
    {"id": 122, "name": "Best Time to Buy and Sell Stock II", "category": "dynamic_programming", "difficulty": "Medium"},
    {"id": 123, "name": "Best Time to Buy and Sell Stock III", "category": "dynamic_programming", "difficulty": "Hard"},
    
    # Math
    {"id": 7, "name": "Reverse Integer", "category": "math", "difficulty": "Medium"},
    {"id": 9, "name": "Palindrome Number", "category": "math", "difficulty": "Easy"},
    {"id": 12, "name": "Integer to Roman", "category": "math", "difficulty": "Medium"},
    {"id": 13, "name": "Roman to Integer", "category": "math", "difficulty": "Easy"},
    {"id": 29, "name": "Divide Two Integers", "category": "math", "difficulty": "Medium"},
    {"id": 43, "name": "Multiply Strings", "category": "math", "difficulty": "Medium"},
    {"id": 50, "name": "Pow(x, n)", "category": "math", "difficulty": "Medium"},
    {"id": 66, "name": "Plus One", "category": "math", "difficulty": "Easy"},
    {"id": 67, "name": "Add Binary", "category": "math", "difficulty": "Easy"},
    {"id": 69, "name": "Sqrt(x)", "category": "math", "difficulty": "Easy"},
]

def generate_problem_stub(problem):
    """Generate a CURSED implementation stub for a LeetCode problem"""
    
    problem_id = problem["id"]
    name = problem["name"]
    category = problem["category"] 
    difficulty = problem["difficulty"]
    
    filename = f"{problem_id:03d}_{name.lower().replace(' ', '_').replace('(', '').replace(')', '').replace(',', '').replace("'", '')}.💀"
    
    content = f"""vibe main

yeet "vibez"
yeet "mathz"

fr fr LeetCode #{problem_id}: {name}
fr fr Difficulty: {difficulty}
fr fr Category: {category.title()}
fr fr TODO: Implement solution using CURSED ඞ pointers!

slay solve_{name.lower().replace(' ', '_').replace('(', '').replace(')', '').replace(',', '').replace("'", '')}() {{
    fr fr TODO: Implement the actual algorithm here
    vibez.spill("=== LeetCode #{problem_id}: {name} ===")
    vibez.spill("TODO: Implementation needed")
    vibez.spill("=== Test Complete ===")
}}

slay test_{name.lower().replace(' ', '_').replace('(', '').replace(')', '').replace(',', '').replace("'", '')}() {{
    solve_{name.lower().replace(' ', '_').replace('(', '').replace(')', '').replace(',', '').replace("'", '')}()
}}

slay main_character() {{
    test_{name.lower().replace(' ', '_').replace('(', '').replace(')', '').replace(',', '').replace("'", '')}()
}}
"""
    
    return filename, content

def main():
    base_dir = "test_programs/leetcode"
    
    print("🔥 Generating CURSED LeetCode Problems with ඞ pointers and 💀 extensions!")
    print("=" * 70)
    
    generated_count = 0
    
    for problem in PROBLEMS:
        category_dir = os.path.join(base_dir, problem["category"])
        os.makedirs(category_dir, exist_ok=True)
        
        filename, content = generate_problem_stub(problem)
        filepath = os.path.join(category_dir, filename)
        
        # Only generate if file doesn't exist
        if not os.path.exists(filepath):
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Generated: {filename} ({problem['difficulty']})")
            generated_count += 1
        else:
            print(f"⏭️  Skipped: {filename} (already exists)")
    
    print("=" * 70)
    print(f"🎉 Generated {generated_count} new LeetCode problems in CURSED!")
    print("💀 All files use proper .💀 skull emoji extension")
    print("ඞ All problems ready for Among Us pointer implementations!")

if __name__ == "__main__":
    main()
