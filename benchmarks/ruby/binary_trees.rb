#!/usr/bin/env ruby
# Binary trees benchmark for Ruby

require 'benchmark'

# A simple tree node class
class TreeNode
  attr_accessor :left, :right, :item
  
  def initialize(item, left = nil, right = nil)
    @item = item
    @left = left
    @right = right
  end
  
  # Check a tree by calculating its item sum
  def check
    return @item + (@left ? @left.check : 0) + (@right ? @right.check : 0)
  end
  
  # Create a binary tree of a given depth
  def self.make(depth)
    return TreeNode.new(1) if depth <= 0
    return TreeNode.new(1, TreeNode.make(depth - 1), TreeNode.make(depth - 1))
  end
end

def main
  n = 10  # Default value, can be provided as argument
  min_depth = 4
  max_depth = [min_depth + 2, n].max
  stretch_depth = max_depth + 1
  
  start_time = Time.now
  
  # Create and check a stretch tree
  stretch_tree = TreeNode.make(stretch_depth)
  stretch_check = stretch_tree.check
  puts "stretch tree of depth #{stretch_depth}\t check: #{stretch_check}"
  
  # Create and check a long-lived tree
  long_lived_tree = TreeNode.make(max_depth)
  
  # Create and check multiple trees of increasing depth
  (min_depth..max_depth).step(2) do |depth|
    iterations = 2 ** (max_depth - depth + min_depth)
    check = 0
    
    iterations.times do
      check += TreeNode.make(depth).check
    end
    
    puts "#{iterations}\t trees of depth #{depth}\t check: #{check}"
  end
  
  # Check the long-lived tree at the end
  long_lived_check = long_lived_tree.check
  puts "long lived tree of depth #{max_depth}\t check: #{long_lived_check}"
  
  elapsed = (Time.now - start_time) * 1000
  puts "Time taken: #{elapsed.round} ms"
end

main