-module(fasta).
-export([main/0]).

% Constants for the random number generator
-define(IM, 139968).
-define(IA, 3877).
-define(IC, 29573).
-define(SEED, 42).

% Define DNA sequences
-define(ALU, "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA").

% Initialize a process dictionary for the random seed
init_seed() ->
    put(seed, ?SEED).

% Generate a random number
gen_random() ->
    Seed = get(seed),
    Value = (Seed * ?IA + ?IC) rem ?IM,
    put(seed, Value),
    Value / ?IM.

% Generate a random FASTA sequence
gen_random_fasta(N, Probs, Chars) ->
    gen_random_fasta(N, Probs, Chars, []).

gen_random_fasta(0, _, _, Acc) ->
    lists:reverse(Acc);
gen_random_fasta(N, Probs, Chars, Acc) ->
    R = gen_random(),
    {Char, _} = pick_char(R, Probs, Chars, 0, 0),
    gen_random_fasta(N-1, Probs, Chars, [Char | Acc]).

pick_char(R, [Prob | ProbsRest], [Char | CharsRest], Index, Sum) ->
    NewSum = Sum + Prob,
    case R < NewSum of
        true -> {Char, Index};
        false -> pick_char(R, ProbsRest, CharsRest, Index + 1, NewSum)
    end.

% Repeat a sequence until it reaches the required length
repeat_fasta(N, Seq) ->
    SeqLen = length(Seq),
    repeat_fasta(N, Seq, SeqLen, []).

repeat_fasta(0, _, _, Acc) ->
    lists:reverse(Acc);
repeat_fasta(N, Seq, SeqLen, Acc) ->
    Index = (N - 1) rem SeqLen + 1,
    Char = lists:nth(Index, Seq),
    repeat_fasta(N-1, Seq, SeqLen, [Char | Acc]).

main() ->
    init_seed(),
    N = 1000000,
    StartTime = erlang:system_time(millisecond),
    
    % Write FASTA header and sequence for Homo sapiens Alu
    io:format(">ONE Homo sapiens alu~n"),
    io:format("~s~n", [repeat_fasta(N, ?ALU)]),
    
    % Write FASTA header and random sequence for IUB ambiguity codes
    IUB_PROB = [0.27, 0.12, 0.12, 0.27, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02],
    IUB_CHAR = ["a", "c", "g", "t", "B", "D", "H", "K", "M", "N", "R", "S", "V", "W", "Y"],
    io:format(">TWO IUB ambiguity codes~n"),
    io:format("~s~n", [gen_random_fasta(N, IUB_PROB, IUB_CHAR)]),
    
    % Write FASTA header and random sequence for Homo sapiens frequency
    HOMO_SAPIENS_PROB = [0.3029549426680, 0.1979883004921, 0.1975473066391, 0.3015094502008],
    HOMO_SAPIENS_CHAR = ["a", "c", "g", "t"],
    io:format(">THREE Homo sapiens frequency~n"),
    io:format("~s~n", [gen_random_fasta(N, HOMO_SAPIENS_PROB, HOMO_SAPIENS_CHAR)]),
    
    ElapsedTime = erlang:system_time(millisecond) - StartTime,
    io:format("Time taken: ~p ms~n", [ElapsedTime]),
    
    {ok, [TotalWords, _]} = process_info(self(), memory),
    io:format("Memory used: ~p KB~n", [TotalWords div 1024]),
    ok.