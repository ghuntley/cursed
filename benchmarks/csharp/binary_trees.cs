// Binary trees benchmark adapted from The Computer Language Benchmarks Game

using System;
using System.Threading;

class BinaryTrees
{
    class TreeNode
    {
        public TreeNode left, right;
        public int item;
        
        public TreeNode(int item)
        {
            this.item = item;
        }
        
        // Create a new tree with the given item value at the root
        public static TreeNode CreateTree(int item, int depth)
        {
            TreeNode node = new TreeNode(item);
            
            if (depth > 0)
            {
                node.left = CreateTree(2 * item - 1, depth - 1);
                node.right = CreateTree(2 * item, depth - 1);
            }
            
            return node;
        }
        
        // Check the tree and return a checksum
        public int Check()
        {
            if (left == null)
            {
                return item;
            }
            return item + left.Check() - right.Check();
        }
    }
    
    public static void Main(string[] args)
    {
        int minDepth = 4;
        int maxDepth = 12;
        
        int stretchDepth = maxDepth + 1;
        DateTime startTime = DateTime.Now;
        
        // Allocate and check a big tree
        TreeNode bigTree = TreeNode.CreateTree(0, stretchDepth);
        Console.WriteLine("stretch tree of depth {0} check: {1}", stretchDepth, bigTree.Check());
        
        // Allow the big tree to be garbage collected
        bigTree = null;
        
        // Allocate a long-lived binary tree
        TreeNode longLivedTree = TreeNode.CreateTree(0, maxDepth);
        
        // Check trees of increasing depth
        for (int depth = minDepth; depth <= maxDepth; depth += 2)
        {
            int iterations = 1 << (maxDepth - depth + minDepth);
            int result = 0;
            
            for (int i = 0; i < iterations; i++)
            {
                TreeNode a = TreeNode.CreateTree(i, depth);
                TreeNode b = TreeNode.CreateTree(-i, depth);
                result += a.Check() + b.Check();
            }
            
            Console.WriteLine("{0} trees of depth {1} check: {2}", iterations * 2, depth, result);
        }
        
        // Check the long-lived tree last
        Console.WriteLine("long lived tree of depth {0} check: {1}", maxDepth, longLivedTree.Check());
        
        TimeSpan elapsed = DateTime.Now - startTime;
        Console.WriteLine("Time taken: {0} ms", elapsed.TotalMilliseconds);
        
        // Get memory stats
        long memoryUsed = GC.GetTotalMemory(true) / 1024;
        Console.WriteLine("Memory used: {0} KB", memoryUsed);
    }
}