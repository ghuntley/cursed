-module(binary_trees).
-export([main/0]).

% A tree is represented as {item, Left, Right} or nil

% Create a new tree with the given item value at the root
new_tree(Item, 0) ->
    {Item, nil, nil};
new_tree(Item, Depth) ->
    {Item, new_tree(2*Item-1, Depth-1), new_tree(2*Item, Depth-1)}.

% Check the tree and return a checksum
check_tree(nil) ->
    0;
check_tree({Item, nil, nil}) ->
    Item;
check_tree({Item, Left, Right}) ->
    Item + check_tree(Left) - check_tree(Right).

% Process multiple trees
process_trees(_, _, 0, Result) ->
    Result;
process_trees(Depth, I, Remaining, Result) ->
    A = new_tree(I, Depth),
    B = new_tree(-I, Depth),
    NewResult = Result + check_tree(A) + check_tree(B),
    process_trees(Depth, I+1, Remaining-1, NewResult).

main() ->
    MinDepth = 4,
    MaxDepth = 12,
    StretchDepth = MaxDepth + 1,
    StartTime = erlang:system_time(millisecond),
    
    % Allocate and check a big tree
    BigTree = new_tree(0, StretchDepth),
    io:format("stretch tree of depth ~p check: ~p~n", [StretchDepth, check_tree(BigTree)]),
    
    % Allocate a long-lived binary tree
    LongLivedTree = new_tree(0, MaxDepth),
    
    % Check trees of increasing depth
    depths_loop(MinDepth, MaxDepth, MinDepth),
    
    % Check the long-lived tree last
    io:format("long lived tree of depth ~p check: ~p~n", [MaxDepth, check_tree(LongLivedTree)]),
    
    ElapsedTime = erlang:system_time(millisecond) - StartTime,
    io:format("Time taken: ~p ms~n", [ElapsedTime]),
    
    {ok, [TotalWords, _]} = process_info(self(), memory),
    io:format("Memory used: ~p KB~n", [TotalWords div 1024]),
    ok.

depths_loop(Depth, MaxDepth, _) when Depth > MaxDepth ->
    ok;
depths_loop(Depth, MaxDepth, MinDepth) ->
    Iterations = 1 bsl (MaxDepth - Depth + MinDepth),
    Result = process_trees(Depth, 0, Iterations, 0),
    io:format("~p trees of depth ~p check: ~p~n", [Iterations*2, Depth, Result]),
    depths_loop(Depth+2, MaxDepth, MinDepth).