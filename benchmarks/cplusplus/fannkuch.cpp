// Fannkuch Redux benchmark for C++

#include <iostream>
#include <vector>
#include <algorithm>
#include <chrono>

// Reverse the first n elements of the vector
void flip(std::vector<int>& p, int n) {
    for (int i = 0; i < n/2; i++) {
        std::swap(p[i], p[n-i-1]);
    }
}

// Fannkuch algorithm implementation
int fannkuch(int n) {
    std::vector<int> p(n);
    std::vector<int> perm(n);
    std::vector<int> count(n);
    int maxFlips = 0;
    int checksum = 0;
    int permCount = 0;
    int sign = 1;
    
    // Initialize permutation
    for (int i = 0; i < n; i++) {
        p[i] = i;
    }
    
    while (true) {
        // Copy permutation
        for (int i = 0; i < n; i++) {
            perm[i] = p[i] + 1;
        }
        
        int first = p[0];
        if (first != 0) {
            // Count flips
            std::fill(count.begin(), count.end(), 0);
            int flips = 0;
            while (perm[0] != 1) {
                int k = perm[0] - 1;
                flip(perm, k);
                flips++;
            }
            
            if (flips > maxFlips) {
                maxFlips = flips;
            }
            
            checksum += sign * flips;
        }
        
        // Generate next permutation
        sign = -sign;
        
        // Find j position for next permutation
        int j = 1;
        while (j < n && p[j-1] >= p[j]) {
            j++;
        }
        
        if (j >= n) break;  // No more permutations
        
        permCount++;
        if (permCount >= 10000) break;  // Limit permutations for benchmark
        
        int firstJ = p[j];
        for (int i = 0; i < j; i++) {
            p[i] = p[j-i];
        }
        p[j] = firstJ;
    }
    
    return maxFlips;
}

int main() {
    int n = 10;  // Standard size for the benchmark
    
    auto startTime = std::chrono::high_resolution_clock::now();
    
    int result = fannkuch(n);
    
    std::cout << "Fannkuch(" << n << "): " << result << std::endl;
    
    auto endTime = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime);
    std::cout << "Time taken: " << elapsed.count() << " ms" << std::endl;
    
    return 0;
}