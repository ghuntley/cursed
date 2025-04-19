(* N-body simulation benchmark adapted from The Computer Language Benchmarks Game *)

(* Constants *)
let pi = 3.141592653589793
let solar_mass = 4.0 *. pi *. pi
let days_per_year = 365.24

(* Planet record *)
type planet = {
  mutable x : float;
  mutable y : float;
  mutable z : float;
  mutable vx : float;
  mutable vy : float;
  mutable vz : float;
  mass : float;
}

(* Initialize solar system *)
let init_solar_system () =
  [|
    (* Sun *)
    { x = 0.0; y = 0.0; z = 0.0;
      vx = 0.0; vy = 0.0; vz = 0.0;
      mass = solar_mass };
    
    (* Jupiter *)
    { x = 4.84143144246472090e+00;
      y = -1.16032004402742839e+00;
      z = -1.03622044471123109e-01;
      vx = 1.66007664274403694e-03 *. days_per_year;
      vy = 7.69901118419740425e-03 *. days_per_year;
      vz = -6.90460016972063023e-05 *. days_per_year;
      mass = 9.54791938424326609e-04 *. solar_mass };
    
    (* Saturn *)
    { x = 8.34336671824457987e+00;
      y = 4.12479856412430479e+00;
      z = -4.03523417114321381e-01;
      vx = -2.76742510726862411e-03 *. days_per_year;
      vy = 4.99852801234917238e-03 *. days_per_year;
      vz = 2.30417297573763929e-05 *. days_per_year;
      mass = 2.85885980666130812e-04 *. solar_mass };
    
    (* Uranus *)
    { x = 1.28943695621391310e+01;
      y = -1.51111514016986312e+01;
      z = -2.23307578892655734e-01;
      vx = 2.96460137564761618e-03 *. days_per_year;
      vy = 2.37847173959480950e-03 *. days_per_year;
      vz = -2.96589568540237556e-05 *. days_per_year;
      mass = 4.36624404335156298e-05 *. solar_mass };
    
    (* Neptune *)
    { x = 1.53796971148509165e+01;
      y = -2.59193146099879641e+01;
      z = 1.79258772950371181e-01;
      vx = 2.68067772490389322e-03 *. days_per_year;
      vy = 1.62824170038242295e-03 *. days_per_year;
      vz = -9.51592254519715870e-05 *. days_per_year;
      mass = 5.15138902046611451e-05 *. solar_mass };
  |]

(* Offset momentum of the sun *)
let offset_momentum bodies =
  let px = ref 0.0 in
  let py = ref 0.0 in
  let pz = ref 0.0 in
  
  for i = 0 to Array.length bodies - 1 do
    px := !px +. bodies.(i).vx *. bodies.(i).mass;
    py := !py +. bodies.(i).vy *. bodies.(i).mass;
    pz := !pz +. bodies.(i).vz *. bodies.(i).mass;
  done;
  
  bodies.(0).vx <- -. !px /. solar_mass;
  bodies.(0).vy <- -. !py /. solar_mass;
  bodies.(0).vz <- -. !pz /. solar_mass

(* Calculate energy of the system *)
let energy bodies =
  let e = ref 0.0 in
  let n = Array.length bodies in
  
  for i = 0 to n - 1 do
    let b = bodies.(i) in
    e := !e +. 0.5 *. b.mass *. (b.vx *. b.vx +. b.vy *. b.vy +. b.vz *. b.vz);
    
    for j = i + 1 to n - 1 do
      let b2 = bodies.(j) in
      let dx = b.x -. b2.x in
      let dy = b.y -. b2.y in
      let dz = b.z -. b2.z in
      let distance = sqrt (dx *. dx +. dy *. dy +. dz *. dz) in
      e := !e -. (b.mass *. b2.mass) /. distance;
    done;
  done;
  
  !e

(* Advance simulation by dt *)
let advance bodies dt =
  let n = Array.length bodies in
  
  for i = 0 to n - 1 do
    let b = bodies.(i) in
    
    for j = i + 1 to n - 1 do
      let b2 = bodies.(j) in
      let dx = b.x -. b2.x in
      let dy = b.y -. b2.y in
      let dz = b.z -. b2.z in
      
      let distance = sqrt (dx *. dx +. dy *. dy +. dz *. dz) in
      let mag = dt /. (distance *. distance *. distance) in
      
      let b_mass_mag = b.mass *. mag in
      let b2_mass_mag = b2.mass *. mag in
      
      b.vx <- b.vx -. dx *. b2_mass_mag;
      b.vy <- b.vy -. dy *. b2_mass_mag;
      b.vz <- b.vz -. dz *. b2_mass_mag;
      
      b2.vx <- b2.vx +. dx *. b_mass_mag;
      b2.vy <- b2.vy +. dy *. b_mass_mag;
      b2.vz <- b2.vz +. dz *. b_mass_mag;
    done;
  done;
  
  for i = 0 to n - 1 do
    let b = bodies.(i) in
    b.x <- b.x +. dt *. b.vx;
    b.y <- b.y +. dt *. b.vy;
    b.z <- b.z +. dt *. b.vz;
  done

let () =
  let n = 1000000 in (* Number of iterations *)
  let bodies = init_solar_system () in
  let start_time = Unix.gettimeofday () in
  
  offset_momentum bodies;
  let initial_energy = energy bodies in
  Printf.printf "Initial energy: %.9f\n" initial_energy;
  
  for i = 1 to n do
    advance bodies 0.01;
  done;
  
  let final_energy = energy bodies in
  let energy_delta = final_energy -. initial_energy in
  Printf.printf "Final energy: %.9f\n" final_energy;
  Printf.printf "Energy delta: %.9f\n" energy_delta;
  
  let elapsed = int_of_float ((Unix.gettimeofday () -. start_time) *. 1000.0) in
  Printf.printf "Time taken: %d ms\n" elapsed;
  
  (* Approximate memory usage *)
  let memory_usage = Gc.stat () in
  Printf.printf "Memory used: %d KB\n" (memory_usage.minor_words |> int_of_float |> (fun x -> x / 1024))