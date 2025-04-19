#!/usr/bin/env php
<?php
// FASTA benchmark - generate and write random DNA sequences

// Constants for the random number generator
define('IM', 139968);
define('IA', 3877);
define('IC', 29573);
define('SEED', 42);

// Define DNA sequences
define('ALU', 'GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA');

// IUB probability array
$IUB_PROB = array(
    0.27, 0.12, 0.12, 0.27, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02
);

// IUB characters array
$IUB_CHAR = array(
    'a', 'c', 'g', 't', 'B',
    'D', 'H', 'K', 'M', 'N',
    'R', 'S', 'V', 'W', 'Y'
);

// Homo sapiens probability array
$HOMO_SAPIENS_PROB = array(
    0.3029549426680, 0.1979883004921,
    0.1975473066391, 0.3015094502008
);

// Homo sapiens characters array
$HOMO_SAPIENS_CHAR = array(
    'a', 'c', 'g', 't'
);

// Generate a random number
function gen_random(&$seed) {
    $value = ($seed * IA + IC) % IM;
    $seed = $value;
    return $value / IM;
}

// Generate a random FASTA sequence
function gen_random_fasta($n, &$seed, $probs, $chars) {
    $buffer = '';
    $length = count($probs);
    
    for ($i = 0; $i < $n; $i++) {
        $r = gen_random($seed);
        $c = '?';
        
        for ($j = 0; $j < $length; $j++) {
            if ($r < $probs[$j]) {
                $c = $chars[$j];
                break;
            }
            $r -= $probs[$j];
        }
        
        $buffer .= $c;
    }
    
    return $buffer;
}

// Repeat a sequence until it reaches the required length
function repeat_fasta($n, $seq) {
    $seq_len = strlen($seq);
    $buffer = '';
    
    for ($i = 0; $i < $n; $i++) {
        $buffer .= $seq[$i % $seq_len];
    }
    
    return $buffer;
}

function main() {
    global $IUB_PROB, $IUB_CHAR, $HOMO_SAPIENS_PROB, $HOMO_SAPIENS_CHAR;
    
    $n = 1000000; // Default sequence length
    $seed = SEED;
    $start_time = microtime(true);
    
    // Write FASTA header and sequence for Homo sapiens Alu
    echo ">ONE Homo sapiens alu\n";
    $alu_seq = repeat_fasta($n, ALU);
    echo "$alu_seq\n";
    
    // Write FASTA header and random sequence for IUB ambiguity codes
    echo ">TWO IUB ambiguity codes\n";
    $iub_seq = gen_random_fasta($n, $seed, $IUB_PROB, $IUB_CHAR);
    echo "$iub_seq\n";
    
    // Write FASTA header and random sequence for Homo sapiens frequency
    echo ">THREE Homo sapiens frequency\n";
    $sapiens_seq = gen_random_fasta($n, $seed, $HOMO_SAPIENS_PROB, $HOMO_SAPIENS_CHAR);
    echo "$sapiens_seq\n";
    
    $elapsed = (microtime(true) - $start_time) * 1000;
    echo "Time taken: $elapsed ms\n";
    
    // Get memory stats
    $memoryUsed = memory_get_peak_usage(true) / 1024;
    echo "Memory used: $memoryUsed KB\n";
}

main();