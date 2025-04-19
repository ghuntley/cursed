program Fasta;

{$mode objfpc}{$H+}

uses
  SysUtils, DateUtils;

const
  (* Constants for the random number generator *)
  IM = 139968;
  IA = 3877;
  IC = 29573;
  
  (* Define DNA sequences *)
  ALU = 'GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA';

type
  TCharArray = array of Char;
  TRealArray = array of Real;

var
  seed: LongInt = 42;
  startTime: TDateTime;
  iubProb: TRealArray;
  iubChar: TCharArray;
  homoSapiensProb: TRealArray;
  homoSapiensChar: TCharArray;

(* Generate a random number *)
function GenRandom: Real;
begin
  seed := (seed * IA + IC) mod IM;
  GenRandom := seed / IM;
end;

(* Generate a random FASTA sequence *)
function GenRandomFasta(n: LongInt; prob: TRealArray; chars: TCharArray): string;
var
  i, j, length: LongInt;
  r, p: Real;
  result: string;
  c: Char;
begin
  length := Length(prob);
  SetLength(result, n);
  
  for i := 1 to n do
  begin
    r := GenRandom;
    p := 0.0;
    
    for j := 0 to length - 1 do
    begin
      p := p + prob[j];
      if r < p then
      begin
        c := chars[j];
        break;
      end;
    end;
    
    result[i] := c;
  end;
  
  GenRandomFasta := result;
end;

(* Repeat a sequence until it reaches the required length *)
function RepeatFasta(n: LongInt; seq: string): string;
var
  i, seqLen: LongInt;
  result: string;
begin
  seqLen := Length(seq);
  SetLength(result, n);
  
  for i := 1 to n do
    result[i] := seq[((i - 1) mod seqLen) + 1];
  
  RepeatFasta := result;
end;

begin
  (* Initialize arrays *)
  SetLength(iubProb, 15);
  SetLength(iubChar, 15);
  
  iubProb[0] := 0.27; iubProb[1] := 0.12; iubProb[2] := 0.12; iubProb[3] := 0.27; iubProb[4] := 0.02;
  iubProb[5] := 0.02; iubProb[6] := 0.02; iubProb[7] := 0.02; iubProb[8] := 0.02; iubProb[9] := 0.02;
  iubProb[10] := 0.02; iubProb[11] := 0.02; iubProb[12] := 0.02; iubProb[13] := 0.02; iubProb[14] := 0.02;
  
  iubChar[0] := 'a'; iubChar[1] := 'c'; iubChar[2] := 'g'; iubChar[3] := 't'; iubChar[4] := 'B';
  iubChar[5] := 'D'; iubChar[6] := 'H'; iubChar[7] := 'K'; iubChar[8] := 'M'; iubChar[9] := 'N';
  iubChar[10] := 'R'; iubChar[11] := 'S'; iubChar[12] := 'V'; iubChar[13] := 'W'; iubChar[14] := 'Y';
  
  SetLength(homoSapiensProb, 4);
  SetLength(homoSapiensChar, 4);
  
  homoSapiensProb[0] := 0.3029549426680; homoSapiensProb[1] := 0.1979883004921;
  homoSapiensProb[2] := 0.1975473066391; homoSapiensProb[3] := 0.3015094502008;
  
  homoSapiensChar[0] := 'a'; homoSapiensChar[1] := 'c'; homoSapiensChar[2] := 'g'; homoSapiensChar[3] := 't';
  
  (* Start timing *)
  startTime := Now;
  
  (* Write FASTA header and sequence for Homo sapiens Alu *)
  WriteLn('>ONE Homo sapiens alu');
  WriteLn(RepeatFasta(1000000, ALU));
  
  (* Write FASTA header and random sequence for IUB ambiguity codes *)
  WriteLn('>TWO IUB ambiguity codes');
  WriteLn(GenRandomFasta(1000000, iubProb, iubChar));
  
  (* Write FASTA header and random sequence for Homo sapiens frequency *)
  WriteLn('>THREE Homo sapiens frequency');
  WriteLn(GenRandomFasta(1000000, homoSapiensProb, homoSapiensChar));
  
  (* Report timing *)
  WriteLn('Time taken: ', MilliSecondsBetween(Now, startTime), ' ms');
  
  (* Approximate memory usage - this is a simplification *)
  WriteLn('Memory used: ', GetHeapStatus.TotalAllocated div 1024, ' KB');
end.