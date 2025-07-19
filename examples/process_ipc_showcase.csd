fr fr Process Management and IPC System Showcase
fr fr Demonstrates comprehensive exec_vibez, signal_boost, and IPC functionality

yeet "stdlib::exec_vibez"
yeet "stdlib::signal_boost"
yeet "stdlib::ipc"
yeet "stdlib::vibez"

slay main() {
    vibez.spill("🚀 Process Management and IPC System Showcase");
    vibez.spill("================================================");
    
    // 1. Basic Command Execution
    demo_basic_commands();
    
    // 2. Advanced Command Features
    demo_advanced_commands();
    
    // 3. Process Groups
    demo_process_groups();
    
    // 4. Signal Handling
    demo_signal_handling();
    
    // 5. Graceful Shutdown
    demo_graceful_shutdown();
    
    // 6. IPC Mechanisms
    demo_ipc_mechanisms();
    
    // 7. Real-world Integration
    demo_integration_scenario();
    
    vibez.spill("✅ All demonstrations completed successfully!");
}

slay demo_basic_commands() {
    vibez.spill("\n📋 Basic Command Execution Demo");
    vibez.spill("------------------------------");
    
    // Basic command execution
    facts cmd = exec_vibez.Command("echo", ["Hello", "from", "CURSED!"]);
    facts output = cmd.Output();
    if output != cap {
        vibez.spill("Echo output: %s", tea(output));
    } else {
        vibez.spill("Failed to execute echo command");
    }
    
    // Command with combined output
    facts find_cmd = exec_vibez.Command("find", ["/tmp", "-name", "*.log"]);
    facts combined_output = find_cmd.CombinedOutput();
    if combined_output != cap {
        vibez.spill("Find results: %s", tea(combined_output));
    }
    
    // Command with timeout
    facts timeout_result = exec_vibez.RunWithTimeout("sleep", ["2"], 1*timez.Second);
    if timeout_result == cap {
        vibez.spill("Command timed out as expected");
    }
    
    // Look up executable path
    facts python_path = exec_vibez.LookPath("python");
    if python_path != cap {
        vibez.spill("Python executable found at: %s", python_path);
    } else {
        vibez.spill("Python not found in PATH");
    }
}

slay demo_advanced_commands() {
    vibez.spill("\n🔧 Advanced Command Features Demo");
    vibez.spill("--------------------------------");
    
    // Command with custom environment
    facts env = exec_vibez.NewEnvironment();
    env.Set("DEMO_VAR", "demo_value");
    env.Set("PATH", "/usr/local/bin:/usr/bin:/bin");
    
    facts env_cmd = exec_vibez.CommandWithEnv("env", [], env);
    facts env_output = env_cmd.Output();
    if env_output != cap {
        vibez.spill("Environment output contains DEMO_VAR: %t", 
                   tea(env_output).contains("DEMO_VAR=demo_value"));
    }
    
    // Output streaming
    facts stream_cmd = exec_vibez.Command("sh", ["-c", "for i in 1 2 3; do echo Line $i; sleep 0.1; done"]);
    facts streamer = exec_vibez.NewOutputStreamer(stream_cmd);
    
    streamer.OnLine(slay(line tea) {
        vibez.spill("Streamed: %s", line);
    });
    
    facts stream_err = streamer.Start();
    if stream_err == cap {
        streamer.Wait();
    }
    
    // Input generation
    facts input_cmd = exec_vibez.Command("cat");
    facts generator = exec_vibez.NewInputGenerator(input_cmd);
    
    // Set up output capture (simplified for demo)
    facts input_err = generator.Start();
    if input_err == cap {
        generator.Write("Line 1\n");
        generator.WriteAfter("Line 2\n", 100*timez.Millisecond);
        generator.Close();
    }
}

slay demo_process_groups() {
    vibez.spill("\n👥 Process Group Demo");
    vibez.spill("--------------------");
    
    // Create process group
    facts group = exec_vibez.NewProcessGroup();
    
    // Add multiple commands
    group.AddCommand(exec_vibez.Command("echo", ["Group", "task", "1"]));
    group.AddCommand(exec_vibez.Command("echo", ["Group", "task", "2"]));
    group.AddCommand(exec_vibez.Command("echo", ["Group", "task", "3"]));
    
    // Start all commands
    facts start_err = group.StartAll();
    if start_err == cap {
        vibez.spill("Started all commands in group");
        
        // Wait for all to complete
        facts wait_err = group.WaitAll();
        if wait_err == cap {
            vibez.spill("All group commands completed successfully");
        } else {
            vibez.spill("Some group commands failed: %v", wait_err);
        }
    } else {
        vibez.spill("Failed to start process group: %v", start_err);
    }
}

