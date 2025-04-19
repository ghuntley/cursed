(* String processing benchmark *)

let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"

let process_strings count size =
  let result = Buffer.create (count * size / 2) in
  for i = 1 to count do
    let str = create_random_string size in
    let processed = process_string str in
    Buffer.add_string result processed
  done;
  Buffer.contents result

and create_random_string size =
  let result = Bytes.create size in
  for i = 0 to size - 1 do
    let idx = Random.int (String.length chars) in
    Bytes.set result i chars.[idx]
  done;
  Bytes.to_string result

and process_string input =
  (* Replace all vowels with their uppercase version *)
  let result = ref input in
  List.iter (fun (vowel, vowel_upper) ->
    result := Str.global_replace (Str.regexp vowel) vowel_upper !result
  ) ["a", "A"; "e", "E"; "i", "I"; "o", "O"; "u", "U"];
  
  (* Replace all digits with their doubled value *)
  for i = 0 to 9 do
    let digit = string_of_int i in
    let doubled = string_of_int (i * 2) in
    result := Str.global_replace (Str.regexp digit) doubled !result
  done;
  
  (* Capitalize the first letter *)
  let result =
    if String.length !result > 0 then
      let first = String.sub !result 0 1 in
      let rest = String.sub !result 1 (String.length !result - 1) in
      String.uppercase_ascii first ^ rest
    else ""
  in
  
  (* Reverse the string *)
  let len = String.length result in
  let reversed = Bytes.create len in
  for i = 0 to len - 1 do
    Bytes.set reversed i result.[len - i - 1]
  done;
  let reversed = Bytes.to_string reversed in
  
  (* Take the first half of the reversed string *)
  String.sub reversed 0 (len / 2)

let () =
  (* Seed the random number generator *)
  Random.init 42;
  let start_time = Unix.gettimeofday () in
  
  (* Process strings of different sizes *)
  let small = process_strings 10000 10 in   (* 10,000 strings of length 10 *)
  let medium = process_strings 1000 100 in  (* 1,000 strings of length 100 *)
  let large = process_strings 100 1000 in   (* 100 strings of length 1,000 *)
  
  let result_length = String.length small + String.length medium + String.length large in
  Printf.printf "Processed string length: %d\n" result_length;
  
  let elapsed = int_of_float ((Unix.gettimeofday () -. start_time) *. 1000.0) in
  Printf.printf "Time taken: %d ms\n" elapsed;
  
  (* Approximate memory usage *)
  let memory_usage = Gc.stat () in
  Printf.printf "Memory used: %d KB\n" (memory_usage.minor_words |> int_of_float |> (fun x -> x / 1024))