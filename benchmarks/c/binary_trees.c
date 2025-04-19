// Binary trees benchmark adapted from The Computer Language Benchmarks Game

#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// A TreeNode structure
typedef struct TreeNode {
    struct TreeNode *left, *right;
    int item;
} TreeNode;

// Create a new tree with the given item value at the root
TreeNode* create_tree(int item, int depth) {
    TreeNode* node = malloc(sizeof(TreeNode));
    node->item = item;
    
    if (depth > 0) {
        node->left = create_tree(2 * item - 1, depth - 1);
        node->right = create_tree(2 * item, depth - 1);
    } else {
        node->left = node->right = NULL;
    }
    
    return node;
}

// Check the tree and return a checksum
int check_tree(TreeNode* node) {
    if (node->left == NULL) {
        return node->item;
    }
    return node->item + check_tree(node->left) - check_tree(node->right);
}

// Free the memory used by a tree
void delete_tree(TreeNode* node) {
    if (node->left != NULL) {
        delete_tree(node->left);
        delete_tree(node->right);
    }
    free(node);
}

int main() {
    int min_depth = 4;
    int max_depth = 12;
    
    int stretch_depth = max_depth + 1;
    clock_t start = clock();
    
    // Allocate and check a big tree
    TreeNode* big_tree = create_tree(0, stretch_depth);
    printf("stretch tree of depth %d check: %d\n", stretch_depth, check_tree(big_tree));
    
    // Free the big tree
    delete_tree(big_tree);
    
    // Allocate a long-lived binary tree
    TreeNode* long_lived_tree = create_tree(0, max_depth);
    
    // Check trees of increasing depth
    for (int depth = min_depth; depth <= max_depth; depth += 2) {
        int iterations = 1 << (max_depth - depth + min_depth);
        int result = 0;
        
        for (int i = 0; i < iterations; i++) {
            TreeNode* a = create_tree(i, depth);
            TreeNode* b = create_tree(-i, depth);
            result += check_tree(a) + check_tree(b);
            delete_tree(a);
            delete_tree(b);
        }
        
        printf("%d trees of depth %d check: %d\n", iterations * 2, depth, result);
    }
    
    // Check the long-lived tree last
    printf("long lived tree of depth %d check: %d\n", max_depth, check_tree(long_lived_tree));
    
    // Free the long-lived tree
    delete_tree(long_lived_tree);
    
    // Calculate elapsed time
    clock_t end = clock();
    double elapsed = (double)(end - start) * 1000.0 / CLOCKS_PER_SEC;
    printf("Time taken: %.2f ms\n", elapsed);
    
    // Note: C doesn't have a standard way to get memory usage
    // This would typically be platform dependent (e.g., getrusage on Unix)
    printf("Memory monitoring not available for C implementation\n");
    
    return 0;
}