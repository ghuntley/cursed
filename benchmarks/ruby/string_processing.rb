# String processing benchmark for Ruby

# Create a random string of specified length
def create_random_string(length)
  chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
  result = ""
  
  length.times do
    result += chars[rand(chars.length)]
  end
  
  result
end

# Process a string with various operations
def process_string(input)
  # Replace all vowels with uppercase version
  result = input.gsub('a', 'A')
                .gsub('e', 'E')
                .gsub('i', 'I')
                .gsub('o', 'O')
                .gsub('u', 'U')
  
  # Replace digits with doubled value
  (0..9).each do |i|
    result = result.gsub(i.to_s, (i * 2).to_s)
  end
  
  # Capitalize first letter if string is not empty
  result = result.capitalize unless result.empty?
  
  # Reverse the string
  reversed = result.reverse
  
  # Take first half
  reversed[0...(reversed.length / 2)]
end

# Process multiple strings of different sizes
def process_strings(count, size)
  result = ""
  
  count.times do
    str = create_random_string(size)
    processed = process_string(str)
    result += processed
  end
  
  result
end

# Main function
def main
  start_time = Time.now
  
  # Process strings of different sizes
  small = process_strings(10000, 10)    # 10,000 strings of length 10
  medium = process_strings(1000, 100)   # 1,000 strings of length 100
  large = process_strings(100, 1000)    # 100 strings of length 1,000
  
  result_length = small.length + medium.length + large.length
  puts "Processed string length: #{result_length}"
  
  elapsed = Time.now - start_time
  puts "Time taken: #{elapsed * 1000} ms"
end

main