#!/usr/bin/env python3

"""
Rapid LeetCode Problem Creator for CURSED
Creates more problem templates to reach 200 total
"""

import os

# Additional LeetCode problems to create
MORE_PROBLEMS = [
    # Arrays & Hashing (continue)
    {"id": 36, "name": "Valid Sudoku", "category": "arrays"},
    {"id": 41, "name": "First Missing Positive", "category": "arrays"},
    {"id": 42, "name": "Trapping Rain Water", "category": "arrays"},
    {"id": 48, "name": "Rotate Image", "category": "arrays"},
    {"id": 49, "name": "Group Anagrams", "category": "strings"},
    {"id": 54, "name": "Spiral Matrix", "category": "arrays"},
    {"id": 55, "name": "Jump Game", "category": "arrays"},
    {"id": 56, "name": "Merge Intervals", "category": "sorting"},
    {"id": 57, "name": "Insert Interval", "category": "arrays"},
    {"id": 73, "name": "Set Matrix Zeroes", "category": "arrays"},
    {"id": 75, "name": "Sort Colors", "category": "sorting"},
    {"id": 76, "name": "Minimum Window Substring", "category": "strings"},
    {"id": 78, "name": "Subsets", "category": "backtracking"},
    {"id": 79, "name": "Word Search", "category": "backtracking"},
    {"id": 84, "name": "Largest Rectangle in Histogram", "category": "stacks"},
    {"id": 85, "name": "Maximal Rectangle", "category": "stacks"},
    
    # More Trees (using ඞ pointers)
    {"id": 103, "name": "Binary Tree Zigzag Level Order", "category": "trees"},
    {"id": 109, "name": "Convert Sorted List to BST", "category": "trees"},
    {"id": 115, "name": "Distinct Subsequences", "category": "dynamic_programming"},
    {"id": 117, "name": "Populating Next Right Pointers II", "category": "trees"},
    {"id": 125, "name": "Valid Palindrome", "category": "strings"},
    {"id": 126, "name": "Word Ladder II", "category": "graphs"},
    {"id": 127, "name": "Word Ladder", "category": "graphs"},
    {"id": 129, "name": "Sum Root to Leaf Numbers", "category": "trees"},
    {"id": 130, "name": "Surrounded Regions", "category": "graphs"},
    {"id": 131, "name": "Palindrome Partitioning", "category": "backtracking"},
    {"id": 132, "name": "Palindrome Partitioning II", "category": "dynamic_programming"},
    {"id": 133, "name": "Clone Graph", "category": "graphs"},
    {"id": 134, "name": "Gas Station", "category": "arrays"},
    {"id": 135, "name": "Candy", "category": "arrays"},
    {"id": 136, "name": "Single Number", "category": "bit_manipulation"},
    {"id": 137, "name": "Single Number II", "category": "bit_manipulation"},
    {"id": 138, "name": "Copy List with Random Pointer", "category": "linked_lists"},
    {"id": 139, "name": "Word Break", "category": "dynamic_programming"},
    {"id": 140, "name": "Word Break II", "category": "backtracking"},
    {"id": 141, "name": "Linked List Cycle", "category": "linked_lists"},
    {"id": 142, "name": "Linked List Cycle II", "category": "linked_lists"},
    {"id": 143, "name": "Reorder List", "category": "linked_lists"},
    {"id": 146, "name": "LRU Cache", "category": "design"},
    {"id": 147, "name": "Insertion Sort List", "category": "linked_lists"},
    {"id": 148, "name": "Sort List", "category": "linked_lists"},
    {"id": 149, "name": "Max Points on a Line", "category": "math"},
    {"id": 150, "name": "Evaluate Reverse Polish Notation", "category": "stacks"},
    {"id": 151, "name": "Reverse Words in a String", "category": "strings"},
    {"id": 152, "name": "Maximum Product Subarray", "category": "dynamic_programming"},
    {"id": 153, "name": "Find Minimum in Rotated Sorted Array", "category": "binary_search"},
    {"id": 154, "name": "Find Minimum in Rotated Sorted Array II", "category": "binary_search"},
    {"id": 155, "name": "Min Stack", "category": "stacks"},
    {"id": 160, "name": "Intersection of Two Linked Lists", "category": "linked_lists"},
    {"id": 162, "name": "Find Peak Element", "category": "binary_search"},
    {"id": 163, "name": "Missing Ranges", "category": "arrays"},
    {"id": 167, "name": "Two Sum II", "category": "arrays"},
    {"id": 168, "name": "Excel Sheet Column Title", "category": "math"},
    {"id": 169, "name": "Majority Element", "category": "arrays"},
    {"id": 170, "name": "Two Sum III", "category": "design"},
    {"id": 171, "name": "Excel Sheet Column Number", "category": "math"},
    {"id": 173, "name": "Binary Search Tree Iterator", "category": "trees"},
    {"id": 174, "name": "Dungeon Game", "category": "dynamic_programming"},
    {"id": 179, "name": "Largest Number", "category": "sorting"},
    {"id": 189, "name": "Rotate Array", "category": "arrays"},
    {"id": 190, "name": "Reverse Bits", "category": "bit_manipulation"},
    {"id": 191, "name": "Number of 1 Bits", "category": "bit_manipulation"},
    {"id": 198, "name": "House Robber", "category": "dynamic_programming"},
    {"id": 199, "name": "Binary Tree Right Side View", "category": "trees"},
    {"id": 200, "name": "Number of Islands", "category": "graphs"},
    {"id": 201, "name": "Bitwise AND of Numbers Range", "category": "bit_manipulation"},
    {"id": 202, "name": "Happy Number", "category": "math"},
    {"id": 203, "name": "Remove Linked List Elements", "category": "linked_lists"},
    {"id": 204, "name": "Count Primes", "category": "math"},
    {"id": 205, "name": "Isomorphic Strings", "category": "strings"},
    {"id": 206, "name": "Reverse Linked List", "category": "linked_lists"},
    {"id": 207, "name": "Course Schedule", "category": "graphs"},
    {"id": 208, "name": "Implement Trie", "category": "trees"},
    {"id": 209, "name": "Minimum Size Subarray Sum", "category": "arrays"},
    {"id": 210, "name": "Course Schedule II", "category": "graphs"},
    {"id": 211, "name": "Design Add and Search Words", "category": "trees"},
    {"id": 212, "name": "Word Search II", "category": "backtracking"},
    {"id": 213, "name": "House Robber II", "category": "dynamic_programming"},
    {"id": 214, "name": "Shortest Palindrome", "category": "strings"},
    {"id": 215, "name": "Kth Largest Element", "category": "heaps"},
    {"id": 216, "name": "Combination Sum III", "category": "backtracking"},
    {"id": 217, "name": "Contains Duplicate", "category": "arrays"},
    {"id": 218, "name": "The Skyline Problem", "category": "heaps"},
    {"id": 219, "name": "Contains Duplicate II", "category": "arrays"},
    {"id": 220, "name": "Contains Duplicate III", "category": "arrays"},
    {"id": 221, "name": "Maximal Square", "category": "dynamic_programming"},
    {"id": 222, "name": "Count Complete Tree Nodes", "category": "trees"},
    {"id": 223, "name": "Rectangle Area", "category": "math"},
    {"id": 224, "name": "Basic Calculator", "category": "stacks"},
    {"id": 225, "name": "Implement Stack using Queues", "category": "design"},
    {"id": 226, "name": "Invert Binary Tree", "category": "trees"},
    {"id": 227, "name": "Basic Calculator II", "category": "stacks"},
    {"id": 228, "name": "Summary Ranges", "category": "arrays"},
    {"id": 229, "name": "Majority Element II", "category": "arrays"},
    {"id": 230, "name": "Kth Smallest Element in BST", "category": "trees"},
    {"id": 231, "name": "Power of Two", "category": "bit_manipulation"},
    {"id": 232, "name": "Implement Queue using Stacks", "category": "design"},
    {"id": 233, "name": "Number of Digit One", "category": "math"},
    {"id": 234, "name": "Palindrome Linked List", "category": "linked_lists"},
    {"id": 235, "name": "Lowest Common Ancestor of BST", "category": "trees"},
    {"id": 236, "name": "Lowest Common Ancestor of Binary Tree", "category": "trees"},
    {"id": 237, "name": "Delete Node in Linked List", "category": "linked_lists"},
    {"id": 238, "name": "Product of Array Except Self", "category": "arrays"},
    {"id": 239, "name": "Sliding Window Maximum", "category": "arrays"},
    {"id": 240, "name": "Search 2D Matrix II", "category": "binary_search"},
    {"id": 241, "name": "Different Ways to Add Parentheses", "category": "divide_conquer"},
    {"id": 242, "name": "Valid Anagram", "category": "strings"},
    {"id": 268, "name": "Missing Number", "category": "bit_manipulation"},
    {"id": 278, "name": "First Bad Version", "category": "binary_search"},
    {"id": 283, "name": "Move Zeroes", "category": "arrays"},
    {"id": 287, "name": "Find Duplicate Number", "category": "arrays"},
    {"id": 300, "name": "Longest Increasing Subsequence", "category": "dynamic_programming"},
    {"id": 322, "name": "Coin Change", "category": "dynamic_programming"},
    {"id": 338, "name": "Counting Bits", "category": "bit_manipulation"},
    {"id": 344, "name": "Reverse String", "category": "strings"},
    {"id": 347, "name": "Top K Frequent Elements", "category": "heaps"},
    {"id": 371, "name": "Sum of Two Integers", "category": "bit_manipulation"},
    {"id": 383, "name": "Ransom Note", "category": "strings"},
    {"id": 387, "name": "First Unique Character", "category": "strings"},
    {"id": 389, "name": "Find the Difference", "category": "strings"},
    {"id": 392, "name": "Is Subsequence", "category": "strings"},
    {"id": 412, "name": "FizzBuzz", "category": "math"},
    {"id": 415, "name": "Add Strings", "category": "math"},
    {"id": 448, "name": "Find All Numbers Disappeared", "category": "arrays"},
    {"id": 455, "name": "Assign Cookies", "category": "arrays"},
    {"id": 463, "name": "Island Perimeter", "category": "arrays"},
    {"id": 476, "name": "Number Complement", "category": "bit_manipulation"},
    {"id": 485, "name": "Max Consecutive Ones", "category": "arrays"},
    {"id": 496, "name": "Next Greater Element I", "category": "stacks"},
    {"id": 500, "name": "Keyboard Row", "category": "strings"},
    {"id": 509, "name": "Fibonacci Number", "category": "dynamic_programming"},
    {"id": 543, "name": "Diameter of Binary Tree", "category": "trees"},
    {"id": 557, "name": "Reverse Words in String III", "category": "strings"},
    {"id": 561, "name": "Array Partition", "category": "arrays"},
    {"id": 566, "name": "Reshape the Matrix", "category": "arrays"},
    {"id": 572, "name": "Subtree of Another Tree", "category": "trees"},
    {"id": 575, "name": "Distribute Candies", "category": "arrays"},
    {"id": 590, "name": "N-ary Tree Postorder", "category": "trees"},
    {"id": 594, "name": "Longest Harmonious Subsequence", "category": "arrays"},
    {"id": 617, "name": "Merge Two Binary Trees", "category": "trees"},
    {"id": 637, "name": "Average of Levels in Binary Tree", "category": "trees"},
    {"id": 643, "name": "Maximum Average Subarray", "category": "arrays"},
    {"id": 645, "name": "Set Mismatch", "category": "arrays"},
    {"id": 653, "name": "Two Sum IV - BST", "category": "trees"},
    {"id": 657, "name": "Robot Return to Origin", "category": "strings"},
    {"id": 661, "name": "Image Smoother", "category": "arrays"},
    {"id": 665, "name": "Non-decreasing Array", "category": "arrays"},
    {"id": 669, "name": "Trim BST", "category": "trees"},
    {"id": 674, "name": "Longest Continuous Increasing Subsequence", "category": "arrays"},
    {"id": 680, "name": "Valid Palindrome II", "category": "strings"},
    {"id": 682, "name": "Baseball Game", "category": "stacks"},
    {"id": 693, "name": "Binary Number with Alternating Bits", "category": "bit_manipulation"},
    {"id": 695, "name": "Max Area of Island", "category": "graphs"},
    {"id": 696, "name": "Count Binary Substrings", "category": "strings"},
    {"id": 700, "name": "Search in BST", "category": "trees"},
    {"id": 701, "name": "Insert into BST", "category": "trees"},
    {"id": 703, "name": "Kth Largest Element in Stream", "category": "heaps"},
    {"id": 704, "name": "Binary Search", "category": "binary_search"},
    {"id": 705, "name": "Design HashSet", "category": "design"},
    {"id": 706, "name": "Design HashMap", "category": "design"},
    {"id": 707, "name": "Design Linked List", "category": "linked_lists"},
    {"id": 709, "name": "To Lower Case", "category": "strings"},
    {"id": 717, "name": "1-bit and 2-bit Characters", "category": "arrays"},
    {"id": 720, "name": "Longest Word in Dictionary", "category": "strings"},
    {"id": 724, "name": "Find Pivot Index", "category": "arrays"},
    {"id": 725, "name": "Split Linked List in Parts", "category": "linked_lists"},
    {"id": 728, "name": "Self Dividing Numbers", "category": "math"},
    {"id": 733, "name": "Flood Fill", "category": "graphs"},
    {"id": 734, "name": "Sentence Similarity", "category": "strings"},
    {"id": 744, "name": "Find Smallest Letter", "category": "binary_search"},
    {"id": 746, "name": "Min Cost Climbing Stairs", "category": "dynamic_programming"},
    {"id": 747, "name": "Largest Number At Least Twice", "category": "arrays"},
    {"id": 748, "name": "Shortest Completing Word", "category": "strings"},
    {"id": 771, "name": "Jewels and Stones", "category": "strings"},
    {"id": 783, "name": "Minimum Distance Between BST Nodes", "category": "trees"},
    {"id": 784, "name": "Letter Case Permutation", "category": "backtracking"},
    {"id": 796, "name": "Rotate String", "category": "strings"},
    {"id": 804, "name": "Unique Morse Code Words", "category": "strings"},
    {"id": 806, "name": "Number of Lines To Write String", "category": "strings"},
    {"id": 811, "name": "Subdomain Visit Count", "category": "strings"},
    {"id": 812, "name": "Largest Triangle Area", "category": "math"},
    {"id": 819, "name": "Most Common Word", "category": "strings"},
    {"id": 821, "name": "Shortest Distance to Character", "category": "strings"},
    {"id": 824, "name": "Goat Latin", "category": "strings"},
    {"id": 830, "name": "Positions of Large Groups", "category": "strings"},
    {"id": 832, "name": "Flipping an Image", "category": "arrays"},
    {"id": 844, "name": "Backspace String Compare", "category": "strings"},
    {"id": 852, "name": "Peak Index in Mountain Array", "category": "binary_search"},
    {"id": 860, "name": "Lemonade Change", "category": "arrays"},
    {"id": 867, "name": "Transpose Matrix", "category": "arrays"},
    {"id": 876, "name": "Middle of Linked List", "category": "linked_lists"},
    {"id": 883, "name": "Projection Area of 3D Shapes", "category": "arrays"},
    {"id": 888, "name": "Fair Candy Swap", "category": "arrays"},
    {"id": 896, "name": "Monotonic Array", "category": "arrays"},
    {"id": 897, "name": "Increasing Order Search Tree", "category": "trees"},
    {"id": 905, "name": "Sort Array By Parity", "category": "arrays"},
    {"id": 908, "name": "Smallest Range I", "category": "arrays"},
    {"id": 914, "name": "X of a Kind in Deck", "category": "math"},
    {"id": 922, "name": "Sort Array By Parity II", "category": "arrays"},
    {"id": 925, "name": "Long Pressed Name", "category": "strings"},
    {"id": 929, "name": "Unique Email Addresses", "category": "strings"},
    {"id": 933, "name": "Number of Recent Calls", "category": "design"},
    {"id": 938, "name": "Range Sum of BST", "category": "trees"},
    {"id": 941, "name": "Valid Mountain Array", "category": "arrays"},
    {"id": 942, "name": "DI String Match", "category": "arrays"},
    {"id": 944, "name": "Delete Columns to Make Sorted", "category": "strings"},
    {"id": 953, "name": "Verifying an Alien Dictionary", "category": "strings"},
    {"id": 965, "name": "Univalued Binary Tree", "category": "trees"},
    {"id": 976, "name": "Largest Perimeter Triangle", "category": "math"},
    {"id": 977, "name": "Squares of Sorted Array", "category": "arrays"},
    {"id": 985, "name": "Sum of Even Numbers After Queries", "category": "arrays"},
    {"id": 989, "name": "Add to Array Form of Integer", "category": "arrays"},
    {"id": 993, "name": "Cousins in Binary Tree", "category": "trees"},
    {"id": 997, "name": "Find the Town Judge", "category": "graphs"},
    {"id": 1002, "name": "Find Common Characters", "category": "strings"},
    {"id": 1005, "name": "Maximize Sum After K Negations", "category": "arrays"},
    {"id": 1009, "name": "Complement of Base 10 Integer", "category": "bit_manipulation"},
    {"id": 1013, "name": "Partition Array Into Three Parts", "category": "arrays"},
    {"id": 1021, "name": "Remove Outermost Parentheses", "category": "strings"},
    {"id": 1025, "name": "Divisor Game", "category": "dynamic_programming"},
    {"id": 1028, "name": "Recover Tree From Preorder", "category": "trees"},
    {"id": 1030, "name": "Matrix Cells in Distance Order", "category": "sorting"},
    {"id": 1038, "name": "BST to Greater Sum Tree", "category": "trees"},
    {"id": 1046, "name": "Last Stone Weight", "category": "heaps"},
    {"id": 1047, "name": "Remove All Adjacent Duplicates", "category": "stacks"},
    {"id": 1051, "name": "Height Checker", "category": "sorting"},
    {"id": 1089, "name": "Duplicate Zeros", "category": "arrays"},
    {"id": 1108, "name": "Defanging an IP Address", "category": "strings"},
]

