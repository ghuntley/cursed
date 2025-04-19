fr fr FASTA benchmark - generate and write random DNA sequences

yeet "fmt"

fr fr Constants for the random number generator
sus IM normie = 139968
sus IA normie = 3877
sus IC normie = 29573
sus SEED normie = 42

fr fr Define DNA sequences
sus ALU tea = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA"

sus IUB_PROB []meal = []meal{
    0.27, 0.12, 0.12, 0.27, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
}

sus IUB_CHAR []tea = []tea{
    "a", "c", "g", "t", "B",
    "D", "H", "K", "M", "N",
    "R", "S", "V", "W", "Y",
}

sus HOMO_SAPIENS_PROB []meal = []meal{
    0.3029549426680, 0.1979883004921,
    0.1975473066391, 0.3015094502008,
}

sus HOMO_SAPIENS_CHAR []tea = []tea{
    "a", "c", "g", "t",
}

sus BUF_SIZE normie = 1024 * 1024

fr fr Generate a random number
slay gen_random(seed @normie) meal {
    sus value normie = (*seed * IA + IC) % IM
    *seed = value
    yolo meal(value) / meal(IM)
}

fr fr Generate a random FASTA sequence
slay gen_random_fasta(n normie, seed @normie, probs []meal, chars []tea) tea {
    sus length normie = len(probs)
    sus buffer tea = tea.make(n)
    
    bestie i := 0; i < n; i++ {
        sus r meal = gen_random(seed)
        sus c tea = ""
        
        bestie j := 0; j < length; j++ {
            lowkey r < probs[j] {
                c = chars[j]
                ghosted
            }
            r -= probs[j]
        }
        
        buffer = buffer + c
    }
    
    yolo buffer
}

fr fr Repeat a sequence until it reaches the required length
slay repeat_fasta(n normie, seq tea) tea {
    sus seq_len normie = len(seq)
    sus buffer tea = tea.make(n)
    
    bestie i := 0; i < n; i++ {
        buffer = buffer + tea.substring(seq, i % seq_len, i % seq_len + 1)
    }
    
    yolo buffer
}

slay main() {
    sus n normie = 1000000 fr fr Default sequence length
    sus seed normie = SEED
    sus start_ts thicc = timez.now()
    
    fr fr Write FASTA header and sequence for Homo sapiens Alu
    fmt.Println(">ONE Homo sapiens alu")
    sus alu_seq tea = repeat_fasta(n, ALU)
    fmt.Println(alu_seq)
    
    fr fr Write FASTA header and random sequence for IUB ambiguity codes
    fmt.Println(">TWO IUB ambiguity codes")
    sus iub_seq tea = gen_random_fasta(n, &seed, IUB_PROB, IUB_CHAR)
    fmt.Println(iub_seq)
    
    fr fr Write FASTA header and random sequence for Homo sapiens frequency
    fmt.Println(">THREE Homo sapiens frequency")
    sus sapiens_seq tea = gen_random_fasta(n, &seed, HOMO_SAPIENS_PROB, HOMO_SAPIENS_CHAR)
    fmt.Println(sapiens_seq)
    
    sus elapsed thicc = timez.now() - start_ts
    fmt.Println("Time taken:", elapsed, "ms")
    fmt.Println("Memory used:", stats.heap_memory(), "KB")
}