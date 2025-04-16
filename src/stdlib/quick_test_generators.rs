//! Implementations of generator functions for the Quick Test module

use crate::object::Object;
use std::rc::Rc;
use rand::{thread_rng, Rng, SeedableRng};

/// Generate a string with a length between min and max
pub fn string_of_n(min: usize, max: usize) -> impl Fn() -> Rc<Object> {
    move || {
        let mut rng = thread_rng();
        let len = rng.gen_range(min..=max);
        let s: String = (0..len)
            .map(|_| {
                // Generate ASCII alphanumeric characters for simplicity
                let idx = rng.gen_range(0..62);
                if idx < 26 {
                    // Lowercase letters
                    (b'a' + idx as u8) as char
                } else if idx < 52 {
                    // Uppercase letters
                    (b'A' + (idx - 26) as u8) as char
                } else {
                    // Digits
                    (b'0' + (idx - 52) as u8) as char
                }
            })
            .collect();
        Rc::new(Object::String(s))
    }
}

/// Generate an integer in the range [min, max]
pub fn int_range_gen(min: i64, max: i64) -> impl Fn() -> Rc<Object> {
    move || {
        let mut rng = thread_rng();
        let val = rng.gen_range(min..=max);
        Rc::new(Object::Integer(val))
    }
}

/// Generator combiner that takes multiple generators and combines them
pub fn combine(
    generators: Vec<Box<dyn Fn() -> Rc<Object>>>,
    combiner: Box<dyn Fn(&[Object]) -> Object>,
) -> impl Fn() -> Object 
{
    move || {
        let values: Vec<Object> = generators
            .iter()
            .map(|gen| {
                let rc = gen();
                match &*rc {
                    Object::Integer(i) => Object::Integer(*i),
                    Object::Float(f) => Object::Float(*f),
                    Object::Boolean(b) => Object::Boolean(*b),
                    Object::String(s) => Object::String(s.clone()),
                    Object::Array(arr) => Object::Array(arr.clone()),
                    Object::HashTable(map) => Object::HashTable(map.clone()),
                    _ => Object::Null,
                }
            })
            .collect();
        
        combiner(&values)
    }
}

/// StateMachine implementation for property testing
pub struct StateMachineImpl<T: Clone> {
    state: Rc<T>,
    actions: Vec<(String, Box<dyn Fn(&Rc<T>) -> bool>, Box<dyn Fn(&Rc<T>) -> bool>)>,
}

impl<T: Clone> StateMachineImpl<T> {
    pub fn new(initial_state: Rc<T>) -> Self {
        StateMachineImpl {
            state: initial_state,
            actions: Vec::new(),
        }
    }
    
    pub fn add_action(
        &mut self,
        name: &str,
        action: Box<dyn Fn(&Rc<T>) -> bool>,
        precondition: Box<dyn Fn(&Rc<T>) -> bool>,
    ) {
        self.actions.push((name.to_string(), action, precondition));
    }
    
    pub fn run(&self, config: &super::quick_test::Config) -> super::quick_test::TestResult {
        let mut rng = thread_rng();
        let mut successes = 0;
        
        for i in 0..config.max_count {
            // Select a random action that satisfies preconditions
            let valid_actions: Vec<&(String, Box<dyn Fn(&Rc<T>) -> bool>, Box<dyn Fn(&Rc<T>) -> bool>)> = 
                self.actions.iter()
                    .filter(|(_, _, precond)| precond(&self.state))
                    .collect();
            
            if valid_actions.is_empty() {
                // No valid actions, consider this a success
                successes += 1;
                continue;
            }
            
            let action_idx = rng.gen_range(0..valid_actions.len());
            let (name, action, _) = valid_actions[action_idx];
            
            // Apply the action
            let result = action(&self.state);
            
            if result {
                successes += 1;
            } else {
                // Action failed, return failure
                return super::quick_test::TestResult {
                    passed: false,
                    count: i,
                    failed_after: i,
                    counterexample: Some(Rc::new(Object::String(name.clone()))),
                    shrunk_counterexample: None,
                };
            }
        }
        
        super::quick_test::TestResult {
            passed: true,
            count: config.max_count,
            failed_after: 0,
            counterexample: None,
            shrunk_counterexample: None,
        }
    }
}

/// Simple implementation of a Rand for testing
pub struct RandImpl {
    seed: u64,
    rng: rand::rngs::StdRng,
}

impl RandImpl {
    pub fn new(seed: u64) -> Self {
        RandImpl {
            seed,
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }
    
    pub fn generate<F>(&mut self, gen: F, _size: usize) -> Object 
    where
        F: Fn() -> Object
    {
        gen()
    }
}

/// Create a generator that produces values from weighted options
pub fn weighted(choices: Vec<(usize, Box<dyn Fn() -> Object>)>) -> impl Fn() -> Object 
{
    // Convert weights to f64 format expected by the implementation
    let choices_f64: Vec<(Box<dyn Fn() -> Object>, f64)> = choices
        .into_iter()
        .map(|(weight, gen)| (gen, weight as f64))
        .collect();
    
    move || {
        let mut rng = thread_rng();
        let total_weight: f64 = choices_f64.iter().map(|(_, w)| w).sum();
        let mut choice = rng.gen_range(0.0..total_weight);
        
        for (gen, weight) in &choices_f64 {
            choice -= weight;
            if choice <= 0.0 {
                return gen();
            }
        }
        
        // This should never happen if weights are positive
        choices_f64[0].0()
    }
}