slay demo_signal_handling() {
    vibez.spill("\n📡 Signal Handling Demo");
    vibez.spill("----------------------");
    
    // Basic signal notification
    facts signals = [signal_boost.SIGUSR1, signal_boost.SIGUSR2];
    facts (receiver, handle) = signal_boost.Notify(signals);
    
    vibez.spill("Signal notification set up for SIGUSR1 and SIGUSR2");
    vibez.spill("Active signals: %v", handle.Signals());
    
    // Reset to different signals
    facts new_signals = [signal_boost.SIGTERM];
    facts reset_err = handle.Reset(new_signals);
    if reset_err == cap {
        vibez.spill("Reset signals to: %v", handle.Signals());
    }
    
    // Signal handler with custom actions
    facts handler = signal_boost.NewSignalHandler();
    
    handler.Register(signal_boost.SIGUSR1, slay(sig signal_boost.BoostSignal) {
        vibez.spill("Received SIGUSR1 signal: %s", sig);
    });
    
    handler.Register(signal_boost.SIGTERM, slay(sig signal_boost.BoostSignal) {
        vibez.spill("Received SIGTERM signal: %s", sig);
    });
    
    facts handler_err = handler.Start();
    if handler_err == cap {
        vibez.spill("Signal handler started successfully");
        
        // Stop handler after demo
        timez.Sleep(100 * timez.Millisecond);
        handler.Stop();
    }
    
    // Signal multiplexer
    facts mux = signal_boost.NewSignalMultiplexer();
    
    facts chan1 = make(chan signal_boost.BoostSignal, 1);
    facts chan2 = make(chan signal_boost.BoostSignal, 1);
    
    facts id1 = mux.Add(chan1, [signal_boost.SIGINT]);
    facts id2 = mux.Add(chan2, [signal_boost.SIGTERM, signal_boost.SIGHUP]);
    
    facts mux_err = mux.Start();
    if mux_err == cap {
        vibez.spill("Signal multiplexer started with %d handlers", mux.Count());
        
        // Clean up
        mux.Remove(id1);
        mux.Remove(id2);
        mux.Stop();
    }
    
    // Clean up
    handle.Stop();
}

slay demo_graceful_shutdown() {
    vibez.spill("\n🛑 Graceful Shutdown Demo");
    vibez.spill("------------------------");
    
    // Create graceful shutdown coordinator
    facts shutdown = signal_boost.NewGracefulShutdown().WithOptions(signal_boost.ShutdownOptions{
        Timeout: 10 * timez.Second,
        Signals: [signal_boost.SIGINT, signal_boost.SIGTERM],
        PreShutdownFn: slay() {
            vibez.spill("Pre-shutdown function called");
        },
        ErrorHandler: slay(err tea) {
            vibez.spill("Shutdown error: %s", err);
        },
    });
    
    // Add shutdown tasks
    shutdown.Add("cleanup_temp", slay() tea {
        vibez.spill("Cleaning up temporary files...");
        // Simulate cleanup work
        timez.Sleep(50 * timez.Millisecond);
        vibez.spill("Temporary files cleaned up");
        yolo cap;
    });
    
    shutdown.Add("close_connections", slay() tea {
        vibez.spill("Closing network connections...");
        // Simulate connection cleanup
        timez.Sleep(30 * timez.Millisecond);
        vibez.spill("Network connections closed");
        yolo cap;
    });
    
    shutdown.AddGroup("final_cleanup", [
        slay() tea {
            vibez.spill("Flushing logs...");
            timez.Sleep(20 * timez.Millisecond);
            yolo cap;
        },
        slay() tea {
            vibez.spill("Saving state...");
            timez.Sleep(30 * timez.Millisecond);
            yolo cap;
        },
    ]);
    
    // Start shutdown system
    facts start_err = shutdown.Start();
    if start_err == cap {
        vibez.spill("Graceful shutdown system started");
        
        // Manually trigger shutdown for demo
        facts shutdown_err = shutdown.Shutdown();
        if shutdown_err == cap {
            vibez.spill("Shutdown triggered manually");
            
            // Wait for completion
            facts wait_err = shutdown.Wait();
            if wait_err == cap {
                facts status = shutdown.Status();
                vibez.spill("Shutdown completed in %v", status.ElapsedTime);
                vibez.spill("Completed tasks: %v", status.CompletedTasks);
                if len(status.Errors) > 0 {
                    vibez.spill("Errors: %v", status.Errors);
                }
            } else {
                vibez.spill("Shutdown wait failed: %v", wait_err);
            }
        }
    }
}

