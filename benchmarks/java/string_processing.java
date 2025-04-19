// String processing benchmark

import java.util.Random;

public class ClassName {
    private static Random random = new Random();
    
    private static String processStrings(int count, int size) {
        StringBuilder result = new StringBuilder();
        
        for (int i = 0; i < count; i++) {
            String str = createRandomString(size);
            String processed = processString(str);
            result.append(processed);
        }
        
        return result.toString();
    }
    
    private static String createRandomString(int size) {
        String chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        StringBuilder result = new StringBuilder(size);
        
        for (int i = 0; i < size; i++) {
            int idx = random.nextInt(chars.length());
            result.append(chars.charAt(idx));
        }
        
        return result.toString();
    }
    
    private static String processString(String input) {
        String result = input;
        
        // Replace all vowels with their uppercase version
        result = result.replace("a", "A");
        result = result.replace("e", "E");
        result = result.replace("i", "I");
        result = result.replace("o", "O");
        result = result.replace("u", "U");
        
        // Replace all digits with their doubled value
        for (int i = 0; i < 10; i++) {
            String digit = Integer.toString(i);
            String doubled = Integer.toString(i * 2);
            result = result.replace(digit, doubled);
        }
        
        // Capitalize the first letter
        if (result.length() > 0) {
            String first = result.substring(0, 1).toUpperCase();
            String rest = result.substring(1);
            result = first + rest;
        }
        
        // Reverse the string
        StringBuilder reversed = new StringBuilder(result);
        reversed.reverse();
        
        // Take the first half of the reversed string
        int halfLen = reversed.length() / 2;
        result = reversed.substring(0, halfLen);
        
        return result;
    }
    
    public static void main(String[] args) {
        long startTime = System.currentTimeMillis();
        
        // Process strings of different sizes
        String small = processStrings(10000, 10);   // 10,000 strings of length 10
        String medium = processStrings(1000, 100);  // 1,000 strings of length 100
        String large = processStrings(100, 1000);   // 100 strings of length 1,000
        
        int resultLength = small.length() + medium.length() + large.length();
        System.out.println("Processed string length: " + resultLength);
        
        long elapsed = System.currentTimeMillis() - startTime;
        System.out.println("Time taken: " + elapsed + " ms");
        
        // Get memory stats
        long memoryUsed = (Runtime.getRuntime().totalMemory() - Runtime.getRuntime().freeMemory()) / 1024;
        System.out.println("Memory used: " + memoryUsed + " KB");
    }
}