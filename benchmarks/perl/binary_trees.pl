#!/usr/bin/env perl
# Binary trees benchmark adapted from The Computer Language Benchmarks Game

use strict;
use warnings;
use Time::HiRes qw(gettimeofday tv_interval);

package TreeNode;

sub new {
    my ($class, $item) = @_;
    return bless { item => $item, left => undef, right => undef }, $class;
}

# Create a new tree with the given item value at the root
sub create_tree {
    my ($item, $depth) = @_;
    my $node = TreeNode->new($item);
    
    if ($depth > 0) {
        $node->{left} = create_tree(2 * $item - 1, $depth - 1);
        $node->{right} = create_tree(2 * $item, $depth - 1);
    }
    
    return $node;
}

# Check the tree and return a checksum
sub check {
    my ($self) = @_;
    
    if (!defined $self->{left}) {
        return $self->{item};
    }
    
    return $self->{item} + $self->{left}->check() - $self->{right}->check();
}

package main;

my $min_depth = 4;
my $max_depth = 12;

my $stretch_depth = $max_depth + 1;
my $start_time = [gettimeofday];

# Allocate and check a big tree
my $big_tree = TreeNode::create_tree(0, $stretch_depth);
printf "stretch tree of depth %d check: %d\n", $stretch_depth, $big_tree->check();

# Allow the big tree to be garbage collected
$big_tree = undef;

# Allocate a long-lived binary tree
my $long_lived_tree = TreeNode::create_tree(0, $max_depth);

# Check trees of increasing depth
for (my $depth = $min_depth; $depth <= $max_depth; $depth += 2) {
    my $iterations = 1 << ($max_depth - $depth + $min_depth);
    my $result = 0;
    
    for (my $i = 0; $i < $iterations; $i++) {
        my $a = TreeNode::create_tree($i, $depth);
        my $b = TreeNode::create_tree(-$i, $depth);
        $result += $a->check() + $b->check();
    }
    
    printf "%d trees of depth %d check: %d\n", $iterations * 2, $depth, $result;
}

# Check the long-lived tree last
printf "long lived tree of depth %d check: %d\n", $max_depth, $long_lived_tree->check();

my $elapsed = tv_interval($start_time) * 1000;
printf "Time taken: %.2f ms\n", $elapsed;

# Perl doesn't have a standard way to get memory usage
print "Memory monitoring not available for Perl implementation\n";