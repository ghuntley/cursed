// Fannkuch redux benchmark

#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// Reverse the first n elements of the array
void flip(int* p, int n) {
    int temp;
    for (int i = 0; i < n/2; i++) {
        temp = p[i];
        p[i] = p[n-i-1];
        p[n-i-1] = temp;
    }
}

// Count flips required to flip elements to get back to original order
int fannkuch(int n) {
    int* p = (int*)malloc(n * sizeof(int));
    int* perm = (int*)malloc(n * sizeof(int));
    int* count = (int*)malloc(n * sizeof(int));
    int maxFlips = 0;
    int checksum = 0;
    
    // Initialize permutation
    for (int i = 0; i < n; i++) {
        p[i] = i;
    }
    
    int permCount = 0;
    int sign = 1;
    
    while (1) {
        // Copy permutation to perm
        for (int i = 0; i < n; i++) {
            perm[i] = p[i] + 1;
        }
        
        int first = p[0];
        if (first != 0) {
            // Count flips
            for (int i = 0; i < n; i++) {
                count[i] = 0;
            }
            
            int flips = 0;
            int k;
            while (perm[0] != 1) {
                k = perm[0] - 1;
                flip(perm, k);
                flips++;
                perm[0] = k + 1;
            }
            
            if (flips > maxFlips) {
                maxFlips = flips;
            }
            
            checksum += sign * flips;
        }
        
        // Generate next permutation
        sign = -sign;
        int j = 1;
        while (j < n && p[j-1] >= p[j]) {
            j++;
        }
        permCount++;
        
        if (j == n) {
            break;
        }
        
        int first_j = p[j];
        for (int i = 0; i < j; i++) {
            if (i % 2 == 0) {
                int temp = p[i];
                p[i] = p[j-i];
                p[j-i] = temp;
            } else {
                int temp = p[i];
                p[i] = p[j-i-1];
                p[j-i-1] = temp;
            }
        }
        
        if (j < 2) {
            j = 1;
            for (int i = 1; i < n; i++) {
                if (p[i-1] > p[i]) {
                    j = i + 1;
                }
            }
            for (int i = 0; i < j-1; i++) {
                int k = i;
                int temp = p[i];
                while (k < j-1) {
                    k++;
                    p[k-1] = p[k];
                }
                p[j-1] = temp;
            }
        } else {
            j--;
            first_j = p[j];
            for (int i = j; i > 0; i--) {
                p[i] = p[i-1];
            }
            p[0] = first_j;
        }
        
        if (permCount >= 10000) {
            break;
        }
    }
    
    free(p);
    free(perm);
    free(count);
    
    return maxFlips;
}

int main() {
    int n = 10; // Default size
    clock_t start = clock();
    
    int result = fannkuch(n);
    
    printf("Fannkuch(%d): %d\n", n, result);
    
    // Calculate elapsed time
    clock_t end = clock();
    double elapsed = (double)(end - start) * 1000.0 / CLOCKS_PER_SEC;
    printf("Time taken: %.2f ms\n", elapsed);
    
    // Note: C doesn't have a standard way to get memory usage
    printf("Memory monitoring not available for C implementation\n");
    
    return 0;
}