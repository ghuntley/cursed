-module(n_bodies).
-export([main/0]).

% Constants
-define(PI, 3.141592653589793).
-define(SOLAR_MASS, 4.0 * ?PI * ?PI).
-define(DAYS_PER_YEAR, 365.24).

% Planet record
-record(planet, {x, y, z, vx, vy, vz, mass}).

% Initialize solar system
init_solar_system() ->
    % Sun
    Sun = #planet{
        x = 0.0,
        y = 0.0,
        z = 0.0,
        vx = 0.0,
        vy = 0.0,
        vz = 0.0,
        mass = ?SOLAR_MASS
    },
    
    % Jupiter
    Jupiter = #planet{
        x = 4.84143144246472090e+00,
        y = -1.16032004402742839e+00,
        z = -1.03622044471123109e-01,
        vx = 1.66007664274403694e-03 * ?DAYS_PER_YEAR,
        vy = 7.69901118419740425e-03 * ?DAYS_PER_YEAR,
        vz = -6.90460016972063023e-05 * ?DAYS_PER_YEAR,
        mass = 9.54791938424326609e-04 * ?SOLAR_MASS
    },
    
    % Saturn
    Saturn = #planet{
        x = 8.34336671824457987e+00,
        y = 4.12479856412430479e+00,
        z = -4.03523417114321381e-01,
        vx = -2.76742510726862411e-03 * ?DAYS_PER_YEAR,
        vy = 4.99852801234917238e-03 * ?DAYS_PER_YEAR,
        vz = 2.30417297573763929e-05 * ?DAYS_PER_YEAR,
        mass = 2.85885980666130812e-04 * ?SOLAR_MASS
    },
    
    % Uranus
    Uranus = #planet{
        x = 1.28943695621391310e+01,
        y = -1.51111514016986312e+01,
        z = -2.23307578892655734e-01,
        vx = 2.96460137564761618e-03 * ?DAYS_PER_YEAR,
        vy = 2.37847173959480950e-03 * ?DAYS_PER_YEAR,
        vz = -2.96589568540237556e-05 * ?DAYS_PER_YEAR,
        mass = 4.36624404335156298e-05 * ?SOLAR_MASS
    },
    
    % Neptune
    Neptune = #planet{
        x = 1.53796971148509165e+01,
        y = -2.59193146099879641e+01,
        z = 1.79258772950371181e-01,
        vx = 2.68067772490389322e-03 * ?DAYS_PER_YEAR,
        vy = 1.62824170038242295e-03 * ?DAYS_PER_YEAR,
        vz = -9.51592254519715870e-05 * ?DAYS_PER_YEAR,
        mass = 5.15138902046611451e-05 * ?SOLAR_MASS
    },
    
    [Sun, Jupiter, Saturn, Uranus, Neptune].

% Offset momentum of the sun
offset_momentum(Bodies) ->
    {Px, Py, Pz} = lists:foldl(
        fun(Body, {AccX, AccY, AccZ}) ->
            {AccX + Body#planet.vx * Body#planet.mass,
             AccY + Body#planet.vy * Body#planet.mass,
             AccZ + Body#planet.vz * Body#planet.mass}
        end,
        {0.0, 0.0, 0.0},
        Bodies
    ),
    
    [Sun | Rest] = Bodies,
    NewSun = Sun#planet{
        vx = -Px / ?SOLAR_MASS,
        vy = -Py / ?SOLAR_MASS,
        vz = -Pz / ?SOLAR_MASS
    },
    
    [NewSun | Rest].

% Calculate energy of the system
energy(Bodies) ->
    energy(Bodies, 0, 0.0).

energy(_, N, E) when N >= length(Bodies) ->
    E;
energy(Bodies, I, E) ->
    B = lists:nth(I+1, Bodies),
    E1 = E + 0.5 * B#planet.mass * (B#planet.vx * B#planet.vx + 
                                   B#planet.vy * B#planet.vy + 
                                   B#planet.vz * B#planet.vz),
    
    E2 = energy_inner(Bodies, I, E1, I+1),
    energy(Bodies, I+1, E2).

energy_inner(_, _, E, J) when J >= length(Bodies) ->
    E;
