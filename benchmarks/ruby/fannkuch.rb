# Fannkuch Redux benchmark for Ruby

# Flip the first n elements in the array
def flip(p, n)
  (0...n/2).each do |i|
    p[i], p[n-i-1] = p[n-i-1], p[i]
  end
end

# Fannkuch algorithm implementation
def fannkuch(n)
  p = (0...n).to_a
  perm = Array.new(n)
  count = Array.new(n, 0)
  max_flips = 0
  checksum = 0
  perm_count = 0
  sign = 1
  
  loop do
    # Copy permutation
    (0...n).each do |i|
      perm[i] = p[i] + 1
    end
    
    # Count flips
    if p[0] != 0
      flips = 0
      k = p[0]
      
      while k != 0
        flip(perm, k+1)
        flips += 1
        k = perm[0] - 1
      end
      
      max_flips = [max_flips, flips].max
      checksum += sign * flips
    end
    
    # Generate next permutation
    sign = -sign
    
    # Find position for next permutation
    j = 1
    while j < n && p[j-1] >= p[j]
      j += 1
    end
    
    break if j >= n  # No more permutations
    
    perm_count += 1
    break if perm_count >= 10000  # Limit permutations for benchmark
    
    first_j = p[j]
    (0...j).each do |i|
      p[i] = p[j-i-1]
    end
    p[j] = first_j
  end
  
  [max_flips, checksum]
end

# Main function
def main
  n = 10  # Standard size for the benchmark
  
  start_time = Time.now
  
  max_flips, checksum = fannkuch(n)
  
  puts "Fannkuch(#{n}): #{max_flips}"
  puts "Checksum: #{checksum}"
  
  elapsed = Time.now - start_time
  puts "Time taken: #{elapsed * 1000} ms"
end

main