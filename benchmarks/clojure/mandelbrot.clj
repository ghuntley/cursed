(ns mandelbrot-benchmark
  (:gen-class))

;; Size constants
(def width 800)
(def height 800)
(def max-iterations 100)

;; Calculate the Mandelbrot set
(defn calculate-mandelbrot [max-iterations]
  (vec (for [y (range height)]
    (vec (for [x (range width)]
      (let [cx (/ (* (- (double x) (/ width 2.0)) 4.0) width)
            cy (/ (* (- (double y) (/ height 2.0)) 4.0) height)]
        (loop [zx 0.0
               zy 0.0
               iteration 0]
          (if (and (<= (+ (* zx zx) (* zy zy)) 4.0) (< iteration max-iterations))
            (let [temp (+ (- (* zx zx) (* zy zy)) cx)]
              (recur temp (+ (* 2.0 zx zy) cy) (inc iteration)))
            iteration)))))))))

;; Count non-black pixels in the result
(defn count-non-black [result max-iterations]
  (reduce + (for [y (range height)]
               (count (filter #(< % max-iterations) (nth result y))))))

(defn -main [& args]
  (let [start-time (System/currentTimeMillis)]
    
    (let [result (calculate-mandelbrot max-iterations)
          count (count-non-black result max-iterations)]
      
      (println "Mandelbrot set calculation finished.")
      (println (str "Image size: " width " x " height))
      (println (str "Maximum iterations: " max-iterations))
      (println (str "Non-black pixels: " count))
      
      (println (str "Time taken: " (- (System/currentTimeMillis) start-time) " ms"))
      
      ;; Estimate memory usage
      (println (str "Memory used: " (/ (- (Runtime/getRuntime) (.freeMemory (Runtime/getRuntime))) 1024) " KB")))))