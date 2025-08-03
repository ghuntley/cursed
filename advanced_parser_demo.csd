// Advanced CURSED Parser Features Demonstration
// This file showcases all the completed advanced parser features

vibe advanced_demo

yeet "testz"
yeet "collections" as coll
yeet { HashMap, Vec } from "std/collections"

// Generic function with constraints
slay compare<T: Comparable + Clone>(a T, b T) normie 
where T: Display {
    lowkey a < b {
        damn -1
    } lowkey a > b {
        damn 1
    } highkey {
        damn 0
    }
}

// Generic struct with inheritance
squad Container<T: Clone> {
    spill items []T
    spill capacity normie = 10
    spill is_sorted lit = cringe
    
    // Constructor with type parameters
    slay new<U: Into<T>>(initial_items []U) Container<T> {
        damn Container {
            items: initial_items.map(|x| x.into()),
            capacity: initial_items.len() * 2,
            is_sorted: cringe
        }
    }
    
    // Method with complex generics
    slay find<F: Fn(T) -> lit>(predicate F) Option<T> {
        bestie item in self.items {
            lowkey predicate(item) {
                damn Some(item.clone())
            }
        }
        damn None
    }
}

// Interface with inheritance and associated types
collab Iterator<T> extends Sized {
    be_like Item = T
    be_like Error = tea
    
    slay next() Option<Item>
    slay size_hint() (normie, Option<normie>)
    
    // Default implementation
    slay collect<C: FromIterator<Item>>() Result<C, Error> {
        sus collection = C::new()
        bestie item in self {
            collection.insert(item)
        }
        damn Ok(collection)
    }
}

// Complex interface composition
collab Drawable extends Positioned, Colored {
    slay draw(canvas Canvas)
    slay area() meal
}

collab Shape extends Drawable {
    slay perimeter() meal
    slay contains(point Point) lit
}

// Advanced struct with multiple interfaces
squad Circle {
    spill center Point
    spill radius meal
    spill color Color = Color::Red
}

// Interface implementation with generics
flex Circle => Shape {
    slay draw(canvas Canvas) {
        canvas.draw_circle(self.center, self.radius, self.color)
    }
    
    slay area() meal {
        damn 3.14159 * self.radius * self.radius
    }
    
    slay perimeter() meal {
        damn 2.0 * 3.14159 * self.radius
    }
    
    slay contains(point Point) lit {
        sus distance = self.center.distance_to(point)
        damn distance <= self.radius
    }
}

// Advanced pattern matching with guards and destructuring
slay analyze_shape(shape Shape) tea {
    vibe_check shape {
        Circle { radius: r, center: Point { x: 0.0, y: 0.0 } } if r > 10.0 => 
            "Large circle at origin",
        
        Circle { radius, .. } if radius < 1.0 => 
            "Small circle",
        
        Rectangle { width: w, height: h } if w == h => 
            spillf("Square with side {}", w),
        
        Rectangle { width, height } => 
            spillf("Rectangle {}x{}", width, height),
        
        Triangle { a, b, c } if a + b > c && b + c > a && a + c > b => 
            "Valid triangle",
        
        Polygon { vertices } if vertices.len() > 8 => 
            "Complex polygon",
        
        shape if shape.area() > 100.0 => 
            "Large shape",
        
        _ => "Unknown shape"
    }
}

// Complex control flow with pattern matching
slay process_data(data []Value) Result<tea, Error> {
    sus results []tea = []
    
    bestie (index, value) in data.enumerate() {
        vibe_check value {
            Value::String(s) if s.len() > 0 => {
                results.push(spillf("String[{}]: {}", index, s))
            },
            
            Value::Number(n) if n > 0 => {
                results.push(spillf("Positive[{}]: {}", index, n))
            },
            
            Value::Array(arr) => {
                bestie (i, item) in arr.enumerate() {
                    vibe_check item {
                        Value::String(nested) => {
                            results.push(spillf("Nested[{}][{}]: {}", index, i, nested))
                        },
                        _ => simp  // continue to next item
                    }
                }
            },
            
            Value::Object { name, .. } if name.starts_with("special_") => {
                results.push(spillf("Special object[{}]: {}", index, name))
            },
            
            Value::Null => {
                // Skip null values
                simp
            },
            
            _ => {
                results.push(spillf("Other[{}]: {:?}", index, value))
            }
        }
    }
    
    damn Ok(results.join("\n"))
}

