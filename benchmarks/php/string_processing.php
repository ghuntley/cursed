#!/usr/bin/env php
<?php
// String processing benchmark

function processStrings($count, $size) {
    $result = "";
    
    for ($i = 0; $i < $count; $i++) {
        $str = createRandomString($size);
        $processed = processString($str);
        $result .= $processed;
    }
    
    return $result;
}

function createRandomString($size) {
    $chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    $result = "";
    
    for ($i = 0; $i < $size; $i++) {
        $idx = rand(0, strlen($chars) - 1);
        $result .= $chars[$idx];
    }
    
    return $result;
}

function processString($input) {
    $result = $input;
    
    // Replace all vowels with their uppercase version
    $result = str_replace("a", "A", $result);
    $result = str_replace("e", "E", $result);
    $result = str_replace("i", "I", $result);
    $result = str_replace("o", "O", $result);
    $result = str_replace("u", "U", $result);
    
    // Replace all digits with their doubled value
    for ($i = 0; $i < 10; $i++) {
        $digit = (string)$i;
        $doubled = (string)($i * 2);
        $result = str_replace($digit, $doubled, $result);
    }
    
    // Capitalize the first letter
    if (strlen($result) > 0) {
        $result = ucfirst($result);
    }
    
    // Reverse the string
    $reversed = strrev($result);
    
    // Take the first half of the reversed string
    $halfLen = (int)(strlen($reversed) / 2);
    $result = substr($reversed, 0, $halfLen);
    
    return $result;
}

function main() {
    $startTime = microtime(true);
    
    // Process strings of different sizes
    $small = processStrings(10000, 10);   // 10,000 strings of length 10
    $medium = processStrings(1000, 100);  // 1,000 strings of length 100
    $large = processStrings(100, 1000);   // 100 strings of length 1,000
    
    $resultLength = strlen($small) + strlen($medium) + strlen($large);
    echo "Processed string length: $resultLength\n";
    
    $elapsed = (microtime(true) - $startTime) * 1000;
    echo "Time taken: $elapsed ms\n";
    
    // Get memory stats
    $memoryUsed = memory_get_peak_usage(true) / 1024;
    echo "Memory used: $memoryUsed KB\n";
}

main();