// String processing benchmark

using System;
using System.Text;

class StringProcessing
{
    static string ProcessStrings(int count, int size)
    {
        StringBuilder result = new StringBuilder();
        
        for (int i = 0; i < count; i++)
        {
            string str = CreateRandomString(size);
            string processed = ProcessString(str);
            result.Append(processed);
        }
        
        return result.ToString();
    }
    
    static string CreateRandomString(int size)
    {
        string chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        StringBuilder result = new StringBuilder(size);
        Random random = new Random();
        
        for (int i = 0; i < size; i++)
        {
            int idx = random.Next(chars.Length);
            result.Append(chars[idx]);
        }
        
        return result.ToString();
    }
    
    static string ProcessString(string input)
    {
        string result = input;
        
        // Replace all vowels with their uppercase version
        result = result.Replace("a", "A")
                       .Replace("e", "E")
                       .Replace("i", "I")
                       .Replace("o", "O")
                       .Replace("u", "U");
        
        // Replace all digits with their doubled value
        for (int i = 0; i < 10; i++)
        {
            string digit = i.ToString();
            string doubled = (i * 2).ToString();
            result = result.Replace(digit, doubled);
        }
        
        // Capitalize the first letter
        if (result.Length > 0)
        {
            result = char.ToUpper(result[0]) + result.Substring(1);
        }
        
        // Reverse the string
        char[] charArray = result.ToCharArray();
        Array.Reverse(charArray);
        string reversed = new string(charArray);
        
        // Take the first half of the reversed string
        int halfLen = reversed.Length / 2;
        result = reversed.Substring(0, halfLen);
        
        return result;
    }
    
    public static void Main()
    {
        DateTime startTime = DateTime.Now;
        
        // Process strings of different sizes
        string small = ProcessStrings(10000, 10);   // 10,000 strings of length 10
        string medium = ProcessStrings(1000, 100);  // 1,000 strings of length 100
        string large = ProcessStrings(100, 1000);   // 100 strings of length 1,000
        
        int resultLength = small.Length + medium.Length + large.Length;
        Console.WriteLine("Processed string length: {0}", resultLength);
        
        TimeSpan elapsed = DateTime.Now - startTime;
        Console.WriteLine("Time taken: {0} ms", elapsed.TotalMilliseconds);
        
        // Get memory stats
        long memoryUsed = GC.GetTotalMemory(true) / 1024;
        Console.WriteLine("Memory used: {0} KB", memoryUsed);
    }
}