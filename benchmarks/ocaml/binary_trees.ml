(* Binary trees benchmark adapted from The Computer Language Benchmarks Game *)

(* A tree type *)
type tree = 
  | Nil
  | Node of int * tree * tree

(* Create a new tree with the given item value at the root *)
let rec new_tree item depth =
  if depth <= 0 then
    Node(item, Nil, Nil)
  else
    Node(item, 
         new_tree (2 * item - 1) (depth - 1),
         new_tree (2 * item) (depth - 1))

(* Check the tree and return a checksum *)
let rec check_tree = function
  | Nil -> 0
  | Node(item, Nil, Nil) -> item
  | Node(item, left, right) -> item + check_tree left - check_tree right

(* Process multiple trees *)
let rec process_trees depth i remaining result =
  if remaining = 0 then
    result
  else
    let a = new_tree i depth in
    let b = new_tree (-i) depth in
    let new_result = result + check_tree a + check_tree b in
    process_trees depth (i + 1) (remaining - 1) new_result

let () =
  let min_depth = 4 in
  let max_depth = 12 in
  let stretch_depth = max_depth + 1 in
  let start_time = Unix.gettimeofday () in
  
  (* Allocate and check a big tree *)
  let stretch_tree = new_tree 0 stretch_depth in
  Printf.printf "stretch tree of depth %d check: %d\n" stretch_depth (check_tree stretch_tree);
  
  (* Allocate a long-lived binary tree *)
  let long_lived_tree = new_tree 0 max_depth in
  
  (* Check trees of increasing depth *)
  let rec depth_loop depth =
    if depth <= max_depth then begin
      let iterations = 1 lsl (max_depth - depth + min_depth) in
      let result = process_trees depth 0 iterations 0 in
      Printf.printf "%d trees of depth %d check: %d\n" (iterations * 2) depth result;
      depth_loop (depth + 2)
    end
  in
  depth_loop min_depth;
  
  (* Check the long-lived tree last *)
  Printf.printf "long lived tree of depth %d check: %d\n" max_depth (check_tree long_lived_tree);
  
  let elapsed = int_of_float ((Unix.gettimeofday () -. start_time) *. 1000.0) in
  Printf.printf "Time taken: %d ms\n" elapsed;
  
  (* Approximate memory usage *)
  let memory_usage = Gc.stat () in
  Printf.printf "Memory used: %d KB\n" (memory_usage.minor_words |> int_of_float |> (fun x -> x / 1024))