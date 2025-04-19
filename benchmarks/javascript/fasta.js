// FASTA benchmark - generate and write random DNA sequences

// Constants for the random number generator
const IM = 139968;
const IA = 3877;
const IC = 29573;
const SEED = 42;

// Define DNA sequences
const ALU = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA";

const IUB_PROB = [
    0.27, 0.12, 0.12, 0.27, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
];

const IUB_CHAR = [
    'a', 'c', 'g', 't', 'B',
    'D', 'H', 'K', 'M', 'N',
    'R', 'S', 'V', 'W', 'Y',
];

const HOMO_SAPIENS_PROB = [
    0.3029549426680, 0.1979883004921,
    0.1975473066391, 0.3015094502008,
];

const HOMO_SAPIENS_CHAR = [
    'a', 'c', 'g', 't',
];

const BUF_SIZE = 1024 * 1024;

// Generate a random number
function genRandom(seed) {
    const value = (seed[0] * IA + IC) % IM;
    seed[0] = value;
    return value / IM;
}

// Generate a random FASTA sequence
function genRandomFasta(n, seed, probs, chars) {
    const length = probs.length;
    let buffer = '';
    
    for (let i = 0; i < n; i++) {
        let r = genRandom(seed);
        let c = ' ';
        
        for (let j = 0; j < length; j++) {
            if (r < probs[j]) {
                c = chars[j];
                break;
            }
            r -= probs[j];
        }
        
        buffer += c;
    }
    
    return buffer;
}

// Repeat a sequence until it reaches the required length
function repeatFasta(n, seq) {
    const seqLen = seq.length;
    let buffer = '';
    
    for (let i = 0; i < n; i++) {
        buffer += seq[i % seqLen];
    }
    
    return buffer;
}

function main() {
    const n = 1000000; // Default sequence length
    const seed = [SEED];
    const startTime = Date.now();
    
    // Write FASTA header and sequence for Homo sapiens Alu
    console.log(">ONE Homo sapiens alu");
    const aluSeq = repeatFasta(n, ALU);
    console.log(aluSeq);
    
    // Write FASTA header and random sequence for IUB ambiguity codes
    console.log(">TWO IUB ambiguity codes");
    const iubSeq = genRandomFasta(n, seed, IUB_PROB, IUB_CHAR);
    console.log(iubSeq);
    
    // Write FASTA header and random sequence for Homo sapiens frequency
    console.log(">THREE Homo sapiens frequency");
    const sapiensSeq = genRandomFasta(n, seed, HOMO_SAPIENS_PROB, HOMO_SAPIENS_CHAR);
    console.log(sapiensSeq);
    
    const elapsed = Date.now() - startTime;
    console.log(`Time taken: ${elapsed} ms`);
    
    // Get memory stats
    const memoryUsageInMB = process.memoryUsage().heapUsed / 1024 / 1024;
    console.log(`Memory used: ${Math.round(memoryUsageInMB * 1024)} KB`);
}

main();