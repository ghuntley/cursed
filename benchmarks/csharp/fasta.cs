// FASTA benchmark - generate and write random DNA sequences

using System;
 using System.Text;

class Fasta
{
    // Constants for the random number generator
    private const int IM = 139968;
    private const int IA = 3877;
    private const int IC = 29573;
    private const int SEED = 42;
    
    // Define DNA sequences
    private const string ALU = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA";
    
    private static double[] IUB_PROB = {
        0.27, 0.12, 0.12, 0.27, 0.02,
        0.02, 0.02, 0.02, 0.02, 0.02,
        0.02, 0.02, 0.02, 0.02, 0.02,
    };
    
    private static char[] IUB_CHAR = {
        'a', 'c', 'g', 't', 'B',
        'D', 'H', 'K', 'M', 'N',
        'R', 'S', 'V', 'W', 'Y',
    };
    
    private static double[] HOMO_SAPIENS_PROB = {
        0.3029549426680, 0.1979883004921,
        0.1975473066391, 0.3015094502008,
    };
    
    private static char[] HOMO_SAPIENS_CHAR = {
        'a', 'c', 'g', 't',
    };
    
    private const int BUF_SIZE = 1024 * 1024;
    
    // Generate a random number
    private static double GenRandom(ref int seed)
    {
        int value = (seed * IA + IC) % IM;
        seed = value;
        return (double)value / (double)IM;
    }
    
    // Generate a random FASTA sequence
    private static string GenRandomFasta(int n, ref int seed, double[] probs, char[] chars)
    {
        StringBuilder buffer = new StringBuilder(n);
        
        for (int i = 0; i < n; i++)
        {
            double r = GenRandom(ref seed);
            char c = '?';
            
            for (int j = 0; j < probs.Length; j++)
            {
                if (r < probs[j])
                {
                    c = chars[j];
                    break;
                }
                r -= probs[j];
            }
            
            buffer.Append(c);
        }
        
        return buffer.ToString();
    }
    
    // Repeat a sequence until it reaches the required length
    private static string RepeatFasta(int n, string seq)
    {
        StringBuilder buffer = new StringBuilder(n);
        int seqLength = seq.Length;
        
        for (int i = 0; i < n; i++)
        {
            buffer.Append(seq[i % seqLength]);
        }
        
        return buffer.ToString();
    }
    
    public static void Main()
    {
        int n = 1000000; // Default sequence length
        int seed = SEED;
        DateTime startTime = DateTime.Now;
        
        // Write FASTA header and sequence for Homo sapiens Alu
        Console.WriteLine(">ONE Homo sapiens alu");
        string aluSeq = RepeatFasta(n, ALU);
        Console.WriteLine(aluSeq);
        
        // Write FASTA header and random sequence for IUB ambiguity codes
        Console.WriteLine(">TWO IUB ambiguity codes");
        string iubSeq = GenRandomFasta(n, ref seed, IUB_PROB, IUB_CHAR);
        Console.WriteLine(iubSeq);
        
        // Write FASTA header and random sequence for Homo sapiens frequency
        Console.WriteLine(">THREE Homo sapiens frequency");
        string sapiensSeq = GenRandomFasta(n, ref seed, HOMO_SAPIENS_PROB, HOMO_SAPIENS_CHAR);
        Console.WriteLine(sapiensSeq);
        
        TimeSpan elapsed = DateTime.Now - startTime;
        Console.WriteLine("Time taken: {0} ms", elapsed.TotalMilliseconds);
        
        // Get memory stats
        long memoryUsed = GC.GetTotalMemory(true) / 1024;
        Console.WriteLine("Memory used: {0} KB", memoryUsed);
    }
}