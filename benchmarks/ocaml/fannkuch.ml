(* Fannkuch redux benchmark *)

(* Reverse the first n elements of the array *)
let flip arr n =
  let result = Array.copy arr in
  for i = 0 to (n - 1) / 2 do
    let temp = result.(i) in
    result.(i) <- result.(n - i - 1);
    result.(n - i - 1) <- temp
  done;
  result

(* Count flips required to flip elements to get back to original order *)
let fannkuch n =
  let p = Array.init n (fun i -> i) in
  let perm = Array.make n 0 in
  let count = Array.make n 0 in
  let rec loop max_flips checksum sign perm_count =
    if perm_count >= 10000 then max_flips
    else begin
      (* Copy permutation to perm *)
      for i = 0 to n - 1 do
        perm.(i) <- p.(i) + 1
      done;

      let first = p.(0) in
      let max_flips', checksum' =
        if first <> 0 then begin
          (* Count flips *)
          Array.fill count 0 n 0;
          
          let rec count_flips perm' flips =
            if perm'.(0) = 1 then flips
            else begin
              let k = perm'.(0) - 1 in
              let perm'' = flip perm' k in
              count_flips perm'' (flips + 1)
            end
          in
          
          let flips = count_flips perm 0 in
          let max_flips' = max max_flips flips in
          let checksum' = checksum + sign * flips in
          (max_flips', checksum')
        end else (max_flips, checksum)
      in

      (* Generate next permutation *)
      let sign' = -sign in
      
      let rec find_j j =
        if j = n then None
        else if p.(j-1) < p.(j) then Some j
        else find_j (j+1)
      in
      
      match find_j 1 with
      | None -> max_flips'  (* We're done *)
      | Some j ->
          let perm_count' = perm_count + 1 in
          
          (* Flip first j elements *)
          for i = 0 to j / 2 - 1 do
            let idx1 = i in
            let idx2 = if i mod 2 = 0 then j - i else j - i - 1 in
            let temp = p.(idx1) in
            p.(idx1) <- p.(idx2);
            p.(idx2) <- temp
          done;
          
          (* Handle special cases *)
          if j < 2 then begin
            (* Find new j *)
            let rec find_next_j j' i =
              if i = n then j'
              else if p.(i-1) > p.(i) then find_next_j (i+1) (i+1)
              else find_next_j j' (i+1)
            in
            let j' = find_next_j 1 1 in
            
            (* Rotate elements *)
            for i = 0 to j' - 2 do
              let k = ref i in
              let temp = p.(i) in
              while !k < j' - 1 do
                incr k;
                p.(!k - 1) <- p.(!k)
              done;
              p.(j' - 1) <- temp
            done;
            
            loop max_flips' checksum' sign' perm_count'
          end else begin
            (* Regular case - rotate elements *)
            let j' = j - 1 in
            let first_j = p.(j') in
            for i = j' downto 1 do
              p.(i) <- p.(i-1)
            done;
            p.(0) <- first_j;
            
            loop max_flips' checksum' sign' perm_count'
          end
    end
  in
  loop 0 0 1 0

let () =
  let n = 10 in
  let start_time = Unix.gettimeofday () in
  
  let result = fannkuch n in
  
  Printf.printf "Fannkuch(%d): %d\n" n result;
  
  let elapsed = int_of_float ((Unix.gettimeofday () -. start_time) *. 1000.0) in
  Printf.printf "Time taken: %d ms\n" elapsed;
  
  (* Approximate memory usage *)
  let memory_usage = Gc.stat () in
  Printf.printf "Memory used: %d KB\n" (memory_usage.minor_words |> int_of_float |> (fun x -> x / 1024))