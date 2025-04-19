// String processing benchmark for C++

#include <iostream>
#include <string>
#include <vector>
#include <random>
#include <algorithm>
#include <chrono>

// Create a random string of specified length
std::string createRandomString(int length) {
    static const std::string chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    static std::random_device rd;
    static std::mt19937 generator(rd());
    static std::uniform_int_distribution<> distribution(0, chars.size() - 1);
    
    std::string result(length, ' ');
    for (int i = 0; i < length; i++) {
        result[i] = chars[distribution(generator)];
    }
    
    return result;
}

// Process a string with various operations
std::string processString(const std::string& input) {
    // Replace all vowels with uppercase version
    std::string result = input;
    
    // Replace vowels
    for (size_t i = 0; i < result.length(); i++) {
        switch (result[i]) {
            case 'a': result[i] = 'A'; break;
            case 'e': result[i] = 'E'; break;
            case 'i': result[i] = 'I'; break;
            case 'o': result[i] = 'O'; break;
            case 'u': result[i] = 'U'; break;
        }
    }
    
    // Replace digits with doubled value
    for (size_t i = 0; i < result.length(); i++) {
        if (std::isdigit(result[i])) {
            int digit = result[i] - '0';
            int doubled = digit * 2;
            // We need to handle multi-digit numbers in string replacement
            if (doubled < 10) {
                result[i] = '0' + doubled;
            } else {
                std::string doubledStr = std::to_string(doubled);
                result.replace(i, 1, doubledStr);
                i += doubledStr.length() - 1;  // Adjust for added characters
            }
        }
    }
    
    // Capitalize first letter if string is not empty
    if (!result.empty()) {
        result[0] = std::toupper(result[0]);
    }
    
    // Reverse the string
    std::reverse(result.begin(), result.end());
    
    // Take first half
    result = result.substr(0, result.length() / 2);
    
    return result;
}

// Process multiple strings of different sizes
std::string processStrings(int count, int size) {
    std::string result;
    
    for (int i = 0; i < count; i++) {
        std::string str = createRandomString(size);
        std::string processed = processString(str);
        result += processed;
    }
    
    return result;
}

int main() {
    auto startTime = std::chrono::high_resolution_clock::now();
    
    // Process strings of different sizes
    std::string small = processStrings(10000, 10);    // 10,000 strings of length 10
    std::string medium = processStrings(1000, 100);   // 1,000 strings of length 100
    std::string large = processStrings(100, 1000);    // 100 strings of length 1,000
    
    size_t resultLength = small.length() + medium.length() + large.length();
    std::cout << "Processed string length: " << resultLength << std::endl;
    
    auto endTime = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime);
    std::cout << "Time taken: " << elapsed.count() << " ms" << std::endl;
    
    return 0;
}