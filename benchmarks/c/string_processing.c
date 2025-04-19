// String processing benchmark

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <ctype.h>

// Allocate and initialize a new string with a given size
char* create_random_string(int size) {
    const char chars[] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const int chars_len = strlen(chars);
    
    char* result = malloc(size + 1);
    for (int i = 0; i < size; i++) {
        int idx = rand() % chars_len;
        result[i] = chars[idx];
    }
    result[size] = '\0';
    
    return result;
}

// Replaces all occurrences of from in str with to
void str_replace(char* str, char from, char to) {
    for (char* p = str; *p; p++) {
        if (*p == from) {
            *p = to;
        }
    }
}

// Process a string according to the rules
char* process_string(const char* input) {
    char* result = strdup(input);
    
    // Replace all vowels with their uppercase version
    str_replace(result, 'a', 'A');
    str_replace(result, 'e', 'E');
    str_replace(result, 'i', 'I');
    str_replace(result, 'o', 'O');
    str_replace(result, 'u', 'U');
    
    // Replace all digits with their doubled value (assuming single-digit)
    for (int i = 0; i < 10; i++) {
        char digit = '0' + i;
        char doubled = '0' + (i * 2 % 10); // Handle overflow for 9
        
        for (char* p = result; *p; p++) {
            if (*p == digit) {
                *p = doubled;
            }
        }
    }
    
    // Capitalize the first letter
    if (result[0] != '\0') {
        result[0] = toupper(result[0]);
    }
    
    // Reverse the string
    int len = strlen(result);
    char* reversed = malloc(len + 1);
    for (int i = 0; i < len; i++) {
        reversed[i] = result[len - 1 - i];
    }
    reversed[len] = '\0';
    free(result);
    
    // Take the first half of the reversed string
    int half_len = len / 2;
    reversed[half_len] = '\0';
    
    return reversed;
}

// Process multiple strings
char* process_strings(int count, int size) {
    // For simplicity, we'll just concatenate the last few characters of each processed string
    int result_size = count * 3; // Store a few chars per string
    char* result = malloc(result_size + 1);
    int result_pos = 0;
    
    for (int i = 0; i < count; i++) {
        char* str = create_random_string(size);
        char* processed = process_string(str);
        
        // Store the first few characters of the processed string
        int processed_len = strlen(processed);
        int chars_to_keep = processed_len < 3 ? processed_len : 3;
        
        strncpy(result + result_pos, processed, chars_to_keep);
        result_pos += chars_to_keep;
        
        free(str);
        free(processed);
    }
    
    result[result_pos] = '\0';
    return result;
}

int main() {
    // Seed the random number generator
    srand(time(NULL));
    
    clock_t start = clock();
    
    // Process strings of different sizes
    char* small = process_strings(10000, 10);   // 10,000 strings of length 10
    char* medium = process_strings(1000, 100);  // 1,000 strings of length 100
    char* large = process_strings(100, 1000);   // 100 strings of length 1,000
    
    int result_length = strlen(small) + strlen(medium) + strlen(large);
    printf("Processed string length: %d\n", result_length);
    
    // Free allocated strings
    free(small);
    free(medium);
    free(large);
    
    // Calculate elapsed time
    clock_t end = clock();
    double elapsed = (double)(end - start) * 1000.0 / CLOCKS_PER_SEC;
    printf("Time taken: %.2f ms\n", elapsed);
    
    // Note: C doesn't have a standard way to get memory usage
    printf("Memory monitoring not available for C implementation\n");
    
    return 0;
}