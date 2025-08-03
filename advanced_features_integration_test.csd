yeet "testz"
yeet "channel_core"

fr fr Advanced Features Integration Test
fr fr Tests integration of pattern matching, defer statements, and select statements
fr fr This demonstrates real-world usage scenarios combining all advanced features

test_start("Advanced Features Integration - Complete System")

fr fr Integration Test 1: Pattern matching with defer cleanup
enum FileOperation {
    Read(tea)
    Write(tea, tea)
    Delete(tea)
}

struct FileManager {
    open_files drip
    max_files drip
}

impl FileManager {
    slay new() FileManager {
        damn FileManager { open_files: 0, max_files: 10 }
    }
    
    slay process_operation(operation FileOperation) tea {
        later {
            fr fr Always cleanup regardless of operation result
            if open_files > 0 {
                open_files = open_files - 1
            }
            vibez.spill("Cleaned up file operation")
        }
        
        open_files = open_files + 1
        
        sus result := match operation {
            FileOperation::Read(filename) when filename.len() > 0 -> {
                later {
                    vibez.spill(format("Closing read handle for {}", filename))
                }
                format("Read from {}", filename)
            }
            FileOperation::Write(filename, content) when filename.len() > 0 && content.len() > 0 -> {
                later {
                    vibez.spill(format("Syncing and closing write handle for {}", filename))
                }
                format("Wrote to {}: {}", filename, content)
            }
            FileOperation::Delete(filename) when filename.len() > 0 -> {
                later {
                    vibez.spill(format("Verified deletion of {}", filename))
                }
                format("Deleted {}", filename)
            }
            _ -> {
                later {
                    vibez.spill("Invalid operation cleanup")
                }
                "Invalid operation"
            }
        }
        
        damn result
    }
}

slay test_pattern_defer_integration() normie {
    sus manager := FileManager::new()
    
    sus result1 := manager.process_operation(FileOperation::Read("test.txt"))
    sus result2 := manager.process_operation(FileOperation::Write("output.txt", "hello world"))
    sus result3 := manager.process_operation(FileOperation::Delete("temp.txt"))
    
    assert_eq_string(result1, "Read from test.txt")
    assert_eq_string(result2, "Wrote to output.txt: hello world")
    assert_eq_string(result3, "Deleted temp.txt")
    
    fr fr All defers should have executed
    assert_eq_int(manager.open_files, 0)
    damn 1
}

assert_eq_int(test_pattern_defer_integration(), 1)

fr fr Integration Test 2: Select statements with pattern matching and defer
enum ChannelMessage {
    Task(drip, tea)
    Control(tea)
    Shutdown
}

struct MessageProcessor {
    processed_count drip
    error_count drip
    is_running lit
}

impl MessageProcessor {
    slay new() MessageProcessor {
        damn MessageProcessor { processed_count: 0, error_count: 0, is_running: based }
    }
    
    slay process_message(msg ChannelMessage) normie {
        later {
            fr fr Always log message processing completion
            vibez.spill("Message processing completed")
        }
        
        sus result := match msg {
            ChannelMessage::Task(id, data) when data.len() > 0 -> {
                later {
                    processed_count = processed_count + 1
                    vibez.spill(format("Task {} completed", id))
                }
                0  fr fr Success
            }
            ChannelMessage::Control(command) when command == "pause" -> {
                later {
                    is_running = cringe
                    vibez.spill("Processor paused")
                }
                0  fr fr Success
            }
            ChannelMessage::Control(command) when command == "resume" -> {
                later {
                    is_running = based
                    vibez.spill("Processor resumed")
                }
                0  fr fr Success
            }
            ChannelMessage::Shutdown -> {
                later {
                    is_running = cringe
                    vibez.spill("Processor shutdown")
                }
                1  fr fr Shutdown signal
            }
            _ -> {
                later {
                    error_count = error_count + 1
                    vibez.spill("Invalid message processed")
                }
                -1  fr fr Error
            }
        }
        
        damn result
    }
    
