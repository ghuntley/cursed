(ns binary-trees-benchmark
  (:gen-class))

;; A TreeNode record
(defrecord TreeNode [left right item])

;; Create a new tree with the given item value at the root
(defn new-tree [item depth]
  (if (<= depth 0)
    (->TreeNode nil nil item)
    (->TreeNode 
      (new-tree (- (* 2 item) 1) (- depth 1))
      (new-tree (* 2 item) (- depth 1))
      item)))

;; Check the tree and return a checksum
(defn check-tree [node]
  (if (nil? node)
    0
    (if (nil? (:left node))
      (:item node)
      (+ (:item node) 
         (- (check-tree (:left node)) 
            (check-tree (:right node)))))))

(defn -main [& args]
  (let [min-depth 4
        max-depth 12
        stretch-depth (+ max-depth 1)
        start-time (System/currentTimeMillis)]

    ;; Allocate and check a big tree
    (let [big-tree (new-tree 0 stretch-depth)]
      (println (str "stretch tree of depth " stretch-depth " check: " (check-tree big-tree))))

    ;; Allocate a long-lived binary tree
    (let [long-lived-tree (new-tree 0 max-depth)]

      ;; Check trees of increasing depth
      (doseq [depth (range min-depth (inc max-depth) 2)]
        (let [iterations (bit-shift-left 1 (+ min-depth (- max-depth depth)))
              result (atom 0)]

          (doseq [i (range iterations)]
            (let [a (new-tree i depth)
                  b (new-tree (- i) depth)]
              (swap! result + (check-tree a) (check-tree b))))

          (println (str (* 2 iterations) " trees of depth " depth " check: " @result))))

      ;; Check the long-lived tree last
      (println (str "long lived tree of depth " max-depth " check: " (check-tree long-lived-tree))))

    (println (str "Time taken: " (- (System/currentTimeMillis) start-time) " ms"))
    
    ;; Estimate memory usage
    (println (str "Memory used: " (/ (- (Runtime/getRuntime) (.freeMemory (Runtime/getRuntime))) 1024) " KB"))))