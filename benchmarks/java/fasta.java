// FASTA benchmark - generate and write random DNA sequences

import java.text.*;
import java.util.*;

public class ClassName {
    // Constants for the random number generator
    private static final int IM = 139968;
    private static final int IA = 3877;
    private static final int IC = 29573;
    private static final int SEED = 42;
    
    // Define DNA sequences
    private static final String ALU = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA";
    
    private static final double[] IUB_PROB = {
        0.27, 0.12, 0.12, 0.27, 0.02,
        0.02, 0.02, 0.02, 0.02, 0.02,
        0.02, 0.02, 0.02, 0.02, 0.02,
    };
    
    private static final char[] IUB_CHAR = {
        'a', 'c', 'g', 't', 'B',
        'D', 'H', 'K', 'M', 'N',
        'R', 'S', 'V', 'W', 'Y',
    };
    
    private static final double[] HOMO_SAPIENS_PROB = {
        0.3029549426680, 0.1979883004921,
        0.1975473066391, 0.3015094502008,
    };
    
    private static final char[] HOMO_SAPIENS_CHAR = {
        'a', 'c', 'g', 't',
    };
    
    private static final int BUF_SIZE = 1024 * 1024;
    
    // Generate a random number
    private static double genRandom(int[] seed) {
        int value = (seed[0] * IA + IC) % IM;
        seed[0] = value;
        return (double)value / (double)IM;
    }
    
    // Generate a random FASTA sequence
    private static String genRandomFasta(int n, int[] seed, double[] probs, char[] chars) {
        int length = probs.length;
        StringBuilder buffer = new StringBuilder(n);
        
        for (int i = 0; i < n; i++) {
            double r = genRandom(seed);
            char c = ' ';
            
            for (int j = 0; j < length; j++) {
                if (r < probs[j]) {
                    c = chars[j];
                    break;
                }
                r -= probs[j];
            }
            
            buffer.append(c);
        }
        
        return buffer.toString();
    }
    
    // Repeat a sequence until it reaches the required length
    private static String repeatFasta(int n, String seq) {
        int seqLen = seq.length();
        StringBuilder buffer = new StringBuilder(n);
        
        for (int i = 0; i < n; i++) {
            buffer.append(seq.charAt(i % seqLen));
        }
        
        return buffer.toString();
    }
    
    public static void main(String[] args) {
        int n = 1000000; // Default sequence length
        int[] seed = {SEED};
        long startTime = System.currentTimeMillis();
        
        // Write FASTA header and sequence for Homo sapiens Alu
        System.out.println(">ONE Homo sapiens alu");
        String aluSeq = repeatFasta(n, ALU);
        System.out.println(aluSeq);
        
        // Write FASTA header and random sequence for IUB ambiguity codes
        System.out.println(">TWO IUB ambiguity codes");
        String iubSeq = genRandomFasta(n, seed, IUB_PROB, IUB_CHAR);
        System.out.println(iubSeq);
        
        // Write FASTA header and random sequence for Homo sapiens frequency
        System.out.println(">THREE Homo sapiens frequency");
        String sapiensSeq = genRandomFasta(n, seed, HOMO_SAPIENS_PROB, HOMO_SAPIENS_CHAR);
        System.out.println(sapiensSeq);
        
        long elapsed = System.currentTimeMillis() - startTime;
        System.out.printf("Time taken: %d ms\n", elapsed);
        
        // Get memory stats
        long memoryUsed = (Runtime.getRuntime().totalMemory() - Runtime.getRuntime().freeMemory()) / 1024;
        System.out.printf("Memory used: %d KB\n", memoryUsed);
    }
}