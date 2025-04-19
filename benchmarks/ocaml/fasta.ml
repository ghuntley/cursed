(* FASTA benchmark - generate and write random DNA sequences *)

(* Constants for the random number generator *)
let im = 139968
let ia = 3877
let ic = 29573
let seed = ref 42

(* Define DNA sequences *)
let alu = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA"

let iub_prob = [|
  0.27; 0.12; 0.12; 0.27; 0.02;
  0.02; 0.02; 0.02; 0.02; 0.02;
  0.02; 0.02; 0.02; 0.02; 0.02
|]

let iub_char = [|
  "a"; "c"; "g"; "t"; "B";
  "D"; "H"; "K"; "M"; "N";
  "R"; "S"; "V"; "W"; "Y";
|]

let homo_sapiens_prob = [|
  0.3029549426680; 0.1979883004921;
  0.1975473066391; 0.3015094502008;
|]

let homo_sapiens_char = [|
  "a"; "c"; "g"; "t";
|]

(* Generate a random number *)
let gen_random () =
  seed := (!seed * ia + ic) mod im;
  float_of_int !seed /. float_of_int im

(* Generate a random FASTA sequence *)
let gen_random_fasta n probs chars =
  let length = Array.length probs in
  let buf = Buffer.create n in
  
  for i = 1 to n do
    let r = ref (gen_random ()) in
    let j = ref 0 in
    
    while !j < length && !r >= probs.(!j) do
      r := !r -. probs.(!j);
      j := !j + 1
    done;
    
    Buffer.add_string buf chars.(!j)
  done;
  
  Buffer.contents buf

(* Repeat a sequence until it reaches the required length *)
let repeat_fasta n seq =
  let seq_len = String.length seq in
  let buf = Buffer.create n in
  
  for i = 0 to n - 1 do
    Buffer.add_char buf seq.[i mod seq_len]
  done;
  
  Buffer.contents buf

let () =
  let n = 1000000 in
  let start_time = Unix.gettimeofday () in
  
  (* Write FASTA header and sequence for Homo sapiens Alu *)
  Printf.printf ">ONE Homo sapiens alu\n";
  let alu_seq = repeat_fasta n alu in
  Printf.printf "%s\n" alu_seq;
  
  (* Write FASTA header and random sequence for IUB ambiguity codes *)
  Printf.printf ">TWO IUB ambiguity codes\n";
  let iub_seq = gen_random_fasta n iub_prob iub_char in
  Printf.printf "%s\n" iub_seq;
  
  (* Write FASTA header and random sequence for Homo sapiens frequency *)
  Printf.printf ">THREE Homo sapiens frequency\n";
  let sapiens_seq = gen_random_fasta n homo_sapiens_prob homo_sapiens_char in
  Printf.printf "%s\n" sapiens_seq;
  
  let elapsed = int_of_float ((Unix.gettimeofday () -. start_time) *. 1000.0) in
  Printf.printf "Time taken: %d ms\n" elapsed;
  
  (* Approximate memory usage *)
  let memory_usage = Gc.stat () in
  Printf.printf "Memory used: %d KB\n" (memory_usage.minor_words |> int_of_float |> (fun x -> x / 1024))