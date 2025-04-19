#!/usr/bin/env perl
# Mandelbrot set calculation benchmark

use strict;
use warnings;
use Time::HiRes qw(gettimeofday tv_interval);

# Size constants
use constant WIDTH => 800;
use constant HEIGHT => 800;
use constant MAX_ITERATIONS => 100;

# Calculate the Mandelbrot set
sub calculate_mandelbrot {
    my ($max_iterations) = @_;
    my @result;
    
    for my $y (0..HEIGHT-1) {
        for my $x (0..WIDTH-1) {
            my $cx = ($x - WIDTH / 2.0) * 4.0 / WIDTH;
            my $cy = ($y - HEIGHT / 2.0) * 4.0 / HEIGHT;
            
            my $zx = 0.0;
            my $zy = 0.0;
            my $iteration = 0;
            
            while ($zx*$zx + $zy*$zy <= 4.0 && $iteration < $max_iterations) {
                my $temp = $zx*$zx - $zy*$zy + $cx;
                $zy = 2.0 * $zx * $zy + $cy;
                $zx = $temp;
                $iteration++;
            }
            
            $result[$y][$x] = $iteration;
        }
    }
    
    return \@result;
}

# Count non-black pixels
sub count_non_black {
    my ($result, $max_iterations) = @_;
    my $count = 0;
    
    for my $y (0..HEIGHT-1) {
        for my $x (0..WIDTH-1) {
            if ($result->[$y][$x] < $max_iterations) {
                $count++;
            }
        }
    }
    
    return $count;
}

# Main
my $start_time = [gettimeofday];

my $result = calculate_mandelbrot(MAX_ITERATIONS);
my $count = count_non_black($result, MAX_ITERATIONS);

print "Mandelbrot set calculation finished.\n";
printf "Image size: %d x %d\n", WIDTH, HEIGHT;
printf "Maximum iterations: %d\n", MAX_ITERATIONS;
printf "Non-black pixels: %d\n", $count;

my $elapsed = tv_interval($start_time) * 1000;
printf "Time taken: %.2f ms\n", $elapsed;

# Perl doesn't have a standard way to get memory usage
print "Memory monitoring not available for Perl implementation\n";