// FASTA benchmark for C++

#include <iostream>
#include <string>
#include <vector>
#include <ctime>
#include <chrono>
#include <algorithm>

// Constants for the random number generator
const int IM = 139968;
const int IA = 3877;
const int IC = 29573;
int SEED = 42;

// Define DNA sequences
const std::string ALU = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA";

const std::vector<double> IUB_PROB = {
    0.27, 0.12, 0.12, 0.27, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
};

const std::vector<char> IUB_CHAR = {
    'a', 'c', 'g', 't', 'B',
    'D', 'H', 'K', 'M', 'N',
    'R', 'S', 'V', 'W', 'Y',
};

const std::vector<double> HOMO_SAPIENS_PROB = {
    0.3029549426680, 0.1979883004921,
    0.1975473066391, 0.3015094502008,
};

const std::vector<char> HOMO_SAPIENS_CHAR = {
    'a', 'c', 'g', 't',
};

// Generate a random number
double genRandom(int& seed) {
    int value = (seed * IA + IC) % IM;
    seed = value;
    return static_cast<double>(value) / IM;
}

// Generate a random FASTA sequence
std::string genRandomFasta(int n, int& seed, const std::vector<double>& probs, const std::vector<char>& chars) {
    size_t length = probs.size();
    std::string buffer;
    buffer.reserve(n);
    
    for (int i = 0; i < n; i++) {
        double r = genRandom(seed);
        char c = ' ';
        
        for (size_t j = 0; j < length; j++) {
            if (r < probs[j]) {
                c = chars[j];
                break;
            }
            r -= probs[j];
        }
        
        buffer.push_back(c);
    }
    
    return buffer;
}

// Repeat a sequence until it reaches the required length
std::string repeatFasta(int n, const std::string& seq) {
    size_t seqLen = seq.length();
    std::string buffer;
    buffer.reserve(n);
    
    for (int i = 0; i < n; i++) {
        buffer.push_back(seq[i % seqLen]);
    }
    
    return buffer;
}

int main() {
    int n = 1000000; // Default sequence length
    int seed = SEED;
    auto startTime = std::chrono::high_resolution_clock::now();
    
    // Write FASTA header and sequence for Homo sapiens Alu
    std::cout << ">ONE Homo sapiens alu" << std::endl;
    std::string aluSeq = repeatFasta(n, ALU);
    std::cout << aluSeq << std::endl;
    
    // Write FASTA header and random sequence for IUB ambiguity codes
    std::cout << ">TWO IUB ambiguity codes" << std::endl;
    std::string iubSeq = genRandomFasta(n, seed, IUB_PROB, IUB_CHAR);
    std::cout << iubSeq << std::endl;
    
    // Write FASTA header and random sequence for Homo sapiens frequency
    std::cout << ">THREE Homo sapiens frequency" << std::endl;
    std::string sapiensSeq = genRandomFasta(n, seed, HOMO_SAPIENS_PROB, HOMO_SAPIENS_CHAR);
    std::cout << sapiensSeq << std::endl;
    
    auto endTime = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime).count();
    std::cout << "Time taken: " << elapsed << " ms" << std::endl;
    
    // Calculate approximate memory usage
    size_t memoryUsage = aluSeq.length() + iubSeq.length() + sapiensSeq.length();
    std::cout << "Memory used: " << memoryUsage / 1024 << " KB" << std::endl;
    
    return 0;
}