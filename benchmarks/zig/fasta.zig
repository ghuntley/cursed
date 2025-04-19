// FASTA benchmark for Zig

const std = @import("std");

// Constants for the random number generator
const IM: i32 = 139968;
const IA: i32 = 3877;
const IC: i32 = 29573;
var SEED: i32 = 42;

// Define DNA sequences
const ALU = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA";

const IUB_PROB = [_]f64{
    0.27, 0.12, 0.12, 0.27, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
};

const IUB_CHAR = [_]u8{
    'a', 'c', 'g', 't', 'B',
    'D', 'H', 'K', 'M', 'N',
    'R', 'S', 'V', 'W', 'Y',
};

const HOMO_SAPIENS_PROB = [_]f64{
    0.3029549426680, 0.1979883004921,
    0.1975473066391, 0.3015094502008,
};

const HOMO_SAPIENS_CHAR = [_]u8{
    'a', 'c', 'g', 't',
};

// Generate a random number
fn genRandom(seed: *i32) f64 {
    const value = (seed.* * IA + IC) % IM;
    seed.* = value;
    return @intToFloat(f64, value) / @intToFloat(f64, IM);
}

// Generate a random FASTA sequence
fn genRandomFasta(alloc: std.mem.Allocator, n: usize, seed: *i32, probs: []const f64, chars: []const u8) ![]u8 {
    const length = probs.len;
    var buffer = try alloc.alloc(u8, n);
    
    for (buffer) |*c, i| {
        var r = genRandom(seed);
        
        for (probs) |prob, j| {
            if (r < prob) {
                c.* = chars[j];
                break;
            }
            r -= prob;
        }
    }
    
    return buffer;
}

// Repeat a sequence until it reaches the required length
fn repeatFasta(alloc: std.mem.Allocator, n: usize, seq: []const u8) ![]u8 {
    const seqLen = seq.len;
    var buffer = try alloc.alloc(u8, n);
    
    for (buffer) |*c, i| {
        c.* = seq[i % seqLen];
    }
    
    return buffer;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};  
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();
    
    const n: usize = 1_000_000; // Default sequence length
    var seed = SEED;
    
    const start_time = std.time.milliTimestamp();
    
    const stdout = std.io.getStdOut().writer();
    
    // Write FASTA header and sequence for Homo sapiens Alu
    try stdout.print(">ONE Homo sapiens alu\n", .{});
    const aluSeq = try repeatFasta(allocator, n, ALU);
    defer allocator.free(aluSeq);
    try stdout.print("{s}\n", .{aluSeq});
    
    // Write FASTA header and random sequence for IUB ambiguity codes
    try stdout.print(">TWO IUB ambiguity codes\n", .{});
    const iubSeq = try genRandomFasta(allocator, n, &seed, &IUB_PROB, &IUB_CHAR);
    defer allocator.free(iubSeq);
    try stdout.print("{s}\n", .{iubSeq});
    
    // Write FASTA header and random sequence for Homo sapiens frequency
    try stdout.print(">THREE Homo sapiens frequency\n", .{});
    const sapiensSeq = try genRandomFasta(allocator, n, &seed, &HOMO_SAPIENS_PROB, &HOMO_SAPIENS_CHAR);
    defer allocator.free(sapiensSeq);
    try stdout.print("{s}\n", .{sapiensSeq});
    
    const elapsed = std.time.milliTimestamp() - start_time;
    try stdout.print("Time taken: {} ms\n", .{elapsed});
    
    // Calculate approximate memory usage
    const memoryUsage = aluSeq.len + iubSeq.len + sapiensSeq.len;
    try stdout.print("Memory used: {} KB\n", .{memoryUsage / 1024});
}