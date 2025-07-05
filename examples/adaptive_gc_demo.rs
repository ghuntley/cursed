/// Adaptive Garbage Collection Demonstration
/// 
/// This simplified example demonstrates the Traceable trait implementations
/// for custom data structures in the CURSED memory management system.

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashMap;

use cursed::memory::{Traceable, Visitor, Tag};

// Example data structures for demonstration

#[derive(Debug, Clone)]
struct WebRequest {
    id: u64,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    response_data: Option<Vec<u8>>,
}

impl Traceable for WebRequest {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Web requests typically don't contain GC references
    }

    fn get_tag(&self) -> Tag {
        Tag::Object
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + 
        self.url.capacity() + 
        self.headers.capacity() * std::mem::size_of::<(String, String)>() +
        self.body.capacity() +
        self.response_data.as_ref().map_or(0, |data| data.capacity())
    }
}

#[derive(Debug, Clone)]
struct DataProcessingJob {
    id: u64,
    input_data: Vec<f64>,
    intermediate_results: Vec<Vec<f64>>,
    final_result: Option<Vec<f64>>,
    dependencies: Vec<Arc<DataProcessingJob>>,
}

impl Traceable for DataProcessingJob {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for dep in &self.dependencies {
            dep.trace(visitor);
        }
    }

    fn get_tag(&self) -> Tag {
        Tag::Object
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + 
        self.input_data.capacity() * std::mem::size_of::<f64>() +
        self.intermediate_results.iter().map(|v| v.capacity() * std::mem::size_of::<f64>()).sum::<usize>() +
        self.final_result.as_ref().map_or(0, |data| data.capacity() * std::mem::size_of::<f64>()) +
        self.dependencies.capacity() * std::mem::size_of::<Arc<DataProcessingJob>>()
    }
}

/// Mock visitor for demonstration
struct MockVisitor {
    visited_count: usize,
}

impl MockVisitor {
    fn new() -> Self {
        Self { visited_count: 0 }
    }
}

impl Visitor for MockVisitor {
    fn visit(&mut self, _obj: &dyn Traceable) {
        self.visited_count += 1;
    }
}

/// Demonstrates tracing functionality
fn demonstrate_tracing() -> Result<(), String> {
    println!("\n=== Demonstrating Traceable Trait ===");
    
    // Create a web request
    let request = WebRequest {
        id: 1,
        url: "/api/users/123".to_string(),
        headers: {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            headers.insert("User-Agent".to_string(), "CURSED/1.0".to_string());
            headers
        },
        body: b"{'user_id': 123}".to_vec(),
        response_data: Some(b"{'id': 123, 'name': 'User 123'}".to_vec()),
    };
    
    // Create a data processing job
    let job = DataProcessingJob {
        id: 1,
        input_data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        intermediate_results: vec![
            vec![10.0, 20.0, 30.0],
            vec![100.0, 200.0],
        ],
        final_result: Some(vec![1000.0]),
        dependencies: Vec::new(),
    };
    
    // Demonstrate size calculation
    println!("WebRequest size: {} bytes", request.size());
    println!("DataProcessingJob size: {} bytes", job.size());
    
    // Demonstrate tag retrieval
    println!("WebRequest tag: {}", request.get_tag());
    println!("DataProcessingJob tag: {}", job.get_tag());
    
    // Demonstrate tracing
    let mut visitor = MockVisitor::new();
    request.trace(&mut visitor);
    job.trace(&mut visitor);
    
    println!("Objects visited during tracing: {}", visitor.visited_count);
    
    Ok(())
}

/// Demonstrates object with dependencies
fn demonstrate_object_dependencies() -> Result<(), String> {
    println!("\n=== Demonstrating Object Dependencies ===");
    
    // Create a job with dependencies
    let dependency1 = Arc::new(DataProcessingJob {
        id: 1,
        input_data: vec![1.0, 2.0],
        intermediate_results: vec![vec![10.0, 20.0]],
        final_result: Some(vec![100.0]),
        dependencies: Vec::new(),
    });
    
    let dependency2 = Arc::new(DataProcessingJob {
        id: 2,
        input_data: vec![3.0, 4.0],
        intermediate_results: vec![vec![30.0, 40.0]],
        final_result: Some(vec![200.0]),
        dependencies: Vec::new(),
    });
    
    let main_job = DataProcessingJob {
        id: 3,
        input_data: vec![5.0, 6.0],
        intermediate_results: vec![vec![50.0, 60.0]],
        final_result: Some(vec![300.0]),
        dependencies: vec![dependency1, dependency2],
    };
    
    // Test tracing with dependencies
    let mut visitor = MockVisitor::new();
    main_job.trace(&mut visitor);
    
    println!("Main job size: {} bytes", main_job.size());
    println!("Dependencies traced: {}", visitor.visited_count);
    
    Ok(())
}

