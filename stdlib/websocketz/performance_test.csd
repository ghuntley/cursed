yeet "testz"
yeet "websocketz"
yeet "stringz"
yeet "timez"

fr fr ========================================
fr fr CURSED WebSocket Performance Tests
fr fr Stress Testing & Benchmarking Suite
fr fr ========================================

slay performance_test_room_scaling() {
    test_start("WebSocket Room Scaling Performance")
    
    fr fr Test room creation performance
    sus start_time normie = 0  fr fr Would use actual timestamp
    sus rooms WebSocketRoom[10]
    
    bestie i normie = 0; i < 10; i++ {
        sus room_id tea = "room_" + stringz.int_to_string(i)
        sus room_name tea = "Performance Test Room " + stringz.int_to_string(i)
        rooms[i] = ws_room_create(room_id, room_name)
        assert_eq_string(rooms[i].room_id, room_id)
        assert_eq_string(rooms[i].name, room_name)
    }
    
    fr fr Add many connections to each room
    bestie room_idx normie = 0; room_idx < 10; room_idx++ {
        bestie conn_idx normie = 0; conn_idx < 25; conn_idx++ {
            sus connection_id normie = (room_idx * 100) + conn_idx + 1000
            sus join_result lit = ws_room_join(&rooms[room_idx], connection_id)
            assert_eq_lit(join_result, based)
        }
        assert_eq_int(ws_room_get_connection_count(rooms[room_idx]), 25)
    }
    
    fr fr Test broadcasting to all rooms simultaneously
    sus total_broadcasts normie = 0
    bestie room_idx normie = 0; room_idx < 10; room_idx++ {
        sus broadcast_msg tea = "Broadcast message to room " + stringz.int_to_string(room_idx)
        sus broadcast_count normie = ws_room_broadcast(rooms[room_idx], broadcast_msg)
        total_broadcasts = total_broadcasts + broadcast_count
        assert_eq_int(broadcast_count, 25)
    }
    assert_eq_int(total_broadcasts, 250)  fr fr 10 rooms * 25 connections
    
    vibez.spill("✅ Successfully broadcast to " + stringz.int_to_string(total_broadcasts) + " connections across 10 rooms")
}

slay performance_test_message_throughput() {
    test_start("WebSocket Message Throughput")
    
    fr fr Create multiple connections
    sus connections WebSocketConnection[5]
    bestie i normie = 0; i < 5; i++ {
        sus url tea = "ws://localhost:808" + stringz.int_to_string(i) + "/perf"
        connections[i] = ws_connection_create(url, cap)
        ws_connection_open(&connections[i])
        assert_true(ws_connection_is_open(connections[i]))
    }
    
    fr fr Send high volume of messages
    sus messages_per_connection normie = 100
    sus total_messages normie = 0
    
    bestie conn_idx normie = 0; conn_idx < 5; conn_idx++ {
        bestie msg_idx normie = 0; msg_idx < messages_per_connection; msg_idx++ {
            sus message tea = "Performance test message " + 
                             stringz.int_to_string(conn_idx) + ":" + 
                             stringz.int_to_string(msg_idx)
            
            fr fr Test different message types
            lowkey msg_idx % 3 == 0 {
                sus result lit = ws_send_text(&connections[conn_idx], message)
                assert_eq_lit(result, based)
            } elif msg_idx % 3 == 1 {
                sus binary_data tea = "BINARY:" + message
                sus result lit = ws_send_binary(&connections[conn_idx], binary_data)
                assert_eq_lit(result, based)
            } else {
                fr fr Send ping every 3rd message
                sus ping_data tea = "PING:" + stringz.int_to_string(msg_idx)
                sus result lit = ws_send_ping(&connections[conn_idx], ping_data)
                assert_eq_lit(result, based)
            }
            total_messages++
        }
    }
    
    assert_eq_int(total_messages, 500)  fr fr 5 connections * 100 messages
    vibez.spill("✅ Successfully sent " + stringz.int_to_string(total_messages) + " messages across 5 connections")
}

