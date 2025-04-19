(ns string-processing-benchmark
  (:require [clojure.string :as str])
  (:gen-class))

(defn create-random-string [size]
  (let [chars "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"]
    (apply str (repeatedly size #(rand-nth chars)))))

(defn process-string [input]
  (let [; Replace all vowels with their uppercase version
        result (-> input
                 (str/replace "a" "A")
                 (str/replace "e" "E")
                 (str/replace "i" "I")
                 (str/replace "o" "O")
                 (str/replace "u" "U"))
        
        ; Replace all digits with their doubled value
        result (reduce (fn [s i]
                         (str/replace s (str i) (str (* i 2))))
                       result
                       (range 10))
        
        ; Capitalize the first letter
        result (if (> (count result) 0)
                 (str/upper-case (subs result 0 1)) 
                 "") 
                 
        ; Rest of the string (excluding first letter)
        result (if (> (count input) 1)
                 (str result (subs input 1))
                 result)
                 
        ; Reverse the string
        reversed (apply str (reverse result))
        
        ; Take the first half of the reversed string
        half-len (quot (count reversed) 2)]
    
    (subs reversed 0 half-len)))

(defn process-strings [count size]
  (let [result (atom "")]
    (dotimes [i count]
      (let [str (create-random-string size)
            processed (process-string str)]
        (swap! result str processed)))
    @result))

(defn -main [& args]
  (let [start-time (System/currentTimeMillis)]
    
    ; Process strings of different sizes
    (let [small (process-strings 10000 10)    ; 10,000 strings of length 10
          medium (process-strings 1000 100)   ; 1,000 strings of length 100
          large (process-strings 100 1000)    ; 100 strings of length 1,000
          
          result-length (+ (count small) (count medium) (count large))]
      
      (println (str "Processed string length: " result-length))
      
      (println (str "Time taken: " (- (System/currentTimeMillis) start-time) " ms"))
      
      ; Estimate memory usage
      (println (str "Memory used: " (/ (- (Runtime/getRuntime) (.freeMemory (Runtime/getRuntime))) 1024) " KB")))))