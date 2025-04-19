-module(fannkuch).
-export([main/0]).

% Reverse the first n elements of the array
flip(P, N) ->
    lists:reverse(lists:sublist(P, N)) ++ lists:nthtail(N, P).

% Count flips required to flip elements to get back to original order
fannkuch(N) ->
    P = lists:seq(0, N-1),
    fannkuch_loop(P, 1, N, 0, 0, 1, 0).

fannkuch_loop(_P, _J, _N, MaxFlips, _Checksum, _Sign, PermCount) when PermCount >= 10000 ->
    MaxFlips;
fannkuch_loop(P, J, N, MaxFlips, Checksum, Sign, PermCount) ->
    First = lists:nth(1, P),
    
    % Count flips if first element is not 0
    {MaxFlips1, Checksum1} = case First of
        0 -> {MaxFlips, Checksum};
        _ ->
            Perm = [X + 1 || X <- P],
            {Flips, _} = count_flips(Perm, 0),
            {max(MaxFlips, Flips), Checksum + Sign * Flips}
    end,
    
    % Generate next permutation
    Sign1 = -Sign,
    
    % Find j
    case find_j(P, J, N) of
        N -> MaxFlips1;  % We're done
        NewJ ->
            P1 = flip_first_j(P, NewJ),
            P2 = if
                NewJ < 2 ->
                    % Special case for j < 2
                    {J1, P3} = find_j_and_rotate(P1, 1, N),
                    P3;
                true ->
                    % Regular case
                    rotate_first_j(P1, NewJ-1)
            end,
            fannkuch_loop(P2, 1, N, MaxFlips1, Checksum1, Sign1, PermCount + 1)
    end.

find_j([A | [B | _]], J, _N) when A >= B ->
    J + 1;
find_j([_ | Rest], J, N) ->
    find_j(Rest, J + 1, N);
find_j([], _J, N) ->
    N.

flip_first_j(P, J) ->
    flip_elements(P, J, 0).

flip_elements(P, J, I) when I < J ->
    Idx1 = I,
    Idx2 = case I rem 2 of
        0 -> J - I;
        1 -> J - I - 1
    end,
    P1 = lists:sublist(P, Idx1) ++ [lists:nth(Idx2+1, P)] ++ 
        lists:sublist(P, Idx1+2, Idx2-Idx1-1) ++ 
        [lists:nth(Idx1+1, P)] ++ lists:nthtail(Idx2+1, P),
    flip_elements(P1, J, I+1);
flip_elements(P, _J, _I) ->
    P.

find_j_and_rotate(P, J, N) ->
    NewJ = find_next_j(P, 1, N),
    P1 = rotate_elements(P, NewJ),
    {NewJ, P1}.

find_next_j([A, B | _], I, _N) when A > B ->
    I + 1;
find_next_j([_ | Rest], I, N) ->
    find_next_j(Rest, I + 1, N);
find_next_j([], _I, _N) ->
    1.

rotate_elements(P, J) ->
    Temp = lists:nth(1, P),
    P1 = rotate_elements_inner(P, J, 0),
    lists:sublist(P1, J) ++ [Temp] ++ lists:nthtail(J+1, P1).

rotate_elements_inner(P, _J, I) when I >= J - 1 ->
    P;
rotate_elements_inner(P, J, I) ->
    P1 = lists:sublist(P, I) ++ 
         [lists:nth(I+2, P)] ++ 
         lists:nthtail(I+2, P),
    rotate_elements_inner(P1, J, I+1).

rotate_first_j(P, J) ->
    FirstJ = lists:nth(J+1, P),
    lists:sublist(P, 1, 0) ++ [FirstJ] ++ 
    lists:sublist(P, 1, J) ++ lists:nthtail(J+2, P).

count_flips(Perm, Flips) ->
    case lists:nth(1, Perm) of
        1 -> {Flips, Perm};
        K ->
            Perm1 = flip(Perm, K),
            count_flips(Perm1, Flips + 1)
    end.

main() ->
    N = 10,
    StartTime = erlang:system_time(millisecond),
    
    Result = fannkuch(N),
    
    io:format("Fannkuch(~p): ~p~n", [N, Result]),
    
    ElapsedTime = erlang:system_time(millisecond) - StartTime,
    io:format("Time taken: ~p ms~n", [ElapsedTime]),
    
    {ok, [TotalWords, _]} = process_info(self(), memory),
    io:format("Memory used: ~p KB~n", [TotalWords div 1024]),
    ok.