slay performance_test_large_frames() {
    test_start("WebSocket Large Frame Handling")
    
    sus conn WebSocketConnection = ws_connection_create("ws://localhost:8080/large", cap)
    ws_connection_open(&conn)
    
    fr fr Test progressively larger frames
    sus frame_sizes normie[8]
    frame_sizes[0] = 100      fr fr 100 bytes
    frame_sizes[1] = 1024     fr fr 1KB
    frame_sizes[2] = 10240    fr fr 10KB
    frame_sizes[3] = 65536    fr fr 64KB
    frame_sizes[4] = 262144   fr fr 256KB
    frame_sizes[5] = 524288   fr fr 512KB
    frame_sizes[6] = 1048576  fr fr 1MB (max default)
    frame_sizes[7] = 2097152  fr fr 2MB (should fail)
    
    bestie i normie = 0; i < 8; i++ {
        sus frame_size normie = frame_sizes[i]
        sus large_message tea = ""
        
        fr fr Build message of target size
        sus segment tea = "A"
        bestie j normie = 0; j < frame_size; j++ {
            large_message = stringz.concat(large_message, segment)
        }
        assert_eq_int(stringz.length(large_message), frame_size)
        
        fr fr Try to send the large message
        sus result lit = ws_send_text(&conn, large_message)
        
        lowkey frame_size <= conn.max_frame_size {
            assert_eq_lit(result, based)
            vibez.spill("✅ Successfully sent " + stringz.int_to_string(frame_size) + " byte frame")
        } else {
            assert_eq_lit(result, cap)  fr fr Should fail for frames exceeding limit
            vibez.spill("✅ Correctly rejected " + stringz.int_to_string(frame_size) + " byte frame (exceeds limit)")
        }
    }
}

slay performance_test_concurrent_rooms() {
    test_start("WebSocket Concurrent Room Operations")
    
    fr fr Create multiple rooms with overlapping users
    sus rooms WebSocketRoom[5]
    sus room_names tea[5]
    room_names[0] = "General"
    room_names[1] = "Tech"
    room_names[2] = "Gaming"
    room_names[3] = "Music"
    room_names[4] = "Sports"
    
    bestie i normie = 0; i < 5; i++ {
        sus room_id tea = "concurrent_" + stringz.int_to_string(i)
        rooms[i] = ws_room_create(room_id, room_names[i])
    }
    
    fr fr Add users to multiple rooms (simulating real-world usage)
    sus total_joins normie = 0
    bestie user_id normie = 2000; user_id < 2050; user_id++ {
        fr fr Each user joins 2-3 random rooms
        bestie room_idx normie = 0; room_idx < 3; room_idx++ {
            sus target_room normie = (user_id + room_idx) % 5
            ws_room_join(&rooms[target_room], user_id)
            total_joins++
        }
    }
    
    fr fr Verify room populations
    sus total_connections normie = 0
    bestie room_idx normie = 0; room_idx < 5; room_idx++ {
        sus room_count normie = ws_room_get_connection_count(rooms[room_idx])
        total_connections = total_connections + room_count
        vibez.spill("Room '" + room_names[room_idx] + "' has " + 
                   stringz.int_to_string(room_count) + " connections")
    }
    
    fr fr Test simultaneous broadcasting to all rooms
    sus total_broadcast_count normie = 0
    bestie room_idx normie = 0; room_idx < 5; room_idx++ {
        sus broadcast_msg tea = "Simultaneous broadcast to " + room_names[room_idx] + " room"
        sus room_broadcast_count normie = ws_room_broadcast(rooms[room_idx], broadcast_msg)
        total_broadcast_count = total_broadcast_count + room_broadcast_count
    }
    
    vibez.spill("✅ Total connections across all rooms: " + stringz.int_to_string(total_connections))
    vibez.spill("✅ Total broadcasts sent: " + stringz.int_to_string(total_broadcast_count))
}

