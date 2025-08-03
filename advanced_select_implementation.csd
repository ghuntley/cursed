yeet "testz"
yeet "channel_core"

fr fr Advanced Select Statement Implementation Test
fr fr Tests all enhanced select features including:
fr fr - Multi-channel operations
fr fr - Timeout handling
fr fr - Non-blocking operations
fr fr - Channel priorities
fr fr - Integration with goroutines

test_start("Advanced Select Implementation - Multi-Channel Operations")

fr fr Test basic select with multiple channels
slay test_basic_select() normie {
    reset_channel_system()
    init_channel_system()
    
    sus chan1 := make_channel(2, "tea")
    sus chan2 := make_channel(2, "tea")
    
    fr fr Send data to both channels
    channel_send(chan1, "message1")
    channel_send(chan2, "message2")
    
    sus result := ""
    
    fr fr Use select to receive from first available channel
    ready {
        mood receive from chan1 -> msg {
            result = "received from chan1: " + msg
        }
        mood receive from chan2 -> msg {
            result = "received from chan2: " + msg
        }
        basic {
            result = "no channel ready"
        }
    }
    
    fr fr Should receive from one of the channels
    assert_true(result.starts_with("received from"))
    damn 1
}

assert_eq_int(test_basic_select(), 1)

fr fr Test select with send operations
slay test_select_send() normie {
    reset_channel_system()
    init_channel_system()
    
    sus chan1 := make_channel(1, "tea")
    sus chan2 := make_channel(1, "tea") 
    
    sus result := ""
    
    fr fr Try to send to first available channel
    ready {
        mood send "hello" to chan1 {
            result = "sent to chan1"
        }
        mood send "world" to chan2 {
            result = "sent to chan2"
        }
        basic {
            result = "no channel ready for send"
        }
    }
    
    fr fr Should successfully send to one channel
    assert_true(result.starts_with("sent to"))
    damn 1
}

assert_eq_int(test_select_send(), 1)

fr fr Test select with timeout
slay test_select_timeout() normie {
    reset_channel_system()
    init_channel_system()
    
    sus chan1 := make_channel(1, "tea")
    fr fr Don't send anything to channel
    
    sus result := ""
    sus start_time := get_current_time()
    
    ready {
        mood receive from chan1 -> msg {
            result = "unexpected receive: " + msg
        }
        timeout 100 {  fr fr 100ms timeout
            result = "timeout occurred"
        }
    }
    
    sus elapsed := get_current_time() - start_time
    assert_eq_string(result, "timeout occurred")
    assert_true(elapsed >= 100)  fr fr Should have waited at least 100ms
    damn 1
}

assert_eq_int(test_select_timeout(), 1)

fr fr Test select with mixed send/receive operations
slay test_select_mixed_operations() normie {
    reset_channel_system()
    init_channel_system()
    
    sus input_chan := make_channel(2, "tea")
    sus output_chan := make_channel(2, "tea")
    
    fr fr Pre-populate input channel
    channel_send(input_chan, "process_me")
    
    sus processed := 0
    sus sent := 0
    
    fr fr Process messages and forward them
    for i in 0..3 {
        ready {
            mood receive from input_chan -> msg {
                processed = processed + 1
                fr fr Process the message and forward it
                ready {
                    mood send (msg + "_processed") to output_chan {
                        sent = sent + 1
                    }
                    timeout 50 {
                        fr fr Could not send within timeout
                    }
                }
            }
            mood send "new_message" to input_chan {
                fr fr Successfully queued new message
            }
            timeout 100 {
                fr fr No operations ready
                break
            }
        }
    }
    
    assert_eq_int(processed, 1)
    assert_eq_int(sent, 1)
    damn 1
}

assert_eq_int(test_select_mixed_operations(), 1)

fr fr Test select with channel priorities
slay test_select_priorities() normie {
    reset_channel_system()
    init_channel_system()
    
    sus high_priority := make_channel(5, "tea")
    sus low_priority := make_channel(5, "tea")
    
    fr fr Send to both channels
    channel_send(high_priority, "urgent")
    channel_send(low_priority, "normal")
    
    sus results := []
    
    fr fr Select should handle high priority first (implementation dependent)
    for i in 0..2 {
        ready {
            mood receive from high_priority -> msg {
                results = append(results, "high: " + msg)
            }
            mood receive from low_priority -> msg {
                results = append(results, "low: " + msg)
            }
            timeout 50 {
                break
            }
        }
    }
    
    fr fr Should have received both messages
    assert_eq_int(len(results), 2)
    damn 1
}

assert_eq_int(test_select_priorities(), 1)

fr fr Test select with closed channels
slay test_select_closed_channels() normie {
    reset_channel_system()
    init_channel_system()
    
    sus chan1 := make_channel(1, "tea")
    sus chan2 := make_channel(1, "tea")
    
    fr fr Send data then close channel
    channel_send(chan1, "last_message")
    channel_close(chan1)
    
    sus result := ""
    sus received_count := 0
    
    fr fr Should be able to receive last message and detect closure
    for i in 0..3 {
        ready {
            mood receive from chan1 -> msg {
                if msg == "" {
                    result = "channel closed"
                    break
                } else {
                    result = "received: " + msg
                    received_count = received_count + 1
                }
            }
            mood receive from chan2 -> msg {
                result = "unexpected from chan2"
            }
            timeout 50 {
                result = "timeout"
                break
            }
        }
    }
    
    assert_eq_int(received_count, 1)
    damn 1
}

