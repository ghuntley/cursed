#!/usr/bin/env perl
# String processing benchmark

use strict;
use warnings;
use Time::HiRes qw(gettimeofday tv_interval);

# Process strings of various sizes
sub process_strings {
    my ($count, $size) = @_;
    my $result = "";
    
    for (my $i = 0; $i < $count; $i++) {
        my $str = create_random_string($size);
        my $processed = process_string($str);
        $result .= $processed;
    }
    
    return $result;
}

# Create a random string of given size
sub create_random_string {
    my ($size) = @_;
    my $chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    my $result = "";
    
    for (my $i = 0; $i < $size; $i++) {
        my $idx = int(rand(length($chars)));
        $result .= substr($chars, $idx, 1);
    }
    
    return $result;
}

# Process a string according to the rules
sub process_string {
    my ($input) = @_;
    my $result = $input;
    
    # Replace all vowels with their uppercase version
    $result =~ s/a/A/g;
    $result =~ s/e/E/g;
    $result =~ s/i/I/g;
    $result =~ s/o/O/g;
    $result =~ s/u/U/g;
    
    # Replace all digits with their doubled value
    for my $i (0..9) {
        my $digit = "$i";
        my $doubled = "" . ($i * 2);
        $result =~ s/$digit/$doubled/g;
    }
    
    # Capitalize the first letter
    if (length($result) > 0) {
        substr($result, 0, 1) = uc(substr($result, 0, 1));
    }
    
    # Reverse the string
    $result = reverse($result);
    
    # Take the first half of the reversed string
    my $half_len = int(length($result) / 2);
    $result = substr($result, 0, $half_len);
    
    return $result;
}

# Main
my $start_time = [gettimeofday];

# Process strings of different sizes
my $small = process_strings(10000, 10);   # 10,000 strings of length 10
my $medium = process_strings(1000, 100);  # 1,000 strings of length 100
my $large = process_strings(100, 1000);   # 100 strings of length 1,000

my $result_length = length($small) + length($medium) + length($large);
print "Processed string length: $result_length\n";

my $elapsed = tv_interval($start_time) * 1000;
printf "Time taken: %.2f ms\n", $elapsed;

# Perl doesn't have a standard way to get memory usage
print "Memory monitoring not available for Perl implementation\n";