slay performance_test_message_queue_stress() {
    test_start("WebSocket Message Queue Stress Test")
    
    sus conn WebSocketConnection = ws_connection_create("ws://localhost:8080/queue", cap)
    ws_connection_open(&conn)
    
    fr fr Fill message queue to near capacity
    sus max_messages normie = 95  fr fr Leave some room (queue size is 100)
    bestie i normie = 0; i < max_messages; i++ {
        sus message_content tea = "Queue stress test message #" + stringz.int_to_string(i) + 
                                 " with additional content to make it realistic"
        sus queue_result lit = ws_queue_message(&conn, WS_OPCODE_TEXT, message_content)
        assert_eq_lit(queue_result, based)
    }
    assert_eq_int(conn.queue_size, max_messages)
    
    fr fr Try to add more messages (should still succeed until queue is full)
    bestie i normie = max_messages; i < 100; i++ {
        sus message_content tea = "Final queue message #" + stringz.int_to_string(i)
        sus queue_result lit = ws_queue_message(&conn, WS_OPCODE_TEXT, message_content)
        assert_eq_lit(queue_result, based)
    }
    assert_eq_int(conn.queue_size, 100)
    
    fr fr Try to add one more (should fail - queue full)
    sus overflow_result lit = ws_queue_message(&conn, WS_OPCODE_TEXT, "This should fail")
    assert_eq_lit(overflow_result, cap)
    assert_eq_int(conn.queue_size, 100)
    
    fr fr Drain the queue rapidly
    sus processed_messages normie = 0
    bestie {
        sus msg WebSocketMessage = ws_receive_message(&conn)
        lowkey msg.message_type == 0 {
            break  fr fr No more messages
        }
        processed_messages++
        assert_true(stringz.contains(msg.payload, "message #"))
    }
    
    assert_eq_int(processed_messages, 100)
    assert_eq_int(conn.queue_size, 0)
    vibez.spill("✅ Successfully processed " + stringz.int_to_string(processed_messages) + " queued messages")
}

slay performance_test_frame_serialization() {
    test_start("WebSocket Frame Serialization Performance")
    
    fr fr Test serialization of different frame types and sizes
    sus frame_types smol[6]
    frame_types[0] = WS_OPCODE_TEXT
    frame_types[1] = WS_OPCODE_BINARY
    frame_types[2] = WS_OPCODE_PING
    frame_types[3] = WS_OPCODE_PONG
    frame_types[4] = WS_OPCODE_CLOSE
    frame_types[5] = WS_OPCODE_CONTINUATION
    
    sus payload_sizes normie[5]
    payload_sizes[0] = 10      fr fr Very small
    payload_sizes[1] = 125     fr fr Control frame limit
    payload_sizes[2] = 1024    fr fr 1KB
    payload_sizes[3] = 65536   fr fr 64KB
    payload_sizes[4] = 131072  fr fr 128KB
    
    sus total_serializations normie = 0
    
    bestie type_idx normie = 0; type_idx < 6; type_idx++ {
        sus opcode smol = frame_types[type_idx]
        sus opcode_name tea = ws_get_opcode_name(opcode)
        
        bestie size_idx normie = 0; size_idx < 5; size_idx++ {
            sus payload_size normie = payload_sizes[size_idx]
            
            fr fr Skip large payloads for control frames
            lowkey ws_is_control_frame(opcode) && payload_size > 125 {
                continue
            }
            
            fr fr Build payload of target size
            sus payload tea = ""
            sus base_char tea = (opcode == WS_OPCODE_BINARY) ? "B" : "T"
            bestie i normie = 0; i < payload_size; i++ {
                payload = stringz.concat(payload, base_char)
            }
            
            fr fr Create and serialize frame
            sus frame WebSocketFrame = ws_frame_create(opcode, payload, based)
            sus serialized tea = ws_frame_serialize(frame)
            
            fr fr Verify serialization
            assert_true(stringz.starts_with(serialized, "WS-FRAME:"))
            assert_true(stringz.contains(serialized, stringz.int_to_string(payload_size)))
            
            total_serializations++
        }
    }
    
    vibez.spill("✅ Successfully serialized " + stringz.int_to_string(total_serializations) + " frames")
}

slay performance_test_handshake_overhead() {
    test_start("WebSocket Handshake Performance")
    
    fr fr Test multiple handshake operations
    sus handshake_count normie = 50
    sus successful_handshakes normie = 0
    
    bestie i normie = 0; i < handshake_count; i++ {
        fr fr Generate client key
        sus client_key tea = ws_generate_key()
        assert_true(ws_validate_key(client_key))
        
        fr fr Compute accept key
        sus accept_key tea = ws_compute_accept_key(client_key)
        assert_true(stringz.length(accept_key) > 0)
        
        fr fr Create handshake request
        sus protocols tea[3]
        protocols[0] = "chat"
        protocols[1] = "echo"
        protocols[2] = "test"
        sus url tea = "/test_" + stringz.int_to_string(i)
        sus request tea = ws_create_handshake_request(url, protocols, 3)
        
        fr fr Validate request
        assert_true(ws_validate_handshake_request(request))
        
        fr fr Create response
        sus response tea = ws_create_handshake_response(client_key, "chat")
        
        fr fr Validate response
        assert_true(ws_validate_handshake_response(response, accept_key))
        
        successful_handshakes++
    }
    
    assert_eq_int(successful_handshakes, handshake_count)
    vibez.spill("✅ Successfully completed " + stringz.int_to_string(successful_handshakes) + " handshakes")
}

