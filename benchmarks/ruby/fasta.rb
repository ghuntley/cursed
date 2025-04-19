#!/usr/bin/env ruby
# FASTA benchmark for Ruby

require 'benchmark'

# Constants for the random number generator
IM = 139968
IA = 3877
IC = 29573
SEED = 42

# Define DNA sequences
ALU = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA"

IUB_PROB = [
    0.27, 0.12, 0.12, 0.27, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
]

IUB_CHAR = [
    'a', 'c', 'g', 't', 'B',
    'D', 'H', 'K', 'M', 'N',
    'R', 'S', 'V', 'W', 'Y',
]

HOMO_SAPIENS_PROB = [
    0.3029549426680, 0.1979883004921,
    0.1975473066391, 0.3015094502008,
]

HOMO_SAPIENS_CHAR = [
    'a', 'c', 'g', 't',
]

# Generate a random number
def gen_random(seed)
  value = (seed * IA + IC) % IM
  [value, value.to_f / IM]
end

# Generate a random FASTA sequence
def gen_random_fasta(n, seed, probs, chars)
  length = probs.length
  buffer = ""
  
  n.times do
    seed, r = gen_random(seed)
    c = ' '
    
    length.times do |j|
      if r < probs[j]
        c = chars[j]
        break
      end
      r -= probs[j]
    end
    
    buffer << c
  end
  
  [seed, buffer]
end

# Repeat a sequence until it reaches the required length
def repeat_fasta(n, seq)
  seq_len = seq.length
  buffer = ""
  
  n.times do |i|
    buffer << seq[i % seq_len]
  end
  
  buffer
end

def main
  n = 1_000_000 # Default sequence length
  seed = SEED
  start_time = Time.now
  
  # Write FASTA header and sequence for Homo sapiens Alu
  puts ">ONE Homo sapiens alu"
  alu_seq = repeat_fasta(n, ALU)
  puts alu_seq
  
  # Write FASTA header and random sequence for IUB ambiguity codes
  puts ">TWO IUB ambiguity codes"
  seed, iub_seq = gen_random_fasta(n, seed, IUB_PROB, IUB_CHAR)
  puts iub_seq
  
  # Write FASTA header and random sequence for Homo sapiens frequency
  puts ">THREE Homo sapiens frequency"
  seed, sapiens_seq = gen_random_fasta(n, seed, HOMO_SAPIENS_PROB, HOMO_SAPIENS_CHAR)
  puts sapiens_seq
  
  elapsed = (Time.now - start_time) * 1000
  puts "Time taken: #{elapsed.round} ms"
  
  # Calculate approximate memory usage
  memory_usage = alu_seq.length + iub_seq.length + sapiens_seq.length
  puts "Memory used: #{memory_usage / 1024} KB"
end

main