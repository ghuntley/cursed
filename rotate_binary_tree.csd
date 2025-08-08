// Binary Tree Rotation Implementation in CURSED
// Demonstrates left and right rotations for AVL/Red-Black tree balancing

yeet "vibez"

// Define the TreeNode structure
squad TreeNode {
    spill value normie           // Node value
    spill left @TreeNode         // Left child pointer
    spill right @TreeNode        // Right child pointer
    spill height normie          // Height for AVL balancing (optional)
}

// Create a new tree node
slay new_node(value normie) @TreeNode {
    sus node @TreeNode = @TreeNode{
        value: value,
        left: nah,
        right: nah,
        height: 1,
    }
    damn node
}

// Right rotation (clockwise)
// Used when left subtree is heavier
//       y                x
//      / \              / \
//     x   C    -->     A   y
//    / \                  / \
//   A   B                B   C
slay rotate_right(y @TreeNode) @TreeNode {
    ready (y == nah || y.left == nah) {
        damn y  // No rotation needed
    }
    
    sus x @TreeNode = y.left
    sus B @TreeNode = x.right
    
    // Perform rotation
    x.right = y
    y.left = B
    
    // Update heights if maintaining AVL property
    y.height = max_height(get_height(y.left), get_height(y.right)) + 1
    x.height = max_height(get_height(x.left), get_height(x.right)) + 1
    
    damn x  // x is new root
}

// Left rotation (counter-clockwise)
// Used when right subtree is heavier
//     x                    y
//    / \                  / \
//   A   y      -->       x   C
//      / \              / \
//     B   C            A   B
slay rotate_left(x @TreeNode) @TreeNode {
    ready (x == nah || x.right == nah) {
        damn x  // No rotation needed
    }
    
    sus y @TreeNode = x.right
    sus B @TreeNode = y.left
    
    // Perform rotation
    y.left = x
    x.right = B
    
    // Update heights if maintaining AVL property
    x.height = max_height(get_height(x.left), get_height(x.right)) + 1
    y.height = max_height(get_height(y.left), get_height(y.right)) + 1
    
    damn y  // y is new root
}

// Get height of a node (0 for null nodes)
slay get_height(node @TreeNode) normie {
    ready (node == nah) {
        damn 0
    }
    damn node.height
}

// Get maximum of two heights
slay max_height(a normie, b normie) normie {
    ready (a > b) {
        damn a
    }
    damn b
}

// Get balance factor for AVL trees
slay get_balance(node @TreeNode) normie {
    ready (node == nah) {
        damn 0
    }
    damn get_height(node.left) - get_height(node.right)
}

// Double rotation: Left-Right rotation
// Used when left child is right-heavy
slay rotate_left_right(node @TreeNode) @TreeNode {
    ready (node == nah || node.left == nah) {
        damn node
    }
    
    // First rotate left child to the left
    node.left = rotate_left(node.left)
    
    // Then rotate the current node to the right
    damn rotate_right(node)
}

// Double rotation: Right-Left rotation
// Used when right child is left-heavy
slay rotate_right_left(node @TreeNode) @TreeNode {
    ready (node == nah || node.right == nah) {
        damn node
    }
    
    // First rotate right child to the right
    node.right = rotate_right(node.right)
    
    // Then rotate the current node to the left
    damn rotate_left(node)
}

// Print tree in-order (for testing)
slay print_inorder(node @TreeNode) {
    ready (node != nah) {
        print_inorder(node.left)
        vibez.spill(node.value)
        print_inorder(node.right)
    }
}

// Example AVL insertion with rotations
slay avl_insert(root @TreeNode, value normie) @TreeNode {
    // Standard BST insertion
    ready (root == nah) {
        damn new_node(value)
    }
    
    ready (value < root.value) {
        root.left = avl_insert(root.left, value)
    } otherwise ready (value > root.value) {
        root.right = avl_insert(root.right, value)
    } otherwise {
        damn root  // Duplicate values not allowed
    }
    
    // Update height of current node
    root.height = 1 + max_height(get_height(root.left), get_height(root.right))
    
    // Get balance factor
    sus balance normie = get_balance(root)
    
    // Left Heavy case
    ready (balance > 1) {
        // Left-Left case
        ready (value < root.left.value) {
            damn rotate_right(root)
        }
        
        // Left-Right case
        ready (value > root.left.value) {
            damn rotate_left_right(root)
        }
    }
    
    // Right Heavy case
    ready (balance < -1) {
        // Right-Right case
        ready (value > root.right.value) {
            damn rotate_left(root)
        }
        
        // Right-Left case
        ready (value < root.right.value) {
            damn rotate_right_left(root)
        }
    }
    
    damn root  // Return unchanged root
}

// Demo function to test rotations
slay demo_rotations() {
    vibez.spill("Creating binary tree with rotations...")
    
    // Create initial tree: 1-2-3 (unbalanced)
    sus root @TreeNode = new_node(1)
    root.right = new_node(2)
    root.right.right = new_node(3)
    
    vibez.spill("Before left rotation:")
    print_inorder(root)
    
    // Perform left rotation to balance
    root = rotate_left(root)
    
    vibez.spill("After left rotation:")
    print_inorder(root)
    
    // Test AVL insertion
    vibez.spill("Building AVL tree with automatic rotations...")
    sus avl_root @TreeNode = nah
    
    sus values []normie = [10, 20, 30, 40, 50, 25]
    sus i normie = 0
    bestie (i < len(values)) {
        avl_root = avl_insert(avl_root, values[i])
        vibez.spill("Inserted", values[i])
        i = i + 1
    }
    
    vibez.spill("Final AVL tree (in-order):")
    print_inorder(avl_root)
}

// Main function demonstrates the rotations
slay main() {
    demo_rotations()
}