energy_inner(Bodies, I, E, J) ->
    B = lists:nth(I+1, Bodies),
    B2 = lists:nth(J+1, Bodies),
    
    Dx = B#planet.x - B2#planet.x,
    Dy = B#planet.y - B2#planet.y,
    Dz = B#planet.z - B2#planet.z,
    
    Distance = math:sqrt(Dx*Dx + Dy*Dy + Dz*Dz),
    E1 = E - (B#planet.mass * B2#planet.mass) / Distance,
    
    energy_inner(Bodies, I, E1, J+1).

% Advance simulation by dt
advance(Bodies, Dt) ->
    % Update velocities
    NewBodies1 = advance_velocities(Bodies, Dt, 0),
    % Update positions
    lists:map(
        fun(B) ->
            B#planet{
                x = B#planet.x + Dt * B#planet.vx,
                y = B#planet.y + Dt * B#planet.vy,
                z = B#planet.z + Dt * B#planet.vz
            }
        end,
        NewBodies1
    ).

advance_velocities(Bodies, _, I) when I >= length(Bodies) ->
    Bodies;
advance_velocities(Bodies, Dt, I) ->
    NewBodies = advance_velocities_inner(Bodies, Dt, I, I+1),
    advance_velocities(NewBodies, Dt, I+1).

advance_velocities_inner(Bodies, _, _, J) when J >= length(Bodies) ->
    Bodies;
advance_velocities_inner(Bodies, Dt, I, J) ->
    B = lists:nth(I+1, Bodies),
    B2 = lists:nth(J+1, Bodies),
    
    Dx = B#planet.x - B2#planet.x,
    Dy = B#planet.y - B2#planet.y,
    Dz = B#planet.z - B2#planet.z,
    
    Distance = math:sqrt(Dx*Dx + Dy*Dy + Dz*Dz),
    Mag = Dt / (Distance * Distance * Distance),
    
    B_mass_mag = B#planet.mass * Mag,
    B2_mass_mag = B2#planet.mass * Mag,
    
    NewB = B#planet{
        vx = B#planet.vx - Dx * B2_mass_mag,
        vy = B#planet.vy - Dy * B2_mass_mag,
        vz = B#planet.vz - Dz * B2_mass_mag
    },
    
    NewB2 = B2#planet{
        vx = B2#planet.vx + Dx * B_mass_mag,
        vy = B2#planet.vy + Dy * B_mass_mag,
        vz = B2#planet.vz + Dz * B_mass_mag
    },
    
    % Replace bodies in the list
    Bodies1 = lists:sublist(Bodies, I) ++ [NewB] ++ lists:nthtail(I+1, Bodies),
    Bodies2 = lists:sublist(Bodies1, J) ++ [NewB2] ++ lists:nthtail(J+1, Bodies1),
    
    advance_velocities_inner(Bodies2, Dt, I, J+1).

% Run simulation for n iterations
run_simulation(Bodies, _, 0) ->
    Bodies;
run_simulation(Bodies, Dt, N) ->
    NewBodies = advance(Bodies, Dt),
    run_simulation(NewBodies, Dt, N-1).

main() ->
    N = 1000000, % Number of iterations
    Bodies = init_solar_system(),
    StartTime = erlang:system_time(millisecond),
    
    BalancedBodies = offset_momentum(Bodies),
    InitialEnergy = energy(BalancedBodies),
    io:format("Initial energy: ~.9f~n", [InitialEnergy]),
    
    FinalBodies = run_simulation(BalancedBodies, 0.01, N),
    FinalEnergy = energy(FinalBodies),
    EnergyDelta = FinalEnergy - InitialEnergy,
    
    io:format("Final energy: ~.9f~n", [FinalEnergy]),
    io:format("Energy delta: ~.9f~n", [EnergyDelta]),
    
    ElapsedTime = erlang:system_time(millisecond) - StartTime,
    io:format("Time taken: ~p ms~n", [ElapsedTime]),
    
    {ok, [TotalWords, _]} = process_info(self(), memory),
    io:format("Memory used: ~p KB~n", [TotalWords div 1024]),
    ok.