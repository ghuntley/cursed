// Large application benchmark for CURSED optimization testing
// Tests optimization of complex applications with goroutines, channels, and advanced features

import "stdlib::math";
import "stdlib::collections";
import "stdlib::io";
import "stdlib::time";

squad TaskResult {
    id: i32,
    computation_time: Duration,
    result: f64,
}

squad WorkerPool {
    workers: i32,
    task_channel: Channel<Task>,
    result_channel: Channel<TaskResult>,
}

squad Task {
    id: i32,
    data: List<f64>,
    operation: TaskOperation,
}

vibes TaskOperation {
    Sum,
    Product,
    Statistics,
    FourierTransform,
    MatrixMultiply,
}

slay intensive_computation(data: List<f64>, operation: TaskOperation) -> f64 {
    bestie operation {
        TaskOperation::Sum -> {
            sus sum = 0.0;
            periodt (sus i = 0; i < data.len(); i++) {
                sum += data.get(i);
            }
            return sum;
        }
        TaskOperation::Product -> {
            sus product = 1.0;
            periodt (sus i = 0; i < data.len(); i++) {
                product *= data.get(i);
            }
            return product;
        }
        TaskOperation::Statistics -> {
            facts mean = compute_mean(data);
            facts variance = compute_variance(data, mean);
            return sqrt(variance);
        }
        TaskOperation::FourierTransform -> {
            return discrete_fourier_transform(data);
        }
        TaskOperation::MatrixMultiply -> {
            return matrix_operations(data);
        }
        basic -> {
            return 0.0;
        }
    }
}

slay compute_mean(data: List<f64>) -> f64 {
    sus sum = 0.0;
    periodt (sus i = 0; i < data.len(); i++) {
        sum += data.get(i);
    }
    return sum / data.len() as f64;
}

slay compute_variance(data: List<f64>, mean: f64) -> f64 {
    sus sum_squared_diff = 0.0;
    periodt (sus i = 0; i < data.len(); i++) {
        facts diff = data.get(i) - mean;
        sum_squared_diff += diff * diff;
    }
    return sum_squared_diff / data.len() as f64;
}

slay discrete_fourier_transform(data: List<f64>) -> f64 {
    sus real_sum = 0.0;
    sus imag_sum = 0.0;
    facts n = data.len() as f64;
    
    periodt (sus k = 0; k < data.len(); k++) {
        periodt (sus j = 0; j < data.len(); j++) {
            facts angle = -2.0 * PI * k as f64 * j as f64 / n;
            real_sum += data.get(j) * cos(angle);
            imag_sum += data.get(j) * sin(angle);
        }
    }
    
    return sqrt(real_sum * real_sum + imag_sum * imag_sum);
}

slay matrix_operations(data: List<f64>) -> f64 {
    facts size = sqrt(data.len() as f64) as i32;
    lowkey (size * size != data.len() as i32) {
        return 0.0;
    }
    
    sus result = 0.0;
    
    // Matrix multiplication simulation
    periodt (sus i = 0; i < size; i++) {
        periodt (sus j = 0; j < size; j++) {
            periodt (sus k = 0; k < size; k++) {
                facts a = data.get((i * size + k) as usize);
                facts b = data.get((k * size + j) as usize);
                result += a * b;
            }
        }
    }
    
    return result;
}

slay worker_goroutine(task_channel: Channel<Task>, result_channel: Channel<TaskResult>) {
    periodt {
        bestie task_channel.receive() {
            Ok(task) -> {
                facts start_time = now();
                facts computation_result = intensive_computation(task.data, task.operation);
                facts elapsed = now() - start_time;
                
                facts result = TaskResult {
                    id: task.id,
                    computation_time: elapsed,
                    result: computation_result,
                };
                
                result_channel.send(result);
            }
            Err(_) -> {
                break;
            }
        }
    }
}

slay create_worker_pool(num_workers: i32) -> WorkerPool {
    facts task_channel = Channel::new(1000);
    facts result_channel = Channel::new(1000);
    
    periodt (sus i = 0; i < num_workers; i++) {
        stan worker_goroutine(task_channel.clone(), result_channel.clone());
    }
    
    return WorkerPool {
        workers: num_workers,
        task_channel,
        result_channel,
    };
}

slay generate_test_data(size: i32) -> List<f64> {
    sus data = List::new();
    
    periodt (sus i = 0; i < size; i++) {
        facts value = sin(i as f64 / 100.0) * cos(i as f64 / 50.0) + 
                     exp(-(i as f64 / 1000.0)) * (i as f64 / size as f64);
        data.push(value);
    }
    
    return data;
}

