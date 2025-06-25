use crate::error::CursedError;
/// Advanced Collections Module for CURSED
/// 
/// Provides specialized data structures including tries, graphs, spatial data structures,
/// probabilistic data structures, and advanced algorithms for high-performance applications.

use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::hash::{Hash, Hasher};
use std::cmp::{Ordering, Reverse};
use std::fmt;
use super::{CollectionsError, CollectionsResult};

// =============================================================================
// TRIE (PREFIX TREE) IMPLEMENTATION
// =============================================================================

/// Trie node for efficient string operations
#[derive(Debug)]
struct TrieNode<T> {
    children: HashMap<char, TrieNode<T>>,
    value: Option<T>,
    is_end_of_word: bool,
}

impl<T> TrieNode<T> {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            value: None,
            is_end_of_word: false,
        }
    }
}

/// Trie data structure for efficient prefix-based operations
#[derive(Debug)]
pub struct Trie<T> {
    root: TrieNode<T>,
    size: usize,
}

impl<T> Trie<T> {
    /// Creates a new empty trie
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
            size: 0,
        }
    }
    
    /// Inserts a key-value pair into the trie
    pub fn insert(&mut self, key: &str, value: T) {
        let mut current = &mut self.root;
        
        for ch in key.chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::new);
        }
        
        if !current.is_end_of_word {
            self.size += 1;
            current.is_end_of_word = true;
        }
        current.value = Some(value);
    }
    
    /// Searches for a key in the trie
    pub fn get(&self, key: &str) -> Option<&T> {
        let mut current = &self.root;
        
        for ch in key.chars() {
            current = current.children.get(&ch)?;
        }
        
        if current.is_end_of_word {
            current.value.as_ref()
        } else {
            None
        }
    }
    
    /// Checks if a key exists in the trie
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
    
    /// Finds all keys with the given prefix
    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut current = &self.root;
        
        // Navigate to the prefix node
        for ch in prefix.chars() {
            if let Some(node) = current.children.get(&ch) {
                current = node;
            } else {
                return Vec::new();
            }
        }
        
        // Collect all keys from this node
        let mut results = Vec::new();
        self.collect_keys(current, prefix.to_string(), &mut results);
        results
    }
    
    /// Finds the longest prefix of the given string that exists in the trie
    pub fn longest_prefix(&self, text: &str) -> Option<String> {
        let mut current = &self.root;
        let mut longest = None;
        let mut current_prefix = String::new();
        
        for ch in text.chars() {
            if let Some(node) = current.children.get(&ch) {
                current_prefix.push(ch);
                current = node;
                
                if current.is_end_of_word {
                    longest = Some(current_prefix.clone());
                }
            } else {
                break;
            }
        }
        
        longest
    }
    
    /// Removes a key from the trie
    pub fn remove(&mut self, key: &str) -> bool {
        if self.remove_recursive(&mut self.root, key, 0) {
            self.size -= 1;
            true
        } else {
            false
        }
    }
    
    /// Returns the number of keys in the trie
    pub fn len(&self) -> usize {
        self.size
    }
    
    /// Checks if the trie is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    /// Helper function to collect all keys from a node
    fn collect_keys(&self, node: &TrieNode<T>, prefix: String, results: &mut Vec<String>) {
        if node.is_end_of_word {
            results.push(prefix.clone());
        }
        
        for (&ch, child) in &node.children {
            let mut new_prefix = prefix.clone();
            new_prefix.push(ch);
            self.collect_keys(child, new_prefix, results);
        }
    }
    
    /// Recursive helper for remove operation
    fn remove_recursive(&mut self, node: &mut TrieNode<T>, key: &str, index: usize) -> bool {
        if index == key.len() {
            if node.is_end_of_word {
                node.is_end_of_word = false;
                node.value = None;
                return node.children.is_empty();
            }
            return false;
        }
        
        let ch = key.chars().nth(index).unwrap();
        if let Some(child) = node.children.get_mut(&ch) {
            let should_delete_child = self.remove_recursive(child, key, index + 1);
            
            if should_delete_child {
                node.children.remove(&ch);
                return !node.is_end_of_word && node.children.is_empty();
            }
        }
        
        false
    }
}

impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// GRAPH DATA STRUCTURE
// =============================================================================

/// Graph representation using adjacency list
#[derive(Debug, Clone)]
pub struct Graph<T> {
    vertices: HashMap<usize, T>,
    edges: HashMap<usize, Vec<(usize, f64)>>, // (destination, weight)
    next_id: usize,
    directed: bool,
}

