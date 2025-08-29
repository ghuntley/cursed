vibe main

yeet "vibez"

# Binary tree node structure
be_like TreeNode squad {
    value normie
    left @TreeNode
    right @TreeNode
}

# Create a new tree node
slay new_node(val normie) @TreeNode {
    sus node @TreeNode = @TreeNode{
        value: val,
        left: nah,
        right: nah
    }
    damn node
}

# Reverse/invert a binary tree recursively
slay reverse_tree(root @TreeNode) @TreeNode {
    # Base case: empty tree
    ready root == nah {
        damn nah
    }
    
    # Swap left and right children
    sus temp @TreeNode = root.left
    root.left = root.right
    root.right = temp
    
    # Recursively reverse left and right subtrees
    reverse_tree(root.left)
    reverse_tree(root.right)
    
    damn root
}

# Print tree in-order traversal
slay print_inorder(root @TreeNode) {
    ready root != nah {
        print_inorder(root.left)
        vibez.spill(root.value)
        print_inorder(root.right)
    }
}

# Build example tree:
#       1
#      / \
#     2   3
#    / \
#   4   5
slay build_example_tree() @TreeNode {
    sus root @TreeNode = new_node(1)
    root.left = new_node(2)
    root.right = new_node(3)
    root.left.left = new_node(4)
    root.left.right = new_node(5)
    damn root
}

slay main() {
    vibez.spill("Building example binary tree...")
    
    sus tree @TreeNode = build_example_tree()
    
    vibez.spill("Original tree (in-order):")
    print_inorder(tree)
    
    vibez.spill("Reversing the tree...")
    reverse_tree(tree)
    
    vibez.spill("Reversed tree (in-order):")
    print_inorder(tree)
}
