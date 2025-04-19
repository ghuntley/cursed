#!/usr/bin/env perl
# FASTA benchmark - generate and write random DNA sequences

use strict;
use warnings;
use Time::HiRes qw(gettimeofday tv_interval);

# Constants for the random number generator
use constant IM => 139968;
use constant IA => 3877;
use constant IC => 29573;
use constant SEED => 42;

# Define DNA sequences
use constant ALU => 'GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA';

# IUB probability and character arrays
my @IUB_PROB = (
    0.27, 0.12, 0.12, 0.27, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02
);

my @IUB_CHAR = (
    'a', 'c', 'g', 't', 'B',
    'D', 'H', 'K', 'M', 'N',
    'R', 'S', 'V', 'W', 'Y'
);

# Homo sapiens probability and character arrays
my @HOMO_SAPIENS_PROB = (
    0.3029549426680, 0.1979883004921,
    0.1975473066391, 0.3015094502008
);

my @HOMO_SAPIENS_CHAR = (
    'a', 'c', 'g', 't'
);

# Generate a random number
sub gen_random {
    my ($seed) = @_;
    my $value = ($$seed * IA + IC) % IM;
    $$seed = $value;
    return $value / IM;
}

# Generate a random FASTA sequence
sub gen_random_fasta {
    my ($n, $seed, $probs, $chars) = @_;
    my $length = scalar @$probs;
    my $buffer = '';
    
    for (my $i = 0; $i < $n; $i++) {
        my $r = gen_random($seed);
        my $c = '?';
        
        for (my $j = 0; $j < $length; $j++) {
            if ($r < $probs->[$j]) {
                $c = $chars->[$j];
                last;
            }
            $r -= $probs->[$j];
        }
        
        $buffer .= $c;
    }
    
    return $buffer;
}

# Repeat a sequence until it reaches the required length
sub repeat_fasta {
    my ($n, $seq) = @_;
    my $seq_len = length($seq);
    my $buffer = '';
    
    for (my $i = 0; $i < $n; $i++) {
        $buffer .= substr($seq, $i % $seq_len, 1);
    }
    
    return $buffer;
}

# Main
my $n = 1000000; # Default sequence length
my $seed = SEED;

my $start_time = [gettimeofday];

# Write FASTA header and sequence for Homo sapiens Alu
print ">ONE Homo sapiens alu\n";
my $alu_seq = repeat_fasta($n, ALU);
print "$alu_seq\n";

# Write FASTA header and random sequence for IUB ambiguity codes
print ">TWO IUB ambiguity codes\n";
my $iub_seq = gen_random_fasta($n, \$seed, \@IUB_PROB, \@IUB_CHAR);
print "$iub_seq\n";

# Write FASTA header and random sequence for Homo sapiens frequency
print ">THREE Homo sapiens frequency\n";
my $sapiens_seq = gen_random_fasta($n, \$seed, \@HOMO_SAPIENS_PROB, \@HOMO_SAPIENS_CHAR);
print "$sapiens_seq\n";

my $elapsed = tv_interval($start_time) * 1000;
printf "Time taken: %.2f ms\n", $elapsed;

# Perl doesn't have a standard way to get memory usage
print "Memory monitoring not available for Perl implementation\n";