    slay run_processor() normie {
        reset_channel_system()
        init_channel_system()
        
        sus task_chan := make_channel(5, "ChannelMessage")
        sus control_chan := make_channel(2, "ChannelMessage")
        sus shutdown_chan := make_channel(1, "ChannelMessage")
        
        fr fr Send test messages
        channel_send(task_chan, ChannelMessage::Task(1, "process this"))
        channel_send(task_chan, ChannelMessage::Task(2, "process that"))
        channel_send(control_chan, ChannelMessage::Control("pause"))
        channel_send(control_chan, ChannelMessage::Control("resume"))
        channel_send(shutdown_chan, ChannelMessage::Shutdown)
        
        later {
            fr fr Cleanup all channels
            channel_close(task_chan)
            channel_close(control_chan)
            channel_close(shutdown_chan)
            vibez.spill("All channels cleaned up")
        }
        
        sus messages_processed := 0
        
        loop {
            ready {
                mood receive from task_chan -> msg {
                    sus result := process_message(msg)
                    if result >= 0 {
                        messages_processed = messages_processed + 1
                    }
                }
                mood receive from control_chan -> msg {
                    sus result := process_message(msg)
                    if result >= 0 {
                        messages_processed = messages_processed + 1
                    }
                }
                mood receive from shutdown_chan -> msg {
                    sus result := process_message(msg)
                    messages_processed = messages_processed + 1
                    if result == 1 {
                        break  fr fr Shutdown received
                    }
                }
                timeout 1000 {
                    break  fr fr Timeout
                }
            }
        }
        
        damn messages_processed
    }
}

slay test_select_pattern_defer_integration() normie {
    sus processor := MessageProcessor::new()
    sus processed := processor.run_processor()
    
    fr fr Should have processed task, control, and shutdown messages
    assert_true(processed >= 5)
    assert_eq_int(processor.processed_count, 2)  fr fr Two tasks
    assert_eq_int(processor.error_count, 0)
    assert_eq_lit(processor.is_running, cringe)  fr fr Should be shutdown
    damn 1
}

assert_eq_int(test_select_pattern_defer_integration(), 1)

fr fr Integration Test 3: Complex resource management with all features
struct Database {
    name tea
    is_connected lit
    transaction_count drip
}

struct Connection {
    db Database
    id drip
    is_active lit
}

enum DatabaseOperation {
    Query(tea)
    Update(tea, tea)
    Transaction([]tea)
    Backup(tea)
}

impl Database {
    slay connect(name tea) Database {
        damn Database { name: name, is_connected: based, transaction_count: 0 }
    }
    
    slay disconnect() {
        is_connected = cringe
        transaction_count = 0
    }
    
    slay get_connection() Connection {
        damn Connection { db: self, id: 42, is_active: based }
    }
}

impl Connection {
    slay close() {
        is_active = cringe
    }
    
    slay execute_operation(op DatabaseOperation) tea {
        later {
            fr fr Always close connection after operation
            close()
            vibez.spill("Connection closed")
        }
        
        if !is_active {
            damn "Connection not active"
        }
        
        sus result := match op {
            DatabaseOperation::Query(sql) when sql.starts_with("SELECT") -> {
                later {
                    vibez.spill("Query result set cleaned up")
                }
                format("Query executed: {}", sql)
            }
            DatabaseOperation::Update(table, data) when table.len() > 0 -> {
                later {
                    db.transaction_count = db.transaction_count + 1
                    vibez.spill("Update transaction committed")
                }
                format("Updated {} with {}", table, data)
            }
            DatabaseOperation::Transaction(queries) when len(queries) > 0 -> {
                later {
                    db.transaction_count = db.transaction_count + 1
                    vibez.spill("Transaction batch completed")
                }
                
                sus transaction_result := ""
                for query in queries {
                    later {
                        vibez.spill(format("Query executed: {}", query))
                    }
                    transaction_result = transaction_result + query + "; "
                }
                
                format("Transaction executed: {}", transaction_result)
            }
            DatabaseOperation::Backup(path) when path.len() > 0 -> {
                later {
                    vibez.spill(format("Backup saved to {}", path))
                }
                format("Backup created at {}", path)
            }
            _ -> {
                later {
                    vibez.spill("Invalid operation cleanup")
                }
                "Invalid operation"
            }
        }
        
        damn result
    }
}

slay test_complex_resource_management() normie {
    reset_channel_system()
    init_channel_system()
    
    sus operation_chan := make_channel(10, "DatabaseOperation")
    sus result_chan := make_channel(10, "tea")
    sus error_chan := make_channel(5, "tea")
    
    later {
        fr fr Cleanup all channels
        channel_close(operation_chan)
        channel_close(result_chan)
        channel_close(error_chan)
        vibez.spill("All database channels cleaned up")
    }
    
    sus db := Database::connect("test_database")
    later {
        db.disconnect()
        vibez.spill("Database disconnected")
    }
    
    fr fr Send operations to channel
    channel_send(operation_chan, DatabaseOperation::Query("SELECT * FROM users"))
    channel_send(operation_chan, DatabaseOperation::Update("users", "name='John'"))
    channel_send(operation_chan, DatabaseOperation::Transaction(["BEGIN", "INSERT INTO log", "COMMIT"]))
    channel_send(operation_chan, DatabaseOperation::Backup("/tmp/backup.sql"))
    
    fr fr Process operations with select
    sus results := []
    sus operations_processed := 0
    
    for i in 0..10 {
        ready {
            mood receive from operation_chan -> op {
                sus conn := db.get_connection()
                
                later {
                    fr fr Ensure connection cleanup even on errors
                    if conn.is_active {
                        conn.close()
                    }
                }
                
                sus result := conn.execute_operation(op)
                
                ready {
                    mood send result to result_chan {
                        operations_processed = operations_processed + 1
                    }
                    timeout 100 {
                        channel_send(error_chan, "Failed to send result")
                    }
                }
            }
            mood receive from result_chan -> result {
                results = append(results, result)
            }
            mood receive from error_chan -> error {
                vibez.spill(format("Error: {}", error))
            }
            timeout 500 {
                break
            }
        }
    }
    
    fr fr Verify all operations were processed
    assert_true(operations_processed >= 4)
    assert_true(len(results) >= 4)
    assert_true(db.transaction_count >= 2)
    assert_eq_lit(db.is_connected, cringe)  fr fr Should be disconnected by defer
    damn 1
}

