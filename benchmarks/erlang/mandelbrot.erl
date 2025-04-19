-module(mandelbrot).
-export([main/0]).

% Size constants
-define(WIDTH, 800).
-define(HEIGHT, 800).
-define(MAX_ITERATIONS, 100).

% Calculate the Mandelbrot set
calculate_mandelbrot(MaxIterations) ->
    lists:map(fun(Y) ->
        lists:map(fun(X) ->
            Cx = (X - ?WIDTH / 2.0) * 4.0 / ?WIDTH,
            Cy = (Y - ?HEIGHT / 2.0) * 4.0 / ?HEIGHT,
            calculate_pixel(Cx, Cy, MaxIterations)
        end, lists:seq(0, ?WIDTH - 1))
    end, lists:seq(0, ?HEIGHT - 1)).

calculate_pixel(Cx, Cy, MaxIterations) ->
    iterate(0.0, 0.0, Cx, Cy, 0, MaxIterations).

iterate(Zx, Zy, Cx, Cy, Iteration, MaxIterations) when Zx*Zx + Zy*Zy =< 4.0, Iteration < MaxIterations ->
    Temp = Zx*Zx - Zy*Zy + Cx,
    iterate(Temp, 2.0*Zx*Zy + Cy, Cx, Cy, Iteration + 1, MaxIterations);
iterate(_, _, _, _, Iteration, _) ->
    Iteration.

% Count non-black pixels in the result
count_non_black(Result, MaxIterations) ->
    lists:foldl(fun(Row, RowAcc) ->
        RowAcc + lists:foldl(fun(Pixel, PixelAcc) ->
            case Pixel < MaxIterations of
                true -> PixelAcc + 1;
                false -> PixelAcc
            end
        end, 0, Row)
    end, 0, Result).

main() ->
    StartTime = erlang:system_time(millisecond),
    
    Result = calculate_mandelbrot(?MAX_ITERATIONS),
    Count = count_non_black(Result, ?MAX_ITERATIONS),
    
    io:format("Mandelbrot set calculation finished.~n"),
    io:format("Image size: ~p x ~p~n", [?WIDTH, ?HEIGHT]),
    io:format("Maximum iterations: ~p~n", [?MAX_ITERATIONS]),
    io:format("Non-black pixels: ~p~n", [Count]),
    
    ElapsedTime = erlang:system_time(millisecond) - StartTime,
    io:format("Time taken: ~p ms~n", [ElapsedTime]),
    
    {ok, [TotalWords, _]} = process_info(self(), memory),
    io:format("Memory used: ~p KB~n", [TotalWords div 1024]),
    ok.