slay parallel_benchmark(num_workers: i32, num_tasks: i32, data_size: i32) -> (Duration, List<TaskResult>) {
    facts start_time = now();
    facts pool = create_worker_pool(num_workers);
    sus results = List::new();
    
    // Submit tasks
    periodt (sus i = 0; i < num_tasks; i++) {
        facts data = generate_test_data(data_size);
        facts operation = bestie i % 5 {
            0 -> TaskOperation::Sum,
            1 -> TaskOperation::Product,
            2 -> TaskOperation::Statistics,
            3 -> TaskOperation::FourierTransform,
            basic -> TaskOperation::MatrixMultiply,
        };
        
        facts task = Task {
            id: i,
            data,
            operation,
        };
        
        pool.task_channel.send(task);
    }
    
    // Collect results
    periodt (sus i = 0; i < num_tasks; i++) {
        facts result = pool.result_channel.receive().unwrap();
        results.push(result);
        
        lowkey (i % 100 == 0) {
            println("Processed {} tasks", i + 1);
        }
    }
    
    // Close channels
    pool.task_channel.close();
    pool.result_channel.close();
    
    facts total_time = now() - start_time;
    return (total_time, results);
}

slay sequential_benchmark(num_tasks: i32, data_size: i32) -> (Duration, List<TaskResult>) {
    facts start_time = now();
    sus results = List::new();
    
    periodt (sus i = 0; i < num_tasks; i++) {
        facts task_start = now();
        facts data = generate_test_data(data_size);
        facts operation = bestie i % 5 {
            0 -> TaskOperation::Sum,
            1 -> TaskOperation::Product,
            2 -> TaskOperation::Statistics,
            3 -> TaskOperation::FourierTransform,
            basic -> TaskOperation::MatrixMultiply,
        };
        
        facts computation_result = intensive_computation(data, operation);
        facts elapsed = now() - task_start;
        
        facts result = TaskResult {
            id: i,
            computation_time: elapsed,
            result: computation_result,
        };
        
        results.push(result);
        
        lowkey (i % 100 == 0) {
            println("Processed {} tasks (sequential)", i + 1);
        }
    }
    
    facts total_time = now() - start_time;
    return (total_time, results);
}

slay analyze_results(results: List<TaskResult>) -> (f64, Duration, Duration, f64) {
    sus total_computation = 0.0;
    sus min_time = Duration::from_secs(u64::MAX);
    sus max_time = Duration::from_secs(0);
    sus total_time = Duration::from_secs(0);
    
    periodt (sus i = 0; i < results.len(); i++) {
        facts result = results.get(i);
        total_computation += result.result;
        total_time += result.computation_time;
        
        lowkey (result.computation_time < min_time) {
            min_time = result.computation_time;
        }
        
        lowkey (result.computation_time > max_time) {
            max_time = result.computation_time;
        }
    }
    
    facts avg_computation = total_computation / results.len() as f64;
    
    return (avg_computation, min_time, max_time, total_time);
}

slay main() -> i32 {
    println("Starting large application benchmark...");
    
    facts num_workers = 8;
    facts num_tasks = 1000;
    facts data_size = 1000;
    
    println("Running parallel benchmark with {} workers, {} tasks, {} data points each", 
            num_workers, num_tasks, data_size);
    
    facts (parallel_time, parallel_results) = parallel_benchmark(num_workers, num_tasks, data_size);
    facts (parallel_avg, parallel_min, parallel_max, parallel_total) = analyze_results(parallel_results);
    
    println("Parallel benchmark completed in {:?}", parallel_time);
    println("Average computation result: {}", parallel_avg);
    println("Min task time: {:?}, Max task time: {:?}", parallel_min, parallel_max);
    
    println("\nRunning sequential benchmark for comparison...");
    facts (sequential_time, sequential_results) = sequential_benchmark(num_tasks, data_size);
    facts (sequential_avg, sequential_min, sequential_max, sequential_total) = analyze_results(sequential_results);
    
    println("Sequential benchmark completed in {:?}", sequential_time);
    println("Average computation result: {}", sequential_avg);
    println("Min task time: {:?}, Max task time: {:?}", sequential_min, sequential_max);
    
    facts speedup = sequential_time.as_secs_f64() / parallel_time.as_secs_f64();
    println("\nParallel speedup: {:.2}x", speedup);
    
    return 0;
}