slay performance_test_extension_processing() {
    test_start("WebSocket Extension Processing Performance")
    
    fr fr Test extension negotiation with various combinations
    sus client_extensions tea[5]
    client_extensions[0] = "permessage-deflate"
    client_extensions[1] = "permessage-deflate; client_max_window_bits"
    client_extensions[2] = "x-webkit-deflate-frame"
    client_extensions[3] = "permessage-deflate, x-webkit-deflate-frame"
    client_extensions[4] = "unknown-extension"
    
    sus server_extensions tea[3]
    server_extensions[0] = "permessage-deflate"
    server_extensions[1] = "x-webkit-deflate-frame"
    server_extensions[2] = "unsupported-extension"
    
    sus successful_negotiations normie = 0
    sus compression_tests normie = 0
    
    bestie client_idx normie = 0; client_idx < 5; client_idx++ {
        bestie server_idx normie = 0; server_idx < 3; server_idx++ {
            sus negotiated tea = ws_extension_negotiate(
                client_extensions[client_idx], 
                server_extensions[server_idx]
            )
            
            lowkey stringz.length(negotiated) > 0 {
                successful_negotiations++
                
                fr fr Test compression/decompression if supported
                lowkey negotiated == "permessage-deflate" {
                    sus test_data tea = "This is test data for compression performance test " + 
                                       stringz.int_to_string(compression_tests)
                    sus compressed tea = ws_extension_compress(test_data, negotiated)
                    sus decompressed tea = ws_extension_decompress(compressed, negotiated)
                    
                    assert_eq_string(decompressed, test_data)
                    compression_tests++
                }
            }
        }
    }
    
    vibez.spill("✅ Successful extension negotiations: " + stringz.int_to_string(successful_negotiations))
    vibez.spill("✅ Compression tests completed: " + stringz.int_to_string(compression_tests))
}

slay performance_test_security_validation() {
    test_start("WebSocket Security Validation Performance")
    
    fr fr Test origin validation with large lists
    sus allowed_origins tea[20]
    bestie i normie = 0; i < 20; i++ {
        allowed_origins[i] = "https://domain" + stringz.int_to_string(i) + ".com"
    }
    
    sus test_origins tea[10]
    test_origins[0] = "https://domain5.com"    fr fr Should pass
    test_origins[1] = "https://domain15.com"   fr fr Should pass
    test_origins[2] = "https://malicious.com"  fr fr Should fail
    test_origins[3] = "https://domain0.com"    fr fr Should pass
    test_origins[4] = "https://domain19.com"   fr fr Should pass
    test_origins[5] = "https://evil.com"       fr fr Should fail
    test_origins[6] = "https://domain10.com"   fr fr Should pass
    test_origins[7] = ""                       fr fr Should fail
    test_origins[8] = "https://domain7.com"    fr fr Should pass
    test_origins[9] = "https://domain99.com"   fr fr Should fail
    
    sus validation_tests normie = 0
    sus passed_validations normie = 0
    
    bestie i normie = 0; i < 10; i++ {
        sus result lit = ws_validate_origin(test_origins[i], allowed_origins, 20)
        validation_tests++
        
        fr fr Count expected passes
        lowkey stringz.contains(test_origins[i], "domain") && 
             !stringz.contains(test_origins[i], "99") {
            assert_eq_lit(result, based)
            passed_validations++
        } else {
            assert_eq_lit(result, cap)
        }
    }
    
    fr fr Test content filtering performance
    sus blocked_words tea[10]
    blocked_words[0] = "spam"
    blocked_words[1] = "malicious"
    blocked_words[2] = "forbidden"
    blocked_words[3] = "phishing"
    blocked_words[4] = "scam"
    blocked_words[5] = "virus"
    blocked_words[6] = "hack"
    blocked_words[7] = "exploit"
    blocked_words[8] = "crack"
    blocked_words[9] = "piracy"
    
    sus filter_tests normie = 0
    sus clean_messages normie = 0
    
    bestie i normie = 0; i < 50; i++ {
        sus message tea = "This is test message number " + stringz.int_to_string(i)
        
        fr fr Add some blocked content to some messages
        lowkey i % 10 == 0 {
            message = stringz.concat(message, " with spam content")
        } elif i % 15 == 0 {
            message = stringz.concat(message, " malicious intent")
        }
        
        sus result lit = ws_content_filter(message, blocked_words, 10)
        filter_tests++
        
        lowkey result {
            clean_messages++
        }
    }
    
    vibez.spill("✅ Origin validations: " + stringz.int_to_string(validation_tests) + 
               " tests, " + stringz.int_to_string(passed_validations) + " passed")
    vibez.spill("✅ Content filtering: " + stringz.int_to_string(filter_tests) + 
               " tests, " + stringz.int_to_string(clean_messages) + " clean messages")
}