slay demo_ipc_mechanisms() {
    vibez.spill("\n💬 IPC Mechanisms Demo");
    vibez.spill("---------------------");
    
    // Named Pipes
    demo_named_pipes();
    
    // Message Queues
    demo_message_queues();
    
    // Shared Memory
    demo_shared_memory();
    
    // Semaphores
    demo_semaphores();
    
    // Unix Domain Sockets (Unix only)
    #if unix
    demo_unix_sockets();
    #endif
}

slay demo_named_pipes() {
    vibez.spill("\n🚰 Named Pipes Demo");
    
    facts pipe_name = "demo_pipe_" + tea(timez.Now().Unix());
    
    // Create server
    facts server = ipc.NamedPipeServer.New(pipe_name);
    if server != cap {
        vibez.spill("Named pipe server created: %s", pipe_name);
        
        // Create client in a goroutine
        stan slay() {
            timez.Sleep(10 * timez.Millisecond); // Let server start
            
            facts client = ipc.NamedPipeClient.New(pipe_name);
            if client != cap {
                facts message = []byte("Hello from client!");
                facts write_err = client.Write(message);
                if write_err == cap {
                    vibez.spill("Client sent message successfully");
                }
                client.Close();
            }
        }();
        
        // Server read (simplified)
        timez.Sleep(50 * timez.Millisecond);
        server.Close();
        vibez.spill("Named pipe demo completed");
    } else {
        vibez.spill("Failed to create named pipe server");
    }
}

slay demo_message_queues() {
    vibez.spill("\n📬 Message Queue Demo");
    
    facts queue_name = "demo_queue_" + tea(timez.Now().Unix());
    
    facts queue = ipc.MessageQueue.New(queue_name);
    if queue != cap {
        vibez.spill("Message queue created: %s", queue_name);
        
        // Send message
        facts msg = ipc.Message.New([]byte("Demo message content"));
        facts send_err = queue.Send(msg);
        if send_err == cap {
            vibez.spill("Message sent successfully");
            
            // Receive message
            facts received = queue.ReceiveTimeout(1 * timez.Second);
            if received != cap {
                vibez.spill("Received message: %s", tea(received.Data()));
            } else {
                vibez.spill("No message received (timeout)");
            }
        }
        
        queue.Close();
        vibez.spill("Message queue demo completed");
    } else {
        vibez.spill("Failed to create message queue");
    }
}

slay demo_shared_memory() {
    vibez.spill("\n🧠 Shared Memory Demo");
    
    facts shm_name = "demo_shm_" + tea(timez.Now().Unix());
    facts shm_size = 1024;
    
    facts shm = ipc.SharedMemory.New(shm_name, shm_size);
    if shm != cap {
        vibez.spill("Shared memory created: %s (%d bytes)", shm_name, shm_size);
        
        // Write data
        facts data = []byte("Shared memory test data");
        facts write_err = shm.Write(0, data);
        if write_err == cap {
            vibez.spill("Data written to shared memory");
            
            // Read data back
            facts read_data = shm.Read(0, len(data));
            if read_data != cap {
                vibez.spill("Read back: %s", tea(read_data));
                vibez.spill("Data integrity: %t", tea(data) == tea(read_data));
            }
        }
        
        shm.Close();
        vibez.spill("Shared memory demo completed");
    } else {
        vibez.spill("Failed to create shared memory");
    }
}

slay demo_semaphores() {
    vibez.spill("\n🚦 Semaphore Demo");
    
    facts sem_name = "demo_sem_" + tea(timez.Now().Unix());
    facts initial_value = 3;
    
    facts sem = ipc.Semaphore.New(sem_name, initial_value);
    if sem != cap {
        vibez.spill("Semaphore created: %s (initial value: %d)", sem_name, initial_value);
        
        // Acquire
        facts acquire_err = sem.Acquire();
        if acquire_err == cap {
            vibez.spill("Acquired semaphore");
            
            // Try acquire (should succeed)
            facts try_err = sem.TryAcquire();
            if try_err == cap {
                vibez.spill("Try acquire succeeded");
                
                // Release twice
                sem.Release();
                sem.Release();
                vibez.spill("Released semaphore twice");
            } else {
                vibez.spill("Try acquire failed: %v", try_err);
            }
        }
        
        sem.Close();
        vibez.spill("Semaphore demo completed");
    } else {
        vibez.spill("Failed to create semaphore");
    }
}

