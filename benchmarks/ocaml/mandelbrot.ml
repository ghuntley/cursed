(* Mandelbrot set calculation benchmark *)

(* Size constants *)
let width = 800
let height = 800
let max_iterations = 100

(* Calculate the Mandelbrot set *)
let calculate_mandelbrot max_iterations =
  Array.init height (fun y ->
    Array.init width (fun x ->
      let cx = (float_of_int x -. float_of_int width /. 2.0) *. 4.0 /. float_of_int width in
      let cy = (float_of_int y -. float_of_int height /. 2.0) *. 4.0 /. float_of_int height in
      
      let rec iterate zx zy iteration =
        if zx *. zx +. zy *. zy > 4.0 || iteration >= max_iterations then
          iteration
        else
          let temp = zx *. zx -. zy *. zy +. cx in
          iterate temp (2.0 *. zx *. zy +. cy) (iteration + 1)
      in
      
      iterate 0.0 0.0 0
    )
  )

(* Count non-black pixels in the result *)
let count_non_black result max_iterations =
  Array.fold_left (fun acc row ->
    acc + Array.fold_left (fun acc' pixel ->
      if pixel < max_iterations then acc' + 1 else acc'
    ) 0 row
  ) 0 result

let () =
  let start_time = Unix.gettimeofday () in
  
  let result = calculate_mandelbrot max_iterations in
  let count = count_non_black result max_iterations in
  
  Printf.printf "Mandelbrot set calculation finished.\n";
  Printf.printf "Image size: %d x %d\n" width height;
  Printf.printf "Maximum iterations: %d\n" max_iterations;
  Printf.printf "Non-black pixels: %d\n" count;
  
  let elapsed = int_of_float ((Unix.gettimeofday () -. start_time) *. 1000.0) in
  Printf.printf "Time taken: %d ms\n" elapsed;
  
  (* Approximate memory usage *)
  let memory_usage = Gc.stat () in
  Printf.printf "Memory used: %d KB\n" (memory_usage.minor_words |> int_of_float |> (fun x -> x / 1024))