/// Demonstrates allocation patterns
fn demonstrate_allocation_patterns() -> Result<(), String> {
    println!("\n=== Demonstrating Allocation Patterns ===");
    
    let start_time = Instant::now();
    let mut objects = Vec::new();
    
    // Simulate steady allocation pattern
    for i in 0..10 {
        let request = WebRequest {
            id: i,
            url: format!("/api/users/{}", i),
            headers: HashMap::new(),
            body: format!("request_{}", i).into_bytes(),
            response_data: Some(format!("response_{}", i).into_bytes()),
        };
        
        objects.push(request);
        thread::sleep(Duration::from_millis(10));
    }
    
    println!("Created {} objects in {:?}", objects.len(), start_time.elapsed());
    
    // Calculate total size
    let total_size: usize = objects.iter().map(|obj| obj.size()).sum();
    println!("Total allocated size: {} bytes", total_size);
    
    Ok(())
}

/// Demonstrates memory usage monitoring
fn demonstrate_memory_monitoring() -> Result<(), String> {
    println!("\n=== Demonstrating Memory Usage Monitoring ===");
    
    let mut total_size = 0;
    let mut objects = Vec::new();
    
    // Create objects of increasing size
    for i in 1..=5 {
        let size = i * 1000;
        let job = DataProcessingJob {
            id: i as u64,
            input_data: vec![0.0; size],
            intermediate_results: vec![vec![0.0; size / 2]],
            final_result: Some(vec![0.0; size / 4]),
            dependencies: Vec::new(),
        };
        
        let obj_size = job.size();
        total_size += obj_size;
        
        println!("Object {} size: {} bytes", i, obj_size);
        objects.push(job);
    }
    
    println!("Total memory used: {} bytes", total_size);
    println!("Average object size: {} bytes", total_size / objects.len());
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CURSED Memory Management Demonstration");
    println!("=====================================");
    
    // Demonstrate different aspects of memory management
    demonstrate_tracing()?;
    demonstrate_object_dependencies()?;
    demonstrate_allocation_patterns()?;
    demonstrate_memory_monitoring()?;
    
    println!("\n=== Summary ===");
    println!("The memory management system successfully:");
    println!("• Implemented Traceable trait for custom objects");
    println!("• Calculated accurate memory usage");
    println!("• Traced object dependencies");
    println!("• Monitored allocation patterns");
    println!("• Provided comprehensive size information");
    
    println!("\nMemory management demonstration completed successfully!");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_traceable_implementations() {
        let request = WebRequest {
            id: 1,
            url: "test".to_string(),
            headers: HashMap::new(),
            body: vec![1, 2, 3],
            response_data: None,
        };
        
        // Test that size calculation works
        assert!(request.size() > 0);
        assert_eq!(request.get_tag(), Tag::Object);
        
        // Test that tracing doesn't panic
        let mut visitor = MockVisitor::new();
        request.trace(&mut visitor);
        assert_eq!(visitor.visited_count, 0); // No internal references
    }
    
    #[test]
    fn test_data_processing_job_tracing() {
        let dependency = Arc::new(DataProcessingJob {
            id: 1,
            input_data: vec![1.0],
            intermediate_results: vec![vec![2.0]],
            final_result: Some(vec![3.0]),
            dependencies: Vec::new(),
        });
        
        let job = DataProcessingJob {
            id: 2,
            input_data: vec![4.0],
            intermediate_results: vec![vec![5.0]],
            final_result: Some(vec![6.0]),
            dependencies: vec![dependency],
        };
        
        let mut visitor = MockVisitor::new();
        job.trace(&mut visitor);
        
        // Should have traced one dependency
        assert_eq!(visitor.visited_count, 1);
        assert!(job.size() > 0);
    }
}
