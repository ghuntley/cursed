// String processing benchmark

function processStrings(count, size) {
    let result = "";
    
    for (let i = 0; i < count; i++) {
        const str = createRandomString(size);
        const processed = processString(str);
        result += processed;
    }
    
    return result;
}

function createRandomString(size) {
    const chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let result = "";
    
    for (let i = 0; i < size; i++) {
        const idx = Math.floor(Math.random() * chars.length);
        result += chars.charAt(idx);
    }
    
    return result;
}

function processString(input) {
    let result = input;
    
    // Replace all vowels with their uppercase version
    result = result.replace(/a/g, "A");
    result = result.replace(/e/g, "E");
    result = result.replace(/i/g, "I");
    result = result.replace(/o/g, "O");
    result = result.replace(/u/g, "U");
    
    // Replace all digits with their doubled value
    for (let i = 0; i < 10; i++) {
        const digit = i.toString();
        const doubled = (i * 2).toString();
        result = result.replace(new RegExp(digit, 'g'), doubled);
    }
    
    // Capitalize the first letter
    if (result.length > 0) {
        const first = result.charAt(0).toUpperCase();
        const rest = result.substring(1);
        result = first + rest;
    }
    
    // Reverse the string
    const reversed = result.split('').reverse().join('');
    
    // Take the first half of the reversed string
    const halfLen = Math.floor(reversed.length / 2);
    result = reversed.substring(0, halfLen);
    
    return result;
}

function main() {
    const startTime = Date.now();
    
    // Process strings of different sizes
    const small = processStrings(10000, 10);   // 10,000 strings of length 10
    const medium = processStrings(1000, 100);  // 1,000 strings of length 100
    const large = processStrings(100, 1000);   // 100 strings of length 1,000
    
    const resultLength = small.length + medium.length + large.length;
    console.log(`Processed string length: ${resultLength}`);
    
    const elapsed = Date.now() - startTime;
    console.log(`Time taken: ${elapsed} ms`);
    
    // Get memory stats
    const memoryUsageInMB = process.memoryUsage().heapUsed / 1024 / 1024;
    console.log(`Memory used: ${Math.round(memoryUsageInMB * 1024)} KB`);
}

main();