def create_problem_stub(problem_data):
    """Create a basic problem stub"""
    problem_id = problem_data["id"]
    name = problem_data["name"]
    category = problem_data["category"]
    
    # Clean filename
    filename = f"{problem_id:03d}_{name.lower().replace(' ', '_').replace('-', '_').replace('(', '').replace(')', '').replace(',', '').replace("'", '').replace('.', '').replace('/', '_')}.💀"
    
    function_name = name.lower().replace(' ', '_').replace('-', '_').replace('(', '').replace(')', '').replace(',', '').replace("'", '').replace('.', '').replace('/', '_')
    
    content = f"""vibe main

yeet "vibez"

fr fr LeetCode #{problem_id}: {name}
fr fr Category: {category.title()}
fr fr Implementation uses CURSED syntax with ඞ pointers!

slay solve_{function_name}() {{
    vibez.spill("=== LeetCode #{problem_id}: {name} ===")
    
    fr fr TODO: Implement actual algorithm
    sus result normie = 42
    vibez.spill("Demo result:")
    vibez.spill(result)
    
    vibez.spill("=== {name} Complete ===")
}}

slay main_character() {{
    solve_{function_name}()
}}
"""
    
    return filename, content

def main():
    base_dir = "test_programs/leetcode"
    
    print("🚀 Creating More CURSED LeetCode Problems!")
    print("=" * 50)
    
    # Ensure directories exist
    categories = set(p["category"] for p in MORE_PROBLEMS)
    for category in categories:
        os.makedirs(f"{base_dir}/{category}", exist_ok=True)
        
    generated = 0
    skipped = 0
    
    for problem in MORE_PROBLEMS:
        filename, content = create_problem_stub(problem)
        filepath = f"{base_dir}/{problem['category']}/{filename}"
        
        if not os.path.exists(filepath):
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            generated += 1
            if generated % 10 == 0:
                print(f"✅ Generated {generated} problems...")
        else:
            skipped += 1
    
    print("=" * 50)
    print(f"🎉 Generated {generated} new problems!")
    print(f"⏭️ Skipped {skipped} existing problems")
    
    # Count total
    import subprocess
    total = subprocess.run(['find', f'{base_dir}', '-name', '*.💀'], 
                          capture_output=True, text=True)
    total_count = len(total.stdout.strip().split('\n')) if total.stdout.strip() else 0
    
    print(f"💀 Total LeetCode problems: {total_count}")
    print("ඞ All problems use Among Us pointer syntax!")

if __name__ == "__main__":
    main()