fr fr =============================================================================
fr fr PERFORMANCE TEST SUITE EXECUTION
fr fr =============================================================================

slay run_all_performance_tests() {
    vibez.spill("🏃 Running WebSocket Performance Test Suite")
    vibez.spill("===========================================")
    
    fr fr Core performance tests
    performance_test_room_scaling()
    performance_test_message_throughput()
    performance_test_large_frames()
    performance_test_concurrent_rooms()
    
    fr fr Advanced performance tests
    performance_test_message_queue_stress()
    performance_test_frame_serialization()
    performance_test_handshake_overhead()
    performance_test_extension_processing()
    performance_test_security_validation()
    
    print_test_summary()
    vibez.spill("✅ WebSocket Performance Test Suite completed!")
}

fr fr Run the complete performance test suite
run_all_performance_tests()

fr fr Additional demo for performance characteristics
vibez.spill("")
vibez.spill("📊 WebSocket Performance Characteristics")
vibez.spill("=======================================")

slay demo_performance_characteristics() {
    vibez.spill("🔧 WebSocket Performance Demo")
    
    fr fr Demo scalability limits
    vibez.spill("📈 Testing scalability limits...")
    sus mega_room WebSocketRoom = ws_room_create("mega", "Maximum Capacity Test")
    
    fr fr Fill room to maximum capacity
    bestie i normie = 1; i <= 50; i++ {  fr fr max_connections = 50
        ws_room_join(&mega_room, 5000 + i)
    }
    
    sus final_count normie = ws_room_get_connection_count(mega_room)
    vibez.spill("Maximum room capacity: " + stringz.int_to_string(final_count) + " connections")
    
    fr fr Test broadcast performance at capacity
    sus broadcast_result normie = ws_room_broadcast(mega_room, "Performance test at maximum capacity")
    vibez.spill("Broadcast to full room: " + stringz.int_to_string(broadcast_result) + " messages sent")
    
    fr fr Demo frame size optimization
    vibez.spill("📦 Frame size optimization...")
    sus sizes normie[3]
    sizes[0] = 100
    sizes[1] = 1024
    sizes[2] = 65536
    
    sus conn WebSocketConnection = ws_connection_create("ws://localhost:8080/demo", cap)
    ws_connection_open(&conn)
    
    bestie i normie = 0; i < 3; i++ {
        sus size normie = sizes[i]
        sus test_msg tea = ""
        bestie j normie = 0; j < size; j++ {
            test_msg = stringz.concat(test_msg, "x")
        }
        
        sus start_time normie = 0  fr fr Would use actual timing
        sus result lit = ws_send_text(&conn, test_msg)
        sus end_time normie = 1    fr fr Would use actual timing
        
        lowkey result {
            vibez.spill("Frame size " + stringz.int_to_string(size) + " bytes: ✅ Sent successfully")
        } else {
            vibez.spill("Frame size " + stringz.int_to_string(size) + " bytes: ❌ Failed to send")
        }
    }
    
    vibez.spill("✅ Performance characteristics demo completed!")
}

demo_performance_characteristics()
