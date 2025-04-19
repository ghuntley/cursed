(ns n-bodies-benchmark
  (:gen-class))

(def pi 3.141592653589793)
(def solar-mass (* 4.0 pi pi))
(def days-per-year 365.24)

;; Planet record
(defrecord Planet [x y z vx vy vz mass])

;; Initialize solar system
(defn init-solar-system []
  (let [bodies (vec [
    ;; Sun
    (->Planet 0.0 0.0 0.0 0.0 0.0 0.0 solar-mass)
    
    ;; Jupiter
    (->Planet 
      4.84143144246472090e+00
      -1.16032004402742839e+00
      -1.03622044471123109e-01
      (* 1.66007664274403694e-03 days-per-year)
      (* 7.69901118419740425e-03 days-per-year)
      (* -6.90460016972063023e-05 days-per-year)
      (* 9.54791938424326609e-04 solar-mass))
    
    ;; Saturn
    (->Planet
      8.34336671824457987e+00
      4.12479856412430479e+00
      -4.03523417114321381e-01
      (* -2.76742510726862411e-03 days-per-year)
      (* 4.99852801234917238e-03 days-per-year)
      (* 2.30417297573763929e-05 days-per-year)
      (* 2.85885980666130812e-04 solar-mass))
    
    ;; Uranus
    (->Planet
      1.28943695621391310e+01
      -1.51111514016986312e+01
      -2.23307578892655734e-01
      (* 2.96460137564761618e-03 days-per-year)
      (* 2.37847173959480950e-03 days-per-year)
      (* -2.96589568540237556e-05 days-per-year)
      (* 4.36624404335156298e-05 solar-mass))
    
    ;; Neptune
    (->Planet
      1.53796971148509165e+01
      -2.59193146099879641e+01
      1.79258772950371181e-01
      (* 2.68067772490389322e-03 days-per-year)
      (* 1.62824170038242295e-03 days-per-year)
      (* -9.51592254519715870e-05 days-per-year)
      (* 5.15138902046611451e-05 solar-mass))
  ])]
    bodies))

;; Offset momentum of the sun
(defn offset-momentum [bodies]
  (let [px (reduce + (map #(* (:vx %) (:mass %)) bodies))
        py (reduce + (map #(* (:vy %) (:mass %)) bodies))
        pz (reduce + (map #(* (:vz %) (:mass %)) bodies))
        sun (first bodies)]
    (assoc bodies 0 
      (assoc sun 
        :vx (/ (- px) solar-mass)
        :vy (/ (- py) solar-mass)
        :vz (/ (- pz) solar-mass)))))

;; Calculate energy of the system
(defn energy [bodies]
  (loop [e 0.0, i 0]
    (if (< i (count bodies))
      (let [b (nth bodies i)
            e (+ e (* 0.5 (:mass b) (+ (* (:vx b) (:vx b)) 
                                      (* (:vy b) (:vy b)) 
                                      (* (:vz b) (:vz b)))))
            e (loop [e e, j (inc i)]
                (if (< j (count bodies))
                  (let [b2 (nth bodies j)
                        dx (- (:x b) (:x b2))
                        dy (- (:y b) (:y b2))
                        dz (- (:z b) (:z b2))
                        distance (Math/sqrt (+ (* dx dx) (* dy dy) (* dz dz)))
                        e (- e (/ (* (:mass b) (:mass b2)) distance))]
                    (recur e (inc j)))
                  e))]
        (recur e (inc i)))
      e)))

;; Advance simulation by dt
(defn advance [bodies dt]
  (let [bodies-count (count bodies)]
    ;; Update velocities
    (loop [bodies bodies, i 0]
      (if (< i bodies-count)
        (let [b (nth bodies i)
              bodies (loop [bodies bodies, j (inc i)]
                      (if (< j bodies-count)
                        (let [b2 (nth bodies j)
                              dx (- (:x b) (:x b2))
                              dy (- (:y b) (:y b2))
                              dz (- (:z b) (:z b2))
                              distance (Math/sqrt (+ (* dx dx) (* dy dy) (* dz dz)))
                              mag (/ dt (* distance distance distance))
                              b-mass-mag (* (:mass b) mag)
                              b2-mass-mag (* (:mass b2) mag)
                              b (assoc b 
                                  :vx (- (:vx b) (* dx b2-mass-mag))
                                  :vy (- (:vy b) (* dy b2-mass-mag))
                                  :vz (- (:vz b) (* dz b2-mass-mag)))
                              b2 (assoc b2
                                   :vx (+ (:vx b2) (* dx b-mass-mag))
                                   :vy (+ (:vy b2) (* dy b-mass-mag))
                                   :vz (+ (:vz b2) (* dz b-mass-mag)))
                              bodies (assoc bodies i b j b2)]
                          (recur bodies (inc j)))
                        bodies))]
          (recur bodies (inc i)))
        ;; Update positions
        (mapv (fn [b]
                (assoc b
                  :x (+ (:x b) (* dt (:vx b)))
                  :y (+ (:y b) (* dt (:vy b)))
                  :z (+ (:z b) (* dt (:vz b)))))
              bodies)))))

(defn -main [& args]
  (let [n 1000000 ; Number of iterations
        bodies (init-solar-system)
        start-time (System/currentTimeMillis)
        bodies (offset-momentum bodies)
        initial-energy (energy bodies)]
    
    (println (str "Initial energy: " (format "%.9f" initial-energy)))
    
    (loop [bodies bodies, i 0]
      (if (< i n)
        (recur (advance bodies 0.01) (inc i))
        (let [final-energy (energy bodies)
              energy-delta (- final-energy initial-energy)]
          (println (str "Final energy: " (format "%.9f" final-energy)))
          (println (str "Energy delta: " (format "%.9f" energy-delta)))
          
          (println (str "Time taken: " (- (System/currentTimeMillis) start-time) " ms"))
          (println (str "Memory used: " (/ (- (Runtime/getRuntime) (.freeMemory (Runtime/getRuntime))) 1024) " KB"))))))