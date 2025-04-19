(ns fannkuch-benchmark
  (:gen-class))

;; Reverse the first n elements of the array
(defn flip [p n]
  (let [result (vec p)]
    (loop [i 0, res result]
      (if (< i (/ n 2))
        (recur (inc i) (assoc res 
                             i (get res (- n i 1))
                             (- n i 1) (get res i)))
        res))))

;; Count flips required to flip elements to get back to original order
(defn fannkuch [n]
  (let [p (vec (range n))
        perm (atom (vec (range 1 (inc n))))
        count (vec (repeat n 0))
        max-flips (atom 0)
        checksum (atom 0)
        perm-count (atom 0)
        sign (atom 1)]
    
    (loop [p p, j 1]
      (if (>= @perm-count 10000)
        @max-flips
        (let [first (get p 0)]
          ;; Count flips if first element is not 0
          (when (not= first 0)
            (reset! perm (mapv inc p))
            (loop [flips 0, curr-perm @perm]
              (if (= (get curr-perm 0) 1)
                (do
                  (when (> flips @max-flips)
                    (reset! max-flips flips))
                  (swap! checksum + (* @sign flips)))
                (let [k (dec (get curr-perm 0))
                      new-perm (flip curr-perm k)]
                  (recur (inc flips) (assoc new-perm 0 (inc k)))))))
          
          ;; Generate next permutation
          (swap! sign *)
          (swap! perm-count inc)
          (swap! sign -)
          
          (let [j (loop [j j]
                    (if (and (< j n) (>= (get p (dec j)) (get p j)))
                      (recur (inc j))
                      j))]
            (if (= j n)
              @max-flips
              (let [p (loop [p p, i 0]
                        (if (< i j)
                          (if (even? i)
                            (recur (assoc p i (get p (- j i)) (- j i) (get p i)) (inc i))
                            (recur (assoc p i (get p (- j i 1)) (- j i 1) (get p i)) (inc i)))
                          p))]
                (if (< j 2)
                  (let [j (loop [j 1, i 1]
                            (if (< i n)
                              (if (> (get p (dec i)) (get p i))
                                (recur (inc i) (inc i))
                                (recur j (inc i)))
                              j))]
                    (recur (loop [p p, i 0]
                             (if (< i (dec j))
                               (let [k i
                                     temp (get p i)]
                                 (recur (loop [p p, k k]
                                          (if (< k (dec j))
                                            (recur (assoc p k (get p (inc k))) (inc k))
                                            (assoc p (dec j) temp)))
                                        (inc i)))
                               p))
                           j))
                  (recur (loop [p p, i j, result p]
                           (if (> i 0)
                             (recur p (dec i) (assoc result i (get p (dec i))))
                             (assoc result 0 (get p j))))
                         (dec j)))))))))

(defn -main [& args]
  (let [n 10
        start-time (System/currentTimeMillis)
        result (fannkuch n)]
    
    (println (str "Fannkuch(" n "): " result))
    
    (println (str "Time taken: " (- (System/currentTimeMillis) start-time) " ms"))
    
    ;; Estimate memory usage
    (println (str "Memory used: " (/ (- (Runtime/getRuntime) (.freeMemory (Runtime/getRuntime))) 1024) " KB"))))