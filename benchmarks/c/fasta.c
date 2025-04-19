// FASTA benchmark - generate and write random DNA sequences

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Constants for the random number generator
#define IM 139968
#define IA 3877
#define IC 29573
#define SEED 42

// DNA sequences and probability tables
#define ALU "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA"

#define BUF_SIZE (1024 * 1024)

// IUB probability array
const double IUB_PROB[] = {
    0.27, 0.12, 0.12, 0.27, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02
};

// IUB characters array
const char IUB_CHAR[] = {
    'a', 'c', 'g', 't', 'B',
    'D', 'H', 'K', 'M', 'N',
    'R', 'S', 'V', 'W', 'Y'
};

// Homo sapiens probability array
const double HOMO_SAPIENS_PROB[] = {
    0.3029549426680, 0.1979883004921,
    0.1975473066391, 0.3015094502008
};

// Homo sapiens characters array
const char HOMO_SAPIENS_CHAR[] = {
    'a', 'c', 'g', 't'
};

// Generate a random number
double gen_random(int* seed) {
    int value = (*seed * IA + IC) % IM;
    *seed = value;
    return (double)value / (double)IM;
}

// Generate a random FASTA sequence
char* gen_random_fasta(int n, int* seed, const double* probs, const char* chars, int char_count) {
    char* buffer = (char*)malloc((n + 1) * sizeof(char));
    
    for (int i = 0; i < n; i++) {
        double r = gen_random(seed);
        char c = '?';
        
        for (int j = 0; j < char_count; j++) {
            if (r < probs[j]) {
                c = chars[j];
                break;
            }
            r -= probs[j];
        }
        
        buffer[i] = c;
    }
    
    buffer[n] = '\0';
    return buffer;
}

// Repeat a sequence until it reaches the required length
char* repeat_fasta(int n, const char* seq) {
    int seq_len = strlen(seq);
    char* buffer = (char*)malloc((n + 1) * sizeof(char));
    
    for (int i = 0; i < n; i++) {
        buffer[i] = seq[i % seq_len];
    }
    
    buffer[n] = '\0';
    return buffer;
}

int main() {
    int n = 1000000; // Default sequence length
    int seed = SEED;
    clock_t start = clock();
    
    // Write FASTA header and sequence for Homo sapiens Alu
    printf(">ONE Homo sapiens alu\n");
    char* alu_seq = repeat_fasta(n, ALU);
    printf("%s\n", alu_seq);
    free(alu_seq);
    
    // Write FASTA header and random sequence for IUB ambiguity codes
    printf(">TWO IUB ambiguity codes\n");
    char* iub_seq = gen_random_fasta(n, &seed, IUB_PROB, IUB_CHAR, 15);
    printf("%s\n", iub_seq);
    free(iub_seq);
    
    // Write FASTA header and random sequence for Homo sapiens frequency
    printf(">THREE Homo sapiens frequency\n");
    char* sapiens_seq = gen_random_fasta(n, &seed, HOMO_SAPIENS_PROB, HOMO_SAPIENS_CHAR, 4);
    printf("%s\n", sapiens_seq);
    free(sapiens_seq);
    
    // Calculate elapsed time
    clock_t end = clock();
    double elapsed = (double)(end - start) * 1000.0 / CLOCKS_PER_SEC;
    printf("Time taken: %.2f ms\n", elapsed);
    
    // Note: C doesn't have a standard way to get memory usage
    printf("Memory monitoring not available for C implementation\n");
    
    return 0;
}