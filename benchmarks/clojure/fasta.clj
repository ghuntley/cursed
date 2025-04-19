(ns fasta-benchmark
  (:require [clojure.string :as str])
  (:gen-class))

;; Constants for the random number generator
(def IM 139968)
(def IA 3877)
(def IC 29573)
(def SEED (atom 42))

;; Define DNA sequences
(def ALU "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA")

(def IUB-CHAR ["a" "c" "g" "t" "B" "D" "H" "K" "M" "N" "R" "S" "V" "W" "Y"])
(def IUB-PROB [0.27 0.12 0.12 0.27 0.02 0.02 0.02 0.02 0.02 0.02 0.02 0.02 0.02 0.02 0.02])

(def HOMO-SAPIENS-CHAR ["a" "c" "g" "t"])
(def HOMO-SAPIENS-PROB [0.3029549426680 0.1979883004921 0.1975473066391 0.3015094502008])

;; Generate a random number
(defn gen-random []
  (swap! SEED #(mod (+ (* % IA) IC) IM))
  (/ @SEED IM))

;; Generate a random FASTA sequence
(defn gen-random-fasta [n probs chars]
  (let [length (count probs)
        cumulative-probs (reductions + probs)]
    (apply str
      (for [_ (range n)]
        (let [r (gen-random)]
          (loop [j 0]
            (if (< r (nth cumulative-probs j))
              (nth chars j)
              (recur (inc j)))))))))

;; Repeat a sequence until it reaches the required length
(defn repeat-fasta [n seq]
  (let [seq-len (count seq)]
    (apply str
      (for [i (range n)]
        (nth seq (mod i seq-len))))))

(defn -main [& args]
  (let [n 1000000
        start-time (System/currentTimeMillis)]
    
    ;; Write FASTA header and sequence for Homo sapiens Alu
    (println ">ONE Homo sapiens alu")
    (println (repeat-fasta n ALU))
    
    ;; Write FASTA header and random sequence for IUB ambiguity codes
    (println ">TWO IUB ambiguity codes")
    (println (gen-random-fasta n IUB-PROB IUB-CHAR))
    
    ;; Write FASTA header and random sequence for Homo sapiens frequency
    (println ">THREE Homo sapiens frequency")
    (println (gen-random-fasta n HOMO-SAPIENS-PROB HOMO-SAPIENS-CHAR))
    
    (println (str "Time taken: " (- (System/currentTimeMillis) start-time) " ms"))
    (println (str "Memory used: " (/ (- (Runtime/getRuntime) (.freeMemory (Runtime/getRuntime))) 1024) " KB"))))