assert_eq_int(test_complex_resource_management(), 1)

fr fr Integration Test 4: Error handling with pattern matching and select
enum Result[T, E] {
    Ok(T)
    Err(E)
}

enum ProcessingError {
    InvalidInput(tea)
    NetworkError(drip)
    TimeoutError
    InternalError(tea)
}

slay test_error_handling_integration() normie {
    reset_channel_system()
    init_channel_system()
    
    sus input_chan := make_channel(5, "tea")
    sus result_chan := make_channel(5, "Result")
    sus error_chan := make_channel(5, "ProcessingError")
    
    later {
        fr fr Cleanup channels
        channel_close(input_chan)
        channel_close(result_chan)
        channel_close(error_chan)
        vibez.spill("Error handling channels cleaned up")
    }
    
    fr fr Process function with comprehensive error handling
    slay process_input(input tea) Result[tea, ProcessingError] {
        later {
            vibez.spill(format("Processing completed for input: {}", input))
        }
        
        sus result := match input {
            s when s.len() == 0 -> Result::Err(ProcessingError::InvalidInput("Empty input"))
            s when s == "error" -> Result::Err(ProcessingError::InternalError("Test error"))
            s when s == "timeout" -> Result::Err(ProcessingError::TimeoutError)
            s when s.starts_with("net") -> Result::Err(ProcessingError::NetworkError(500))
            s when s.len() > 100 -> Result::Err(ProcessingError::InvalidInput("Input too long"))
            s -> Result::Ok(format("Processed: {}", s))
        }
        
        damn result
    }
    
    fr fr Send test inputs
    channel_send(input_chan, "valid_input")
    channel_send(input_chan, "error")
    channel_send(input_chan, "")
    channel_send(input_chan, "network_fail")
    channel_send(input_chan, "timeout")
    channel_send(input_chan, "another_valid")
    
    sus successful_results := []
    sus errors := []
    sus processed_count := 0
    
    for i in 0..20 {
        ready {
            mood receive from input_chan -> input {
                sus result := process_input(input)
                processed_count = processed_count + 1
                
                sus outcome := match result {
                    Result::Ok(value) -> {
                        later {
                            vibez.spill(format("Success result logged: {}", value))
                        }
                        ready {
                            mood send result to result_chan {
                                fr fr Success
                            }
                            timeout 50 {
                                fr fr Could not send result
                            }
                        }
                        "ok"
                    }
                    Result::Err(error) -> {
                        later {
                            vibez.spill("Error result logged")
                        }
                        ready {
                            mood send error to error_chan {
                                fr fr Error sent
                            }
                            timeout 50 {
                                fr fr Could not send error
                            }
                        }
                        "error"
                    }
                }
            }
            mood receive from result_chan -> success_result {
                sus value := match success_result {
                    Result::Ok(v) -> v
                    _ -> "unexpected"
                }
                successful_results = append(successful_results, value)
            }
            mood receive from error_chan -> error {
                sus error_msg := match error {
                    ProcessingError::InvalidInput(msg) -> format("Invalid: {}", msg)
                    ProcessingError::NetworkError(code) -> format("Network: {}", code)
                    ProcessingError::TimeoutError -> "Timeout occurred"
                    ProcessingError::InternalError(msg) -> format("Internal: {}", msg)
                }
                errors = append(errors, error_msg)
            }
            timeout 200 {
                break
            }
        }
    }
    
    fr fr Verify error handling worked correctly
    assert_eq_int(processed_count, 6)  fr fr All inputs processed
    assert_eq_int(len(successful_results), 2)  fr fr Two valid inputs
    assert_eq_int(len(errors), 4)  fr fr Four error cases
    damn 1
}

assert_eq_int(test_error_handling_integration(), 1)

print_test_summary()
