// Fannkuch redux benchmark

use std::time::Instant;

// Reverse the first n elements of the array
fn flip(p: &mut [usize], n: usize) {
    for i in 0..(n/2) {
        p.swap(i, n-i-1);
    }
}

// Count flips required to flip elements to get back to original order
fn fannkuch(n: usize) -> usize {
    let mut p = Vec::with_capacity(n);
    let mut perm = vec![0; n];
    let mut count = vec![0; n];
    let mut max_flips = 0;
    let mut checksum = 0;
    
    // Initialize permutation
    for i in 0..n {
        p.push(i);
    }
    
    let mut perm_count = 0;
    let mut sign = 1;
    
    loop {
        // Copy permutation to perm
        for i in 0..n {
            perm[i] = p[i] + 1;
        }
        
        let first = p[0];
        if first != 0 {
            // Count flips
            for i in 0..n {
                count[i] = 0;
            }
            
            let mut flips = 0;
            let mut k;
            while perm[0] != 1 {
                k = perm[0] - 1;
                flip(&mut perm, k);
                flips += 1;
                perm[0] = k + 1;
            }
            
            if flips > max_flips {
                max_flips = flips;
            }
            
            checksum += sign * flips as isize;
        }
        
        // Generate next permutation
        sign = -sign;
        let mut j = 1;
        while j < n && p[j-1] >= p[j] {
            j += 1;
        }
        
        if j == n {
            break;
        }
        
        perm_count += 1;
        
        let first_j = p[j];
        for i in 0..j {
            if i % 2 == 0 {
                p.swap(i, j-i);
            } else {
                p.swap(i, j-i-1);
            }
        }
        
        if j < 2 {
            let mut j = 1;
            for i in 1..n {
                if p[i-1] > p[i] {
                    j = i + 1;
                }
            }
            
            for i in 0..(j-1) {
                let mut k = i;
                let temp = p[i];
                while k < j-1 {
                    k += 1;
                    p[k-1] = p[k];
                }
                p[j-1] = temp;
            }
        } else {
            let j_idx = j;
            j -= 1;
            let first_j = p[j];
            for i in (1..=j).rev() {
                p[i] = p[i-1];
            }
            p[0] = first_j;
        }
        
        if perm_count >= 10000 {
            break;
        }
    }
    
    max_flips
}

fn main() {
    let n = 10;
    let start_time = Instant::now();
    
    let result = fannkuch(n);
    
    println!("Fannkuch({}): {}", n, result);
    
    let elapsed = start_time.elapsed();
    println!("Time taken: {} ms", elapsed.as_millis());
    
    // Get approximate memory usage
    let memory_usage = std::mem::size_of::<Vec<usize>>() * 3 + n * std::mem::size_of::<usize>() * 3;
    println!("Memory used: {} KB", memory_usage / 1024);
}