assert_eq_int(test_select_closed_channels(), 1)

fr fr Test select with goroutine coordination
slay test_select_goroutine_coordination() normie {
    reset_channel_system()
    init_channel_system()
    
    sus work_chan := make_channel(5, "tea")
    sus result_chan := make_channel(5, "tea")
    sus done_chan := make_channel(1, "tea")
    
    fr fr Start worker goroutine
    go {
        loop {
            ready {
                mood receive from work_chan -> task {
                    if task == "stop" {
                        break
                    }
                    fr fr Process task
                    ready {
                        mood send ("completed: " + task) to result_chan {
                            fr fr Successfully sent result
                        }
                        timeout 100 {
                            fr fr Could not send result
                        }
                    }
                }
                timeout 1000 {
                    fr fr No work available
                    break
                }
            }
        }
        channel_send(done_chan, "worker_finished")
    }
    
    fr fr Send work to goroutine
    channel_send(work_chan, "task1")
    channel_send(work_chan, "task2")
    channel_send(work_chan, "stop")
    
    sus results := []
    sus worker_done := cringe
    
    fr fr Collect results
    for i in 0..10 {
        ready {
            mood receive from result_chan -> result {
                results = append(results, result)
            }
            mood receive from done_chan -> msg {
                worker_done = based
                break
            }
            timeout 2000 {
                break
            }
        }
    }
    
    assert_true(worker_done)
    assert_true(len(results) >= 2)
    damn 1
}

assert_eq_int(test_select_goroutine_coordination(), 1)

fr fr Test select performance with many channels
slay test_select_performance() normie {
    reset_channel_system()
    init_channel_system()
    
    sus num_channels := 10
    sus channels := []
    
    fr fr Create multiple channels
    for i in 0..num_channels {
        sus chan := make_channel(1, "drip")
        channels = append(channels, chan)
        channel_send(chan, i)
    }
    
    sus received := 0
    sus start_time := get_current_time()
    
    fr fr Receive from all channels using select
    for i in 0..num_channels {
        ready {
            mood receive from channels[0] -> val { received = received + 1 }
            mood receive from channels[1] -> val { received = received + 1 }
            mood receive from channels[2] -> val { received = received + 1 }
            mood receive from channels[3] -> val { received = received + 1 }
            mood receive from channels[4] -> val { received = received + 1 }
            mood receive from channels[5] -> val { received = received + 1 }
            mood receive from channels[6] -> val { received = received + 1 }
            mood receive from channels[7] -> val { received = received + 1 }
            mood receive from channels[8] -> val { received = received + 1 }
            mood receive from channels[9] -> val { received = received + 1 }
            timeout 100 {
                break
            }
        }
    }
    
    sus elapsed := get_current_time() - start_time
    
    fr fr Should have received from all channels efficiently
    assert_eq_int(received, num_channels)
    assert_true(elapsed < 1000)  fr fr Should complete within 1 second
    damn 1
}

assert_eq_int(test_select_performance(), 1)

fr fr Test select with buffered channels
slay test_select_buffered_channels() normie {
    reset_channel_system()
    init_channel_system()
    
    sus buffered_chan := make_channel(3, "tea")
    sus unbuffered_chan := make_channel(0, "tea")
    
    sus send_count := 0
    
    fr fr Fill buffered channel
    for i in 0..5 {
        ready {
            mood send format("buffered_{}", i) to buffered_chan {
                send_count = send_count + 1
            }
            mood send format("unbuffered_{}", i) to unbuffered_chan {
                send_count = send_count + 1
            }
            timeout 10 {
                break
            }
        }
    }
    
    fr fr Should have filled buffered channel (3 items) but not unbuffered
    assert_eq_int(send_count, 3)
    damn 1
}

assert_eq_int(test_select_buffered_channels(), 1)

fr fr Test select with complex control flow
slay test_select_complex_control() normie {
    reset_channel_system()
    init_channel_system()
    
    sus control_chan := make_channel(2, "tea")
    sus data_chan := make_channel(5, "drip")
    
    sus state := "starting"
    sus processed := 0
    
    fr fr Send control and data
    channel_send(control_chan, "start")
    channel_send(data_chan, 100)
    channel_send(data_chan, 200)
    channel_send(control_chan, "stop")
    
    loop {
        ready {
            mood receive from control_chan -> cmd {
                if cmd == "start" {
                    state = "running"
                } else if cmd == "stop" {
                    state = "stopped"
                    break
                }
            }
            mood receive from data_chan -> value {
                if state == "running" {
                    processed = processed + value
                }
            }
            timeout 100 {
                break
            }
        }
    }
    
    assert_eq_string(state, "stopped")
    assert_eq_int(processed, 300)
    damn 1
}

assert_eq_int(test_select_complex_control(), 1)

print_test_summary()