// Advanced error handling with multiple catch blocks
slay network_operation(url tea) Result<tea, NetworkError> {
    shook {
        sus connection = await connect_to(url)?
        later connection.close()
        
        sus request = build_request()?
        sus response = await connection.send(request)?
        
        vibe_check response.status {
            200..299 => {
                sus body = await response.text()?
                damn Ok(body)
            },
            404 => {
                yikes NotFoundError { url: url.clone() }
            },
            500..599 => {
                yikes ServerError { 
                    status: response.status,
                    message: response.status_text()
                }
            },
            status => {
                yikes UnknownError { 
                    status: status,
                    url: url.clone()
                }
            }
        }
    } fam ConnectionError(err) {
        damn Err(NetworkError::Connection(err))
    } fam TimeoutError => {
        damn Err(NetworkError::Timeout)
    } fam ParseError { message } => {
        damn Err(NetworkError::Parse(message))
    } fam err => {
        damn Err(NetworkError::Unknown(err.to_string()))
    }
}

// Async function with complex concurrency patterns
async slay parallel_processing(items []tea) Result<[]ProcessedItem, Error> {
    sus results []ProcessedItem = []
    sus workers_count = min(items.len(), 10)
    sus semaphore = Semaphore::new(workers_count)
    
    sus tasks []Task<Result<ProcessedItem, Error>> = []
    
    bestie item in items {
        sus task = stan async {
            sus _permit = await semaphore.acquire()
            later semaphore.release()
            
            shook {
                sus processed = await process_item_async(item)
                damn Ok(processed)
            } fam err => {
                damn Err(Error::ProcessingFailed(item.clone(), err))
            }
        }
        tasks.push(task)
    }
    
    // Wait for all tasks with timeout
    ready {
        mood all_results := await join_all(tasks):
            bestie result in all_results {
                vibe_check result {
                    Ok(processed) => results.push(processed),
                    Err(err) => {
                        vibez.spillf("Processing error: {}", err)
                        damn Err(err)
                    }
                }
            }
        
        mood <- timeout(Duration::seconds(30)):
            damn Err(Error::Timeout)
    }
    
    damn Ok(results)
}

// Complex generic function with multiple constraints
slay transform_and_collect<T, U, F, C>(
    items []T, 
    transform F, 
    collector C
) Result<C::Output, C::Error>
where 
    F: Fn(T) -> Result<U, Error>,
    C: Collector<U>,
    T: Clone + Send,
    U: Send + Sync,
    C::Output: Send,
    C::Error: From<Error>
{
    sus transformed []U = []
    
    bestie item in items {
        vibe_check transform(item.clone()) {
            Ok(result) => transformed.push(result),
            Err(err) => damn Err(C::Error::from(err))
        }
    }
    
    damn collector.collect(transformed)
}

// Advanced lambda expressions with captures
slay create_processors(config ProcessorConfig) []Processor {
    sus processors []Processor = []
    sus shared_state = Arc::new(Mutex::new(ProcessorState::new()))
    
    bestie i in 0..config.processor_count {
        sus processor = |[shared_state, config] input| async move {
            sus state = shared_state.lock().await
            
            vibe_check config.mode {
                ProcessorMode::Fast => {
                    damn process_fast(input, &state)
                },
                ProcessorMode::Thorough => {
                    damn process_thorough(input, &state)
                },
                ProcessorMode::Custom { algorithm } => {
                    damn algorithm.process(input, &state)
                }
            }
        }
        
        processors.push(processor)
    }
    
    damn processors
}

