#!/usr/bin/env php
<?php
// Fannkuch redux benchmark

// Reverse the first n elements of the array
function flip(&$p, $n) {
    for ($i = 0; $i < $n/2; $i++) {
        $temp = $p[$i];
        $p[$i] = $p[$n-$i-1];
        $p[$n-$i-1] = $temp;
    }
}

// Count flips required to flip elements to get back to original order
function fannkuch($n) {
    $p = array();
    $perm = array();
    $count = array();
    $maxFlips = 0;
    $checksum = 0;
    
    // Initialize permutation
    for ($i = 0; $i < $n; $i++) {
        $p[$i] = $i;
    }
    
    $permCount = 0;
    $sign = 1;
    
    while (true) {
        // Copy permutation to perm
        for ($i = 0; $i < $n; $i++) {
            $perm[$i] = $p[$i] + 1;
        }
        
        $first = $p[0];
        if ($first != 0) {
            // Count flips
            for ($i = 0; $i < $n; $i++) {
                $count[$i] = 0;
            }
            
            $flips = 0;
            while ($perm[0] != 1) {
                $k = $perm[0] - 1;
                flip($perm, $k);
                $flips++;
                $perm[0] = $k + 1;
            }
            
            if ($flips > $maxFlips) {
                $maxFlips = $flips;
            }
            
            $checksum += $sign * $flips;
        }
        
        // Generate next permutation
        $sign = -$sign;
        $j = 1;
        while ($j < $n && $p[$j-1] >= $p[$j]) {
            $j++;
        }
        
        if ($j == $n) {
            break;
        }
        
        $permCount++;
        
        $firstJ = $p[$j];
        for ($i = 0; $i < $j; $i++) {
            if ($i % 2 == 0) {
                $temp = $p[$i];
                $p[$i] = $p[$j-$i];
                $p[$j-$i] = $temp;
            } else {
                $temp = $p[$i];
                $p[$i] = $p[$j-$i-1];
                $p[$j-$i-1] = $temp;
            }
        }
        
        if ($j < 2) {
            $j = 1;
            for ($i = 1; $i < $n; $i++) {
                if ($p[$i-1] > $p[$i]) {
                    $j = $i + 1;
                }
            }
            
            for ($i = 0; $i < $j-1; $i++) {
                $k = $i;
                $temp = $p[$i];
                while ($k < $j-1) {
                    $k++;
                    $p[$k-1] = $p[$k];
                }
                $p[$j-1] = $temp;
            }
        } else {
            $j--;
            $firstJ = $p[$j];
            for ($i = $j; $i > 0; $i--) {
                $p[$i] = $p[$i-1];
            }
            $p[0] = $firstJ;
        }
        
        if ($permCount >= 10000) {
            break;
        }
    }
    
    return $maxFlips;
}

function main() {
    $n = 10; // Default size
    $start_time = microtime(true);
    
    $result = fannkuch($n);
    
    echo "Fannkuch($n): $result\n";
    
    $elapsed = (microtime(true) - $start_time) * 1000;
    echo "Time taken: $elapsed ms\n";
    
    // Get memory stats
    $memoryUsed = memory_get_peak_usage(true) / 1024;
    echo "Memory used: $memoryUsed KB\n";
}

main();