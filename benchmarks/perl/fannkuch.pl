#!/usr/bin/env perl
# Fannkuch redux benchmark

use strict;
use warnings;
use Time::HiRes qw(gettimeofday tv_interval);

# Reverse the first n elements of the array
sub flip {
    my ($p, $n) = @_;
    for (my $i = 0; $i < $n/2; $i++) {
        my $temp = $p->[$i];
        $p->[$i] = $p->[$n-$i-1];
        $p->[$n-$i-1] = $temp;
    }
}

# Count flips required to flip elements to get back to original order
sub fannkuch {
    my ($n) = @_;
    my @p = (0..$n-1);
    my @perm = (0) x $n;
    my @count = (0) x $n;
    my $maxFlips = 0;
    my $checksum = 0;
    
    my $permCount = 0;
    my $sign = 1;
    
    while (1) {
        # Copy permutation to perm
        for (my $i = 0; $i < $n; $i++) {
            $perm[$i] = $p[$i] + 1;
        }
        
        my $first = $p[0];
        if ($first != 0) {
            # Count flips
            @count = (0) x $n;
            
            my $flips = 0;
            while ($perm[0] != 1) {
                my $k = $perm[0] - 1;
                flip(\@perm, $k);
                $flips++;
                $perm[0] = $k + 1;
            }
            
            if ($flips > $maxFlips) {
                $maxFlips = $flips;
            }
            
            $checksum += $sign * $flips;
        }
        
        # Generate next permutation
        $sign = -$sign;
        my $j = 1;
        while ($j < $n && $p[$j-1] >= $p[$j]) {
            $j++;
        }
        
        if ($j == $n) {
            last;
        }
        
        $permCount++;
        
        my $firstJ = $p[$j];
        for (my $i = 0; $i < $j; $i++) {
            if ($i % 2 == 0) {
                my $temp = $p[$i];
                $p[$i] = $p[$j-$i];
                $p[$j-$i] = $temp;
            } else {
                my $temp = $p[$i];
                $p[$i] = $p[$j-$i-1];
                $p[$j-$i-1] = $temp;
            }
        }
        
        if ($j < 2) {
            $j = 1;
            for (my $i = 1; $i < $n; $i++) {
                if ($p[$i-1] > $p[$i]) {
                    $j = $i + 1;
                }
            }
            for (my $i = 0; $i < $j-1; $i++) {
                my $k = $i;
                my $temp = $p[$i];
                while ($k < $j-1) {
                    $k++;
                    $p[$k-1] = $p[$k];
                }
                $p[$j-1] = $temp;
            }
        } else {
            $j--;
            $firstJ = $p[$j];
            for (my $i = $j; $i > 0; $i--) {
                $p[$i] = $p[$i-1];
            }
            $p[0] = $firstJ;
        }
        
        if ($permCount >= 10000) {
            last;
        }
    }
    
    return $maxFlips;
}

# Main
my $n = 10; # Default size
my $start_time = [gettimeofday];

my $result = fannkuch($n);

printf "Fannkuch(%d): %d\n", $n, $result;

my $elapsed = tv_interval($start_time) * 1000;
printf "Time taken: %.2f ms\n", $elapsed;

# Perl doesn't have a standard way to get memory usage
print "Memory monitoring not available for Perl implementation\n";