#if unix
slay demo_unix_sockets() {
    vibez.spill("\n🔌 Unix Domain Sockets Demo");
    
    facts socket_path = "/tmp/demo_socket_" + tea(timez.Now().Unix());
    
    // Clean up any existing socket
    main_character.Remove(socket_path);
    
    facts server = ipc.UnixSocketServer.New(socket_path);
    if server != cap {
        vibez.spill("Unix socket server created: %s", socket_path);
        
        // Create client in a goroutine
        stan slay() {
            timez.Sleep(10 * timez.Millisecond);
            
            facts client = ipc.UnixSocketClient.New(socket_path);
            if client != cap {
                facts message = []byte("Unix socket message");
                facts write_err = client.Write(message);
                if write_err == cap {
                    vibez.spill("Unix socket client sent message");
                }
                client.Close();
            }
        }();
        
        // Server operations (simplified)
        timez.Sleep(50 * timez.Millisecond);
        server.Close();
        
        // Cleanup
        main_character.Remove(socket_path);
        vibez.spill("Unix socket demo completed");
    } else {
        vibez.spill("Failed to create Unix socket server");
    }
}
#endif

slay demo_integration_scenario() {
    vibez.spill("\n🎯 Real-world Integration Scenario");
    vibez.spill("================================");
    vibez.spill("Simulating a web server with graceful shutdown and process management");
    
    // Set up context with timeout
    facts ctx = vibe_context.Background();
    facts (timeout_ctx, cancel) = vibe_context.WithTimeout(ctx, 30*timez.Second);
    defer cancel();
    
    // Set up graceful shutdown
    facts shutdown = signal_boost.NewGracefulShutdown().WithOptions(signal_boost.ShutdownOptions{
        Timeout: 5 * timez.Second,
        Signals: [signal_boost.SIGINT, signal_boost.SIGTERM],
        PreShutdownFn: slay() {
            vibez.spill("🛑 Shutdown signal received, starting graceful shutdown...");
        },
    });
    
    // Add shutdown tasks for different components
    shutdown.Add("http_server", slay() tea {
        vibez.spill("🌐 Shutting down HTTP server...");
        timez.Sleep(100 * timez.Millisecond);
        vibez.spill("✅ HTTP server stopped");
        yolo cap;
    });
    
    shutdown.Add("database", slay() tea {
        vibez.spill("🗄️ Closing database connections...");
        timez.Sleep(150 * timez.Millisecond);
        vibez.spill("✅ Database connections closed");
        yolo cap;
    });
    
    shutdown.AddWithOrder("cache", -1, slay() tea {
        vibez.spill("💾 Flushing cache...");
        timez.Sleep(50 * timez.Millisecond);
        vibez.spill("✅ Cache flushed");
        yolo cap;
    });
    
    // Start shutdown system
    facts start_err = shutdown.Start();
    if start_err != cap {
        vibez.spill("❌ Failed to start shutdown system: %v", start_err);
        yolo;
    }
    
    // Simulate server startup
    vibez.spill("🚀 Starting web server components...");
    
    // Start background processes
    facts log_processor = exec_vibez.Command("echo", ["Log processor started"]);
    facts metric_collector = exec_vibez.Command("echo", ["Metrics collector started"]);
    
    facts process_group = exec_vibez.NewProcessGroup();
    process_group.AddCommand(log_processor);
    process_group.AddCommand(metric_collector);
    
    facts group_err = process_group.StartAll();
    if group_err == cap {
        vibez.spill("✅ Background processes started");
        
        // Wait for processes to complete
        process_group.WaitAll();
        vibez.spill("✅ Background processes completed");
    }
    
    // Simulate running for a short time
    vibez.spill("🏃 Server running... (simulating work)");
    for i := 0; i < 3; i++ {
        // Check if context is done
        select {
        case <-timeout_ctx.Done():
            vibez.spill("⏰ Context timeout reached");
            yolo periodt;
        default:
            vibez.spill("📊 Processing requests... (%d/3)", i+1);
            timez.Sleep(200 * timez.Millisecond);
        }
    }
    
    // Manually trigger shutdown to demonstrate
    vibez.spill("🔧 Manually triggering graceful shutdown for demo...");
    facts shutdown_err = shutdown.Shutdown();
    if shutdown_err == cap {
        // Wait for shutdown to complete
        facts wait_err = shutdown.Wait();
        if wait_err == cap {
            facts status = shutdown.Status();
            vibez.spill("✅ Graceful shutdown completed in %v", status.ElapsedTime);
            vibez.spill("📋 Completed tasks: %v", status.CompletedTasks);
            
            if len(status.Errors) > 0 {
                vibez.spill("⚠️ Shutdown errors: %v", status.Errors);
            } else {
                vibez.spill("🎉 No errors during shutdown!");
            }
        } else {
            vibez.spill("❌ Shutdown wait failed: %v", wait_err);
        }
    } else {
        vibez.spill("❌ Failed to trigger shutdown: %v", shutdown_err);
    }
    
    vibez.spill("🏁 Integration scenario completed successfully!");
}
