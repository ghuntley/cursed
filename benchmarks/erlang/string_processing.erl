-module(string_processing).
-export([main/0]).

process_strings(Count, Size) ->
    process_strings(Count, Size, "").

process_strings(0, _, Result) ->
    Result;
process_strings(Count, Size, Result) ->
    Str = create_random_string(Size),
    Processed = process_string(Str),
    process_strings(Count - 1, Size, Result ++ Processed).

create_random_string(Size) ->
    Chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
    CharsLength = length(Chars),
    create_random_string(Size, Chars, CharsLength, []).

create_random_string(0, _, _, Result) ->
    lists:reverse(Result);
create_random_string(Size, Chars, CharsLength, Result) ->
    RandomIndex = rand:uniform(CharsLength) - 1,
    RandomChar = lists:nth(RandomIndex + 1, Chars),
    create_random_string(Size - 1, Chars, CharsLength, [RandomChar | Result]).

process_string(Input) ->
    % Replace all vowels with their uppercase version
    Result1 = lists:foldl(
        fun({From, To}, Acc) ->
            re:replace(Acc, From, To, [global, {return, list}])
        end,
        Input,
        [{"a", "A"}, {"e", "E"}, {"i", "I"}, {"o", "O"}, {"u", "U"}]
    ),
    
    % Replace all digits with their doubled value
    Result2 = lists:foldl(
        fun(I, Acc) ->
            Digit = integer_to_list(I),
            Doubled = integer_to_list(I * 2),
            re:replace(Acc, Digit, Doubled, [global, {return, list}])
        end,
        Result1,
        lists:seq(0, 9)
    ),
    
    % Capitalize the first letter
    Result3 = case Result2 of
        [] -> [];
        [First | Rest] ->
            [string:to_upper(First) | Rest]
    end,
    
    % Reverse the string
    Reversed = lists:reverse(Result3),
    
    % Take the first half of the reversed string
    HalfLen = length(Reversed) div 2,
    string:slice(Reversed, 0, HalfLen).

main() ->
    % Seed the random number generator
    rand:seed(exsss, {erlang:monotonic_time(), erlang:time_offset(), erlang:unique_integer()}),
    StartTime = erlang:system_time(millisecond),
    
    % Process strings of different sizes
    Small = process_strings(10000, 10),  % 10,000 strings of length 10
    Medium = process_strings(1000, 100), % 1,000 strings of length 100
    Large = process_strings(100, 1000),  % 100 strings of length 1,000
    
    ResultLength = length(Small) + length(Medium) + length(Large),
    io:format("Processed string length: ~p~n", [ResultLength]),
    
    ElapsedTime = erlang:system_time(millisecond) - StartTime,
    io:format("Time taken: ~p ms~n", [ElapsedTime]),
    
    {ok, [TotalWords, _]} = process_info(self(), memory),
    io:format("Memory used: ~p KB~n", [TotalWords div 1024]),
    ok.