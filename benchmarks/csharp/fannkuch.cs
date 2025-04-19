// Fannkuch redux benchmark

using System;

class Fannkuch
{
    // Reverse the first n elements of the array
    private static void Flip(int[] p, int n)
    {
        for (int i = 0; i < n/2; i++)
        {
            int temp = p[i];
            p[i] = p[n-i-1];
            p[n-i-1] = temp;
        }
    }
    
    // Count flips required to flip elements to get back to original order
    private static int FannkuchCalculation(int n)
    {
        int[] p = new int[n];
        int[] perm = new int[n];
        int[] count = new int[n];
        int maxFlips = 0;
        int checksum = 0;
        
        // Initialize permutation
        for (int i = 0; i < n; i++)
        {
            p[i] = i;
        }
        
        int permCount = 0;
        int sign = 1;
        
        while (true)
        {
            // Copy permutation to perm
            for (int i = 0; i < n; i++)
            {
                perm[i] = p[i] + 1;
            }
            
            int first = p[0];
            if (first != 0)
            {
                // Count flips
                for (int i = 0; i < n; i++)
                {
                    count[i] = 0;
                }
                
                int flips = 0;
                while (perm[0] != 1)
                {
                    int k = perm[0] - 1;
                    Flip(perm, k);
                    flips++;
                    perm[0] = k + 1;
                }
                
                if (flips > maxFlips)
                {
                    maxFlips = flips;
                }
                
                checksum += sign * flips;
            }
            
            // Generate next permutation
            sign = -sign;
            int j = 1;
            while (j < n && p[j-1] >= p[j])
            {
                j++;
            }
            
            if (j == n)
            {
                break;
            }
            
            permCount++;
            
            int firstJ = p[j];
            for (int i = 0; i < j; i++)
            {
                if (i % 2 == 0)
                {
                    int temp = p[i];
                    p[i] = p[j-i];
                    p[j-i] = temp;
                }
                else
                {
                    int temp = p[i];
                    p[i] = p[j-i-1];
                    p[j-i-1] = temp;
                }
            }
            
            if (j < 2)
            {
                j = 1;
                for (int i = 1; i < n; i++)
                {
                    if (p[i-1] > p[i])
                    {
                        j = i + 1;
                    }
                }
                
                for (int i = 0; i < j-1; i++)
                {
                    int k = i;
                    int temp = p[i];
                    while (k < j-1)
                    {
                        k++;
                        p[k-1] = p[k];
                    }
                    p[j-1] = temp;
                }
            }
            else
            {
                j--;
                firstJ = p[j];
                for (int i = j; i > 0; i--)
                {
                    p[i] = p[i-1];
                }
                p[0] = firstJ;
            }
            
            if (permCount >= 10000)
            {
                break;
            }
        }
        
        return maxFlips;
    }
    
    public static void Main()
    {
        int n = 10; // Default size
        DateTime startTime = DateTime.Now;
        
        int result = FannkuchCalculation(n);
        
        Console.WriteLine("Fannkuch({0}): {1}", n, result);
        
        TimeSpan elapsed = DateTime.Now - startTime;
        Console.WriteLine("Time taken: {0} ms", elapsed.TotalMilliseconds);
        
        // Get memory stats
        long memoryUsed = GC.GetTotalMemory(true) / 1024;
        Console.WriteLine("Memory used: {0} KB", memoryUsed);
    }
}