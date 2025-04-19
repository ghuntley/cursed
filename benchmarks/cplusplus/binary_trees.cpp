// Binary trees benchmark for C++

#include <iostream>
#include <chrono>
#include <memory>
#include <cstdint>

class TreeNode {
public:
    TreeNode* left;
    TreeNode* right;
    int item;
    
    TreeNode(int i, TreeNode* l = nullptr, TreeNode* r = nullptr)
        : item(i), left(l), right(r) {}
    
    ~TreeNode() {
        delete left;
        delete right;
    }
};

// Create a tree of specified depth
TreeNode* makeTree(int depth, int item = 0) {
    if (depth <= 0) {
        return new TreeNode(item);
    }
    return new TreeNode(
        item,
        makeTree(depth - 1, 2 * item - 1),
        makeTree(depth - 1, 2 * item)
    );
}

// Check a tree by calculating checksum
int checkTree(TreeNode* node) {
    if (!node) return 0;
    if (!node->left) return node->item;
    return node->item + checkTree(node->left) - checkTree(node->right);
}

int main() {
    const int minDepth = 4;
    const int maxDepth = 12;
    const int stretchDepth = maxDepth + 1;
    
    auto startTime = std::chrono::high_resolution_clock::now();
    
    // Stretch tree
    TreeNode* stretchTree = makeTree(stretchDepth);
    std::cout << "stretch tree of depth " << stretchDepth
              << "\t check: " << checkTree(stretchTree) << std::endl;
    delete stretchTree;
    
    // Long-lived tree
    TreeNode* longLivedTree = makeTree(maxDepth);
    
    // Trees of increasing depth
    for (int depth = minDepth; depth <= maxDepth; depth += 2) {
        int iterations = 1 << (maxDepth - depth + minDepth);
        int check = 0;
        
        for (int i = 0; i < iterations; i++) {
            TreeNode* a = makeTree(depth, i);
            TreeNode* b = makeTree(depth, -i);
            check += checkTree(a) + checkTree(b);
            delete a;
            delete b;
        }
        
        std::cout << iterations * 2 << "\t trees of depth " << depth
                  << "\t check: " << check << std::endl;
    }
    
    std::cout << "long lived tree of depth " << maxDepth
              << "\t check: " << checkTree(longLivedTree) << std::endl;
    delete longLivedTree;
    
    auto endTime = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime);
    std::cout << "Time taken: " << elapsed.count() << " ms" << std::endl;
    
    return 0;
}