impl<T> Graph<T> {
    /// Creates a new graph
    pub fn new(directed: bool) -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            next_id: 0,
            directed,
        }
    }
    
    /// Creates a new directed graph
    pub fn new_directed() -> Self {
        Self::new(true)
    }
    
    /// Creates a new undirected graph
    pub fn new_undirected() -> Self {
        Self::new(false)
    }
    
    /// Adds a vertex to the graph
    pub fn add_vertex(&mut self, data: T) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.vertices.insert(id, data);
        self.edges.insert(id, Vec::new());
        id
    }
    
    /// Adds an edge between two vertices
    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) -> CollectionsResult<()> {
        if !self.vertices.contains_key(&from) || !self.vertices.contains_key(&to) {
            return Err(CollectionsError::InvalidOperation {
                operation: "add_edge".to_string(),
                reason: "Vertex not found".to_string(),
            });
        }
        
        self.edges.get_mut(&from).unwrap().push((to, weight));
        
        if !self.directed {
            self.edges.get_mut(&to).unwrap().push((from, weight));
        }
        
        Ok(())
    }
    
    /// Gets the data for a vertex
    pub fn get_vertex(&self, id: usize) -> Option<&T> {
        self.vertices.get(&id)
    }
    
    /// Gets all neighbors of a vertex
    pub fn neighbors(&self, id: usize) -> Option<&Vec<(usize, f64)>> {
        self.edges.get(&id)
    }
    
    /// Performs depth-first search from a starting vertex
    pub fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_recursive(start, &mut visited, &mut result);
        result
    }
    
    /// Performs breadth-first search from a starting vertex
    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        queue.push_back(start);
        visited.insert(start);
        
        while let Some(vertex) = queue.pop_front() {
            result.push(vertex);
            
            if let Some(neighbors) = self.edges.get(&vertex) {
                for &(neighbor, _) in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        
        result
    }
    
    /// Finds shortest path using Dijkstra's algorithm
    pub fn shortest_path(&self, start: usize, end: usize) -> Option<(Vec<usize>, f64)> {
        let mut distances: HashMap<usize, f64> = HashMap::new();
        let mut previous: HashMap<usize, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();
        
        // Initialize distances
        for &vertex in self.vertices.keys() {
            distances.insert(vertex, f64::INFINITY);
        }
        distances.insert(start, 0.0);
        heap.push(Reverse((0.0, start)));
        
        while let Some(Reverse((dist, u))) = heap.pop() {
            if u == end {
                break;
            }
            
            if dist > distances[&u] {
                continue;
            }
            
            if let Some(neighbors) = self.edges.get(&u) {
                for &(v, weight) in neighbors {
                    let alt = distances[&u] + weight;
                    if alt < distances[&v] {
                        distances.insert(v, alt);
                        previous.insert(v, u);
                        heap.push(Reverse((alt, v)));
                    }
                }
            }
        }
        
        // Reconstruct path
        if !previous.contains_key(&end) && start != end {
            return None;
        }
        
        let mut path = Vec::new();
        let mut current = end;
        
        while current != start {
            path.push(current);
            current = previous[&current];
        }
        path.push(start);
        path.reverse();
        
        Some((path, distances[&end]))
    }
    
    /// Checks if the graph has cycles (for directed graphs)
    pub fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for &vertex in self.vertices.keys() {
            if !visited.contains(&vertex) {
                if self.has_cycle_recursive(vertex, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Returns the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
    
    /// Returns the number of edges
    pub fn edge_count(&self) -> usize {
        if self.directed {
            self.edges.values().map(|v| v.len()).sum()
        } else {
            self.edges.values().map(|v| v.len()).sum::<usize>() / 2
        }
    }
    
    /// Helper for DFS
    fn dfs_recursive(&self, vertex: usize, visited: &mut HashSet<usize>, result: &mut Vec<usize>) {
        visited.insert(vertex);
        result.push(vertex);
        
        if let Some(neighbors) = self.edges.get(&vertex) {
            for &(neighbor, _) in neighbors {
                if !visited.contains(&neighbor) {
                    self.dfs_recursive(neighbor, visited, result);
                }
            }
        }
    }
    
    /// Helper for cycle detection
    fn has_cycle_recursive(
        &self,
        vertex: usize,
        visited: &mut HashSet<usize>,
        rec_stack: &mut HashSet<usize>,
    ) -> bool {
        visited.insert(vertex);
        rec_stack.insert(vertex);
        
        if let Some(neighbors) = self.edges.get(&vertex) {
            for &(neighbor, _) in neighbors {
                if !visited.contains(&neighbor) {
                    if self.has_cycle_recursive(neighbor, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&neighbor) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(&vertex);
        false
    }
}

// =============================================================================
// BLOOM FILTER (PROBABILISTIC DATA STRUCTURE)
// =============================================================================

/// Bloom filter for approximate membership testing
#[derive(Debug, Clone)]
pub struct BloomFilter {
    bit_array: Vec<bool>,
    size: usize,
    hash_functions: usize,
    items_count: usize,
}

impl BloomFilter {
    /// Creates a new Bloom filter
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        let size = Self::optimal_size(expected_items, false_positive_rate);
        let hash_functions = Self::optimal_hash_functions(size, expected_items);
        
        Self {
            bit_array: vec![false; size],
            size,
            hash_functions,
            items_count: 0,
        }
    }
    
    /// Adds an item to the filter
    pub fn add<T: Hash>(&mut self, item: &T) {
        for i in 0..self.hash_functions {
            let hash = self.hash_item(item, i);
            let index = hash % self.size;
            self.bit_array[index] = true;
        }
        self.items_count += 1;
    }
    
    /// Tests if an item might be in the set
    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        for i in 0..self.hash_functions {
            let hash = self.hash_item(item, i);
            let index = hash % self.size;
            if !self.bit_array[index] {
                return false;
            }
        }
        true
    }
    
    /// Estimates the current false positive rate
    pub fn false_positive_rate(&self) -> f64 {
        let bits_set = self.bit_array.iter().filter(|&&b| b).count() as f64;
        let ratio = bits_set / self.size as f64;
        ratio.powi(self.hash_functions as i32)
    }
    
    /// Returns the number of items added
    pub fn len(&self) -> usize {
        self.items_count
    }
    
    /// Checks if the filter is empty
    pub fn is_empty(&self) -> bool {
        self.items_count == 0
    }
    
    /// Clears the filter
    pub fn clear(&mut self) {
        self.bit_array.fill(false);
        self.items_count = 0;
    }
    
    /// Computes optimal bit array size
    fn optimal_size(expected_items: usize, false_positive_rate: f64) -> usize {
        let ln2 = std::f64::consts::LN_2;
        let size = -(expected_items as f64 * false_positive_rate.ln()) / (ln2 * ln2);
        size.ceil() as usize
    }
    
    /// Computes optimal number of hash functions
    fn optimal_hash_functions(size: usize, expected_items: usize) -> usize {
        let ratio = size as f64 / expected_items as f64;
        let hash_functions = ratio * std::f64::consts::LN_2;
        hash_functions.ceil() as usize
    }
    
    /// Hashes an item with a specific seed
    fn hash_item<T: Hash>(&self, item: &T, seed: usize) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        seed.hash(&mut hasher);
        hasher.finish() as usize
    }
}

// =============================================================================
// SPATIAL DATA STRUCTURES - QUADTREE
// =============================================================================

/// 2D point representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// 2D bounding rectangle
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains(&self, point: &Point2D) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
    
    pub fn intersects(&self, other: &Rectangle) -> bool {
        !(self.x > other.x + other.width
            || self.x + self.width < other.x
            || self.y > other.y + other.height
            || self.y + self.height < other.y)
    }
}

/// QuadTree node
#[derive(Debug)]
enum QuadTreeNode<T> {
    Leaf {
        points: Vec<(Point2D, T)>,
    },
    Internal {
        northwest: Box<QuadTreeNode<T>>,
        northeast: Box<QuadTreeNode<T>>,
        southwest: Box<QuadTreeNode<T>>,
        southeast: Box<QuadTreeNode<T>>,
    },
}

/// QuadTree for efficient spatial queries
#[derive(Debug)]
pub struct QuadTree<T> {
    root: QuadTreeNode<T>,
    boundary: Rectangle,
    capacity: usize,
}

impl<T: Clone> QuadTree<T> {
    /// Creates a new QuadTree
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        Self {
            root: QuadTreeNode::Leaf { points: Vec::new() },
            boundary,
            capacity,
        }
    }
    
    /// Inserts a point with associated data
    pub fn insert(&mut self, point: Point2D, data: T) -> bool {
        if !self.boundary.contains(&point) {
            return false;
        }
        
        self.insert_recursive(&mut self.root, &self.boundary, point, data)
    }
    
    /// Queries points within a rectangle
    pub fn query_range(&self, range: &Rectangle) -> Vec<(Point2D, &T)> {
        let mut results = Vec::new();
        self.query_recursive(&self.root, &self.boundary, range, &mut results);
        results
    }
    
    /// Finds the nearest point to a given point
    pub fn nearest_neighbor(&self, point: &Point2D) -> Option<(Point2D, &T)> {
        let mut best = None;
        let mut best_distance = f64::INFINITY;
        
        self.nearest_recursive(
            &self.root,
            &self.boundary,
            point,
            &mut best,
            &mut best_distance,
        );
        
        best
    }
    
    /// Recursive insertion
    fn insert_recursive(
        &mut self,
        node: &mut QuadTreeNode<T>,
        boundary: &Rectangle,
        point: Point2D,
        data: T,
    ) -> bool {
        match node {
            QuadTreeNode::Leaf { points } => {
                points.push((point, data));
                
                if points.len() > self.capacity {
                    self.subdivide(node, boundary);
                }
                
                true
            }
            QuadTreeNode::Internal {
                northwest,
                northeast,
                southwest,
                southeast,
            } => {
                let mid_x = boundary.x + boundary.width / 2.0;
                let mid_y = boundary.y + boundary.height / 2.0;
                
                if point.x <= mid_x && point.y <= mid_y {
                    // Northwest
                    let nw_boundary = Rectangle::new(
                        boundary.x,
                        boundary.y,
                        boundary.width / 2.0,
                        boundary.height / 2.0,
                    );
                    self.insert_recursive(northwest, &nw_boundary, point, data)
                } else if point.x > mid_x && point.y <= mid_y {
                    // Northeast
                    let ne_boundary = Rectangle::new(
                        mid_x,
                        boundary.y,
                        boundary.width / 2.0,
                        boundary.height / 2.0,
                    );
                    self.insert_recursive(northeast, &ne_boundary, point, data)
                } else if point.x <= mid_x && point.y > mid_y {
                    // Southwest
                    let sw_boundary = Rectangle::new(
                        boundary.x,
                        mid_y,
                        boundary.width / 2.0,
                        boundary.height / 2.0,
                    );
                    self.insert_recursive(southwest, &sw_boundary, point, data)
                } else {
                    // Southeast
                    let se_boundary = Rectangle::new(
                        mid_x,
                        mid_y,
                        boundary.width / 2.0,
                        boundary.height / 2.0,
                    );
                    self.insert_recursive(southeast, &se_boundary, point, data)
                }
            }
        }
    }
    
    /// Subdivides a leaf node into four quadrants
    fn subdivide(&mut self, node: &mut QuadTreeNode<T>, boundary: &Rectangle) {
        if let QuadTreeNode::Leaf { points } = node {
            let mut nw = Box::new(QuadTreeNode::Leaf { points: Vec::new() });
            let mut ne = Box::new(QuadTreeNode::Leaf { points: Vec::new() });
            let mut sw = Box::new(QuadTreeNode::Leaf { points: Vec::new() });
            let mut se = Box::new(QuadTreeNode::Leaf { points: Vec::new() });
            
            let mid_x = boundary.x + boundary.width / 2.0;
            let mid_y = boundary.y + boundary.height / 2.0;
            
            // Redistribute points
            for (point, data) in points.drain(..) {
                if point.x <= mid_x && point.y <= mid_y {
                    if let QuadTreeNode::Leaf { points: nw_points } = nw.as_mut() {
                        nw_points.push((point, data));
                    }
                } else if point.x > mid_x && point.y <= mid_y {
                    if let QuadTreeNode::Leaf { points: ne_points } = ne.as_mut() {
                        ne_points.push((point, data));
                    }
                } else if point.x <= mid_x && point.y > mid_y {
                    if let QuadTreeNode::Leaf { points: sw_points } = sw.as_mut() {
                        sw_points.push((point, data));
                    }
                } else {
                    if let QuadTreeNode::Leaf { points: se_points } = se.as_mut() {
                        se_points.push((point, data));
                    }
                }
            }
            
            *node = QuadTreeNode::Internal {
                northwest: nw,
                northeast: ne,
                southwest: sw,
                southeast: se,
            };
        }
    }
    
    /// Recursive range query
    fn query_recursive(
        &self,
        node: &QuadTreeNode<T>,
        boundary: &Rectangle,
        range: &Rectangle,
        results: &mut Vec<(Point2D, &T)>,
    ) {
        if !boundary.intersects(range) {
            return;
        }
        
        match node {
            QuadTreeNode::Leaf { points } => {
                for (point, data) in points {
                    if range.contains(point) {
                        results.push(*point, data);
                    }
                }
            }
            QuadTreeNode::Internal {
                northwest,
                northeast,
                southwest,
                southeast,
            } => {
                let mid_x = boundary.x + boundary.width / 2.0;
                let mid_y = boundary.y + boundary.height / 2.0;
                
                let nw_boundary = Rectangle::new(
                    boundary.x,
                    boundary.y,
                    boundary.width / 2.0,
                    boundary.height / 2.0,
                );
                let ne_boundary = Rectangle::new(
                    mid_x,
                    boundary.y,
                    boundary.width / 2.0,
                    boundary.height / 2.0,
                );
                let sw_boundary = Rectangle::new(
                    boundary.x,
                    mid_y,
                    boundary.width / 2.0,
                    boundary.height / 2.0,
                );
                let se_boundary = Rectangle::new(
                    mid_x,
                    mid_y,
                    boundary.width / 2.0,
                    boundary.height / 2.0,
                );
                
                self.query_recursive(northwest, &nw_boundary, range, results);
                self.query_recursive(northeast, &ne_boundary, range, results);
                self.query_recursive(southwest, &sw_boundary, range, results);
                self.query_recursive(southeast, &se_boundary, range, results);
            }
        }
    }
    
    /// Recursive nearest neighbor search
    fn nearest_recursive(
        &self,
        node: &QuadTreeNode<T>,
        boundary: &Rectangle,
        target: &Point2D,
        best: &mut Option<(Point2D, &T)>,
        best_distance: &mut f64,
    ) {
        match node {
            QuadTreeNode::Leaf { points } => {
                for (point, data) in points {
                    let distance = target.distance_to(point);
                    if distance < *best_distance {
                        *best_distance = distance;
                        *best = Some(*point, data);
                    }
                }
            }
            QuadTreeNode::Internal {
                northwest,
                northeast,
                southwest,
                southeast,
            } => {
                let mid_x = boundary.x + boundary.width / 2.0;
                let mid_y = boundary.y + boundary.height / 2.0;
                
                // Check which quadrant the target is in first
                let quadrants = [
                    (northwest, Rectangle::new(boundary.x, boundary.y, boundary.width / 2.0, boundary.height / 2.0)),
                    (northeast, Rectangle::new(mid_x, boundary.y, boundary.width / 2.0, boundary.height / 2.0)),
                    (southwest, Rectangle::new(boundary.x, mid_y, boundary.width / 2.0, boundary.height / 2.0)),
                    (southeast, Rectangle::new(mid_x, mid_y, boundary.width / 2.0, boundary.height / 2.0)),
                ];
                
                for (quadrant, quad_boundary) in quadrants.iter() {
                    if quad_boundary.contains(target) {
                        self.nearest_recursive(quadrant, quad_boundary, target, best, best_distance);
                        break;
                    }
                }
                
                // Check other quadrants if they might contain closer points
                for (quadrant, quad_boundary) in quadrants.iter() {
                    if !quad_boundary.contains(target) {
                        // Calculate minimum distance to this quadrant
                        let min_dist = self.min_distance_to_rectangle(target, quad_boundary);
                        if min_dist < *best_distance {
                            self.nearest_recursive(quadrant, quad_boundary, target, best, best_distance);
                        }
                    }
                }
            }
        }
    }
    
    /// Calculates minimum distance from point to rectangle
    fn min_distance_to_rectangle(&self, point: &Point2D, rect: &Rectangle) -> f64 {
        let dx = (point.x - rect.x).max(0.0).max(point.x - (rect.x + rect.width));
        let dy = (point.y - rect.y).max(0.0).max(point.y - (rect.y + rect.height));
        (dx * dx + dy * dy).sqrt()
    }
}

// =============================================================================
// CONVENIENCE FUNCTIONS
// =============================================================================

/// Creates a new string trie
pub fn string_trie<T>() -> Trie<T> {
    Trie::new()
}

/// Creates a new directed graph
pub fn directed_graph<T>() -> Graph<T> {
    Graph::new_directed()
}

/// Creates a new undirected graph
pub fn undirected_graph<T>() -> Graph<T> {
    Graph::new_undirected()
}

/// Creates a bloom filter for the given parameters
pub fn bloom_filter(expected_items: usize, false_positive_rate: f64) -> BloomFilter {
    BloomFilter::new(expected_items, false_positive_rate)
}

/// Creates a new quadtree for 2D spatial indexing
pub fn quadtree<T: Clone>(boundary: Rectangle, capacity: usize) -> QuadTree<T> {
    QuadTree::new(boundary, capacity)
}

