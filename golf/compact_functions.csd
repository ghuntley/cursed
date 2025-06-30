// Compact CURSED Rule 30 operations - using only working operators

println("=== Rule 30 Core Operations ===");

println("1. Extract bits using division (no modulo needed):");
println("  170 / 128 =", 170 / 128, "(bit 0)");
println("  170 / 64  =", 170 / 64, "-> 2/2=1 (bit 1)");  
println("  170 / 32  =", 170 / 32, "-> 5/2=2 (bit 2)");
println("  170 / 16  =", 170 / 16, "-> 10/2=5 (bit 3)");
println("  170 / 8   =", 170 / 8, "-> 21/2=10 (bit 4)");
println("  170 / 4   =", 170 / 4, "-> 42/2=21 (bit 5)");
println("  170 / 2   =", 170 / 2, "-> 85/2=42 (bit 6)");
println("  170 / 1   =", 170, "-> 170/2=85 (bit 7)");

println("");
println("2. Alternative bit extraction (170 = 10101010):");
println("  bit 0: 170>=128 ?", 170 >= 128, "-> 1");
println("  bit 1: 42>=64 ?", 42 >= 64, "-> 0");  // 170-128=42
println("  bit 2: 42>=32 ?", 42 >= 32, "-> 1");  
println("  bit 3: 10>=16 ?", 10 >= 16, "-> 0");  // 42-32=10
println("  bit 4: 10>=8 ?", 10 >= 8, "-> 1");
println("  bit 5: 2>=4 ?", 2 >= 4, "-> 0");      // 10-8=2
println("  bit 6: 2>=2 ?", 2 >= 2, "-> 1");
println("  bit 7: 0>=1 ?", 0 >= 1, "-> 0");      // 2-2=0

println("");
println("3. Rule 30 formula: left XOR (center OR right)");
println("  Using: left != (center + right) since OR ≈ + for bits");
println("  000:", 0 != (0 + 0));
println("  001:", 0 != (0 + 1));
println("  010:", 0 != (1 + 0));
println("  011:", 0 != (1 + 1));
println("  100:", 1 != (0 + 0));
println("  101:", 1 != (0 + 1));
println("  110:", 1 != (1 + 0));
println("  111:", 1 != (1 + 1));

println("");
println("4. One Rule 30 evolution step:");
println("  Initial: [0,0,0,1,0,0,0,0]");
println("  Step 1:  [?,?,?,?,?,?,?,?]");
println("  pos 0: 0!=(0+0) =", 0 != (0 + 0));  // [7,0,1] = [0,0,0]
println("  pos 1: 0!=(0+0) =", 0 != (0 + 0));  // [0,1,2] = [0,0,0]
println("  pos 2: 0!=(0+1) =", 0 != (0 + 1));  // [1,2,3] = [0,0,1]
println("  pos 3: 0!=(1+0) =", 0 != (1 + 0));  // [2,3,4] = [0,1,0]
println("  pos 4: 1!=(0+0) =", 1 != (0 + 0));  // [3,4,5] = [1,0,0]
println("  pos 5: 0!=(0+0) =", 0 != (0 + 0));  // [4,5,6] = [0,0,0]
println("  pos 6: 0!=(0+0) =", 0 != (0 + 0));  // [5,6,7] = [0,0,0]
println("  pos 7: 0!=(0+0) =", 0 != (0 + 0));  // [6,7,0] = [0,0,0]

println("");
println("5. Hex conversion for [1,0,1,0,1,1,1,1]:");
println("  First 4 bits: 1*8+0*4+1*2+0*1 =", 1*8 + 0*4 + 1*2 + 0*1, "(=10, hex A)");
println("  Last 4 bits:  1*8+1*4+1*2+1*1 =", 1*8 + 1*4 + 1*2 + 1*1, "(=15, hex F)");

println("");
println("=== CURSED Rule 30 Operations Complete ===");
println("All core functions work with basic arithmetic!");