// Union types with pattern matching
union Result<T, E> {
    Ok(T),
    Err(E)
}

union Option<T> {
    Some(T),
    None
}

union Message {
    Text { content: tea, sender: tea },
    Image { url: tea, alt_text: Option<tea> },
    File { path: tea, size: normie },
    System { level: LogLevel, message: tea }
}

// Pattern matching on complex union types
slay handle_message(msg Message) {
    vibe_check msg {
        Message::Text { content, sender } if sender == "admin" => {
            vibez.spillf("[ADMIN] {}", content)
        },
        
        Message::Text { content, sender } => {
            vibez.spillf("{}: {}", sender, content)
        },
        
        Message::Image { url, alt_text: Some(alt) } => {
            vibez.spillf("Image: {} ({})", url, alt)
        },
        
        Message::Image { url, alt_text: None } => {
            vibez.spillf("Image: {}", url)
        },
        
        Message::File { path, size } if size > 1024 * 1024 => {
            vibez.spillf("Large file: {} ({} MB)", path, size / 1024 / 1024)
        },
        
        Message::File { path, size } => {
            vibez.spillf("File: {} ({} bytes)", path, size)
        },
        
        Message::System { level: LogLevel::Error, message } => {
            vibez.spillf("ERROR: {}", message)
        },
        
        Message::System { level, message } => {
            vibez.spillf("[{}] {}", level, message)
        }
    }
}

// Advanced type aliases with constraints
be_like EventHandler<T: Clone + Send> = slay(T) -> Result<(), Error>
be_like AsyncEventHandler<T> = async slay(T) -> Result<(), Error> 
    where T: Clone + Send + Sync

be_like ProcessorFn<I, O> = slay(I) -> Future<Output = Result<O, Error>>
    where I: Send, O: Send

// Testing framework integration
test_start("Advanced parser features")

// Test generic functions
sus numbers = [1, 2, 3, 4, 5]
sus sorted = numbers.clone()
sorted.sort_by(|a, b| compare(a, b))
assert_eq_int(sorted[0], 1)

// Test pattern matching
sus shape = Circle { 
    center: Point { x: 0.0, y: 0.0 }, 
    radius: 15.0,
    color: Color::Blue 
}
sus description = analyze_shape(shape)
assert_eq_string(description, "Large circle at origin")

// Test async operations
sus items = ["item1", "item2", "item3"]
vibe_check await parallel_processing(items) {
    Ok(results) => {
        assert_eq_int(results.len(), 3)
        vibez.spill("Parallel processing successful")
    },
    Err(err) => {
        vibez.spillf("Parallel processing failed: {}", err)
        assert_true(cringe)  // Fail the test
    }
}

// Test complex pattern matching
sus test_data = [
    Value::String("hello"),
    Value::Number(42),
    Value::Array([Value::String("nested")]),
    Value::Null
]
vibe_check process_data(test_data) {
    Ok(result) => {
        assert_true(result.len() > 0)
        vibez.spill("Data processing successful")
    },
    Err(err) => {
        vibez.spillf("Data processing failed: {}", err)
        assert_true(cringe)
    }
}

print_test_summary()

vibez.spill("✅ Advanced CURSED parser features demonstration complete!")
vibez.spill("🚀 All language constructs parsed successfully:")
vibez.spill("   - Pattern matching with guards and destructuring")
vibez.spill("   - Generic types with complex constraints")
vibez.spill("   - Interface inheritance and composition")
vibez.spill("   - Advanced struct definitions with methods")
vibez.spill("   - Complex control flow constructs")
vibez.spill("   - Error handling and defer statements")
vibez.spill("   - Async/await and concurrency patterns")
vibez.spill("   - Union types and pattern matching")
vibez.spill("   - Lambda expressions with captures")
vibez.spill("   - Advanced type system features")
