// Binary trees benchmark adapted from The Computer Language Benchmarks Game

public class ClassName {
    private static class TreeNode {
        private TreeNode left, right;
        private int item;
        
        public TreeNode(int item) {
            this.item = item;
        }
        
        // Create a new tree with the given item value at the root
        public static TreeNode createTree(int item, int depth) {
            TreeNode node = new TreeNode(item);
            
            if (depth > 0) {
                node.left = createTree(2 * item - 1, depth - 1);
                node.right = createTree(2 * item, depth - 1);
            }
            
            return node;
        }
        
        // Check the tree and return a checksum
        public int check() {
            if (left == null) {
                return item;
            }
            return item + left.check() - right.check();
        }
    }
    
    public static void main(String[] args) {
        int minDepth = 4;
        int maxDepth = 12;
        
        int stretchDepth = maxDepth + 1;
        long startTime = System.currentTimeMillis();
        
        // Allocate and check a big tree
        TreeNode bigTree = TreeNode.createTree(0, stretchDepth);
        System.out.println("stretch tree of depth " + stretchDepth + " check: " + bigTree.check());
        
        // Allow the big tree to be garbage collected
        bigTree = null;
        
        // Allocate a long-lived binary tree
        TreeNode longLivedTree = TreeNode.createTree(0, maxDepth);
        
        // Check trees of increasing depth
        for (int depth = minDepth; depth <= maxDepth; depth += 2) {
            int iterations = 1 << (maxDepth - depth + minDepth);
            int result = 0;
            
            for (int i = 0; i < iterations; i++) {
                TreeNode a = TreeNode.createTree(i, depth);
                TreeNode b = TreeNode.createTree(-i, depth);
                result += a.check() + b.check();
            }
            
            System.out.println((iterations * 2) + " trees of depth " + depth + " check: " + result);
        }
        
        // Check the long-lived tree last
        System.out.println("long lived tree of depth " + maxDepth + " check: " + longLivedTree.check());
        
        long elapsed = System.currentTimeMillis() - startTime;
        System.out.println("Time taken: " + elapsed + " ms");
        
        // Get memory stats
        long memoryUsed = (Runtime.getRuntime().totalMemory() - Runtime.getRuntime().freeMemory()) / 1024;
        System.out.println("Memory used: " + memoryUsed + " KB");
    }
}