// Fannkuch redux benchmark

// Reverse the first n elements of the array
function flip(p, n) {
    for (let i = 0; i < Math.floor(n / 2); i++) {
        const temp = p[i];
        p[i] = p[n - i - 1];
        p[n - i - 1] = temp;
    }
}

// Count flips required to flip elements to get back to original order
function fannkuch(n) {
    const p = new Array(n);
    const perm = new Array(n);
    const count = new Array(n);
    let maxFlips = 0;
    let checksum = 0;
    
    // Initialize permutation
    for (let i = 0; i < n; i++) {
        p[i] = i;
    }
    
    let permCount = 0;
    let sign = 1;
    
    while (true) {
        // Copy permutation to perm
        for (let i = 0; i < n; i++) {
            perm[i] = p[i] + 1;
        }
        
        const first = p[0];
        if (first !== 0) {
            // Count flips
            for (let i = 0; i < n; i++) {
                count[i] = 0;
            }
            
            let flips = 0;
            while (perm[0] !== 1) {
                const k = perm[0] - 1;
                flip(perm, k);
                flips++;
                perm[0] = k + 1;
            }
            
            if (flips > maxFlips) {
                maxFlips = flips;
            }
            
            checksum += sign * flips;
        }
        
        // Generate next permutation
        sign = -sign;
        let j = 1;
        while (j < n && p[j - 1] >= p[j]) {
            j++;
        }
        
        if (j === n) {
            break;
        }
        
        permCount++;
        
        const firstJ = p[j];
        for (let i = 0; i < j; i++) {
            if (i % 2 === 0) {
                const temp = p[i];
                p[i] = p[j - i];
                p[j - i] = temp;
            } else {
                const temp = p[i];
                p[i] = p[j - i - 1];
                p[j - i - 1] = temp;
            }
        }
        
        if (j < 2) {
            j = 1;
            for (let i = 1; i < n; i++) {
                if (p[i - 1] > p[i]) {
                    j = i + 1;
                }
            }
            
            for (let i = 0; i < j - 1; i++) {
                let k = i;
                const temp = p[i];
                while (k < j - 1) {
                    k++;
                    p[k - 1] = p[k];
                }
                p[j - 1] = temp;
            }
        } else {
            j--;
            const firstJ = p[j];
            for (let i = j; i > 0; i--) {
                p[i] = p[i - 1];
            }
            p[0] = firstJ;
        }
        
        if (permCount >= 10000) {
            break;
        }
    }
    
    return maxFlips;
}

function main() {
    const n = 10;
    const startTime = Date.now();
    
    const result = fannkuch(n);
    
    console.log(`Fannkuch(${n}): ${result}`);
    
    const elapsed = Date.now() - startTime;
    console.log(`Time taken: ${elapsed} ms`);
    
    // Get memory stats
    const memoryUsageInMB = process.memoryUsage().heapUsed / 1024 / 1024;
    console.log(`Memory used: ${Math.round(memoryUsageInMB * 1024)} KB`);
}

main();