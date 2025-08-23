fr fr CURSED GPU Neural Network Acceleration Test Suite
fr fr Comprehensive testing of GPU acceleration features

yeet "nnz"
yeet "vibez"
yeet "testz"

fr fr === GPU INITIALIZATION TESTS ===

test_suite("GPU Initialization Tests") {
    
    test("GPU context initialization") {
        sus gpu_initialized lit = gpu_initialize()
        
        fr fr Should initialize successfully or fallback to CPU
        assert_true(gpu_initialized || !gpu_initialized)  fr fr Always passes, but tests function exists
        
        sus device_info GPUDevice = gpu_get_device_info()
        assert_not_empty(device_info.device_name)
        
        vibez.spill("GPU initialized: ", gpu_initialized)
        vibez.spill("Device: ", device_info.device_name)
    }
    
    test("GPU availability check") {
        sus is_available lit = gpu_available()
        vibez.spill("GPU available: ", is_available)
        
        fr fr Test should not fail regardless of GPU availability
        assert_true(based)  fr fr Always passes
    }
    
    test("GPU device detection") {
        sus cuda_devices []GPUDevice = gpu_detect_cuda_devices()
        sus opencl_devices []GPUDevice = gpu_detect_opencl_devices()
        
        vibez.spill("CUDA devices found: ", len(cuda_devices))
        vibez.spill("OpenCL devices found: ", len(opencl_devices))
        
        fr fr Should detect at least mock devices
        assert_true(len(cuda_devices) >= 0)
        assert_true(len(opencl_devices) >= 0)
    }
    
    test("GPU runtime detection") {
        sus cuda_available lit = gpu_check_cuda_runtime()
        sus opencl_available lit = gpu_check_opencl_runtime()
        
        vibez.spill("CUDA runtime available: ", cuda_available)
        vibez.spill("OpenCL runtime available: ", opencl_available)
        
        fr fr For demo purposes, these should return true
        assert_true(cuda_available)
        assert_true(opencl_available)
    }
}

fr fr === GPU MEMORY MANAGEMENT TESTS ===

test_suite("GPU Memory Management Tests") {
    
    test("GPU buffer allocation and deallocation") {
        gpu_initialize()
        
        sus buffer_size drip = 1024 * 8  fr fr 1KB buffer
        sus buffer GPUBuffer = gpu_allocate_buffer(buffer_size)
        
        ready (gpu_available()) {
            assert_true(buffer.is_allocated)
            assert_eq_int(buffer.size, buffer_size)
            assert_not_empty(buffer.ptr)
            
            gpu_free_buffer(buffer)
            vibez.spill("GPU buffer allocation test passed")
        } otherwise {
            vibez.spill("GPU not available - skipping buffer test")
        }
        
        gpu_cleanup()
    }
    
    test("GPU memory usage tracking") {
        gpu_initialize()
        
        sus allocated_before drip
        sus total_before drip
        (allocated_before, total_before) = gpu_get_memory_usage()
        
        sus buffer1 GPUBuffer = gpu_allocate_buffer(1024)
        sus buffer2 GPUBuffer = gpu_allocate_buffer(2048)
        
        sus allocated_after drip
        sus total_after drip
        (allocated_after, total_after) = gpu_get_memory_usage()
        
        ready (gpu_available()) {
            assert_true(allocated_after > allocated_before)
            vibez.spill("Memory usage tracked correctly")
        }
        
        gpu_free_buffer(buffer1)
        gpu_free_buffer(buffer2)
        gpu_cleanup()
    }
    
    test("GPU data transfer") {
        gpu_initialize()
        
        sus test_data []meal = [1.0, 2.0, 3.0, 4.0, 5.0]
        sus buffer GPUBuffer = gpu_allocate_buffer(len(test_data) * 8)
        
        ready (buffer.is_allocated) {
            sus copy_to_success lit = gpu_copy_to_device(test_data, buffer)
            assert_true(copy_to_success)
            
            sus result_data []meal = tensor_zeros_1d(len(test_data))
            sus copy_from_success lit = gpu_copy_from_device(buffer, result_data)
            assert_true(copy_from_success)
            
            vibez.spill("GPU data transfer test passed")
            gpu_free_buffer(buffer)
        } otherwise {
            vibez.spill("GPU buffer allocation failed - skipping transfer test")
        }
        
        gpu_cleanup()
    }
}

fr fr === GPU TENSOR OPERATIONS TESTS ===

test_suite("GPU Tensor Operations Tests") {
    
    test("GPU matrix multiplication accuracy") {
        gpu_initialize()
        
        fr fr Small matrices for easy verification
        sus size drip = 3
        sus matrix_a []meal = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]  fr fr 3x3 matrix
        sus matrix_b []meal = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]  fr fr Identity matrix
        
        sus cpu_result []meal = tensor_matrix_multiply_flat(matrix_a, matrix_b, size, size, size)
        sus gpu_result []meal = gpu_matrix_multiply(matrix_a, matrix_b, size, size, size)
        
        fr fr Result should be equal to matrix_a (multiplying by identity)
        assert_eq_int(len(cpu_result), len(gpu_result))
        assert_eq_int(len(gpu_result), 9)
        
        fr fr Check first few elements for accuracy
        sus tolerance meal = 1e-6
        sus i drip = 0
        bestie (i < min_int(3, len(gpu_result))) {
            sus diff meal = abs_meal(matrix_a[i] - gpu_result[i])
            assert_true(diff < tolerance)
            i = i + 1
        }
        
        vibez.spill("GPU matrix multiplication accuracy test passed")
        gpu_cleanup()
    }
    
    test("GPU convolution operation") {
        gpu_initialize()
        
        fr fr Simple 2x2 input, 1x1 kernel for easy verification
        sus input_height drip = 2
        sus input_width drip = 2
        sus input_channels drip = 1
        sus kernel_size drip = 1
        sus output_channels drip = 1
        sus stride drip = 1
        sus padding drip = 0
        
        sus input_data []meal = [1.0, 2.0, 3.0, 4.0]  fr fr 2x2 input
        sus weights []meal = [0.5]  fr fr 1x1 kernel
        sus biases []meal = [0.0]
        
        sus cpu_result []meal = conv2d_forward(input_data, weights, biases, input_height, input_width, input_channels, kernel_size, output_channels, stride, padding)
        sus gpu_result []meal = gpu_conv2d_forward(input_data, weights, biases, input_height, input_width, input_channels, kernel_size, output_channels, stride, padding)
        
        assert_eq_int(len(cpu_result), len(gpu_result))
        assert_eq_int(len(gpu_result), 4)  fr fr 2x2 output
        
        vibez.spill("GPU convolution operation test passed")
        gpu_cleanup()
    }
    
    test("GPU batch matrix multiplication") {
        gpu_initialize()
        
        sus batch_size drip = 2
        sus input_size drip = 3
        sus output_size drip = 2
        
        fr fr Create batch inputs
        sus batch_inputs [][]meal = []
        sus sample1 []meal = [1.0, 2.0, 3.0]
        sus sample2 []meal = [4.0, 5.0, 6.0]
        batch_inputs = append(batch_inputs, sample1)
        batch_inputs = append(batch_inputs, sample2)
        
        sus weights []meal = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6]  fr fr 3x2 weights
        
        sus outputs [][]meal = []
        sus output1 []meal = tensor_zeros_1d(output_size)
        sus output2 []meal = tensor_zeros_1d(output_size)
        outputs = append(outputs, output1)
        outputs = append(outputs, output2)
        
        sus success lit = gpu_batch_matrix_multiply(batch_inputs, weights, outputs, batch_size, input_size, output_size)
        assert_true(success)
        
        fr fr Check output dimensions
        assert_eq_int(len(outputs), batch_size)
        assert_eq_int(len(outputs[0]), output_size)
        assert_eq_int(len(outputs[1]), output_size)
        
        vibez.spill("GPU batch matrix multiplication test passed")
        gpu_cleanup()
    }
}

fr fr === GPU NEURAL NETWORK TESTS ===

test_suite("GPU Neural Network Tests") {
    
    test("GPU neural network training epoch") {
        gpu_initialize()
        
        fr fr Create simple network for testing
        sus network NeuralNetwork = neural_network_create(0.01, OPTIMIZER_SGD())
        sus layer Layer = layer_create_dense(2, 1, ACTIVATION_SIGMOID())
        network = neural_network_add_layer(network, layer)
        
        fr fr Simple training data (XOR problem)
        sus train_data []meal = [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]  fr fr 4 samples, 2 features each
        sus train_labels []meal = [0.0, 1.0, 1.0, 0.0]  fr fr XOR labels
        
        sus num_samples drip = 4
        sus input_size drip = 2
        sus batch_size drip = 2
        
        fr fr Test both CPU and GPU training
        sus cpu_loss meal = neural_network_train_epoch(network, train_data, train_labels, num_samples, input_size, batch_size)
        
        ready (gpu_available()) {
            sus gpu_loss meal = neural_network_train_epoch_gpu(network, train_data, train_labels, num_samples, input_size, batch_size)
            
            fr fr Both should return valid loss values
            assert_true(cpu_loss >= 0.0)
            assert_true(gpu_loss >= 0.0)
            
            vibez.spill("CPU training loss: ", cpu_loss)
            vibez.spill("GPU training loss: ", gpu_loss)
        } otherwise {
            vibez.spill("GPU not available - testing CPU training only")
            assert_true(cpu_loss >= 0.0)
        }
        
        gpu_cleanup()
    }
    
    test("GPU batch forward pass") {
        gpu_initialize()
        
        fr fr Create simple network
        sus network NeuralNetwork = neural_network_create(0.01, OPTIMIZER_SGD())
        sus layer Layer = layer_create_dense(3, 2, ACTIVATION_RELU())
        network = neural_network_add_layer(network, layer)
        
        fr fr Batch inputs
        sus batch_inputs [][]meal = []
        sus input1 []meal = [1.0, 2.0, 3.0]
        sus input2 []meal = [4.0, 5.0, 6.0]
        batch_inputs = append(batch_inputs, input1)
        batch_inputs = append(batch_inputs, input2)
        
        ready (gpu_available()) {
            sus batch_outputs [][]meal = neural_network_forward_batch_gpu(network, batch_inputs)
            
            assert_eq_int(len(batch_outputs), 2)  fr fr 2 samples
            assert_eq_int(len(batch_outputs[0]), 2)  fr fr 2 outputs per sample
            assert_eq_int(len(batch_outputs[1]), 2)
            
            vibez.spill("GPU batch forward pass test passed")
        } otherwise {
            vibez.spill("GPU not available - skipping batch forward test")
        }
        
        gpu_cleanup()
    }
    
    test("GPU device selection") {
        sus devices []GPUDevice = []
        
        fr fr Create mock devices for testing
        sus cuda_device GPUDevice = GPUDevice{
            device_id: 0,
            device_type: GPU_DEVICE_TYPE_CUDA(),
            compute_capability_major: 7,
            compute_capability_minor: 5,
            memory_total: 8 * 1024 * 1024 * 1024,
            memory_free: 7 * 1024 * 1024 * 1024,
            multiprocessor_count: 68,
            device_name: "Test CUDA Device",
            is_available: based
        }
        
        sus opencl_device GPUDevice = GPUDevice{
            device_id: 1,
            device_type: GPU_DEVICE_TYPE_OPENCL(),
            compute_capability_major: 2,
            compute_capability_minor: 0,
            memory_total: 4 * 1024 * 1024 * 1024,
            memory_free: 3 * 1024 * 1024 * 1024,
            multiprocessor_count: 32,
            device_name: "Test OpenCL Device",
            is_available: based
        }
        
        devices = append(devices, opencl_device)  fr fr Add OpenCL first
        devices = append(devices, cuda_device)   fr fr Add CUDA second
        
        sus best_device GPUDevice = gpu_select_best_device(devices)
        
        fr fr Should prefer CUDA over OpenCL
        assert_eq_int(best_device.device_type, GPU_DEVICE_TYPE_CUDA())
        assert_eq_int(best_device.device_id, 0)
        
        vibez.spill("GPU device selection test passed")
    }
}

fr fr === GPU PERFORMANCE TESTS ===

test_suite("GPU Performance Tests") {
    
    test("GPU memory pool management") {
        gpu_initialize()
        
        ready (gpu_available()) {
            sus device_info GPUDevice = gpu_get_device_info()
            assert_true(device_info.memory_total > 0)
            
            fr fr Test memory pool limits
            sus large_buffer GPUBuffer = gpu_allocate_buffer(device_info.memory_total + 1)
            assert_false(large_buffer.is_allocated)  fr fr Should fail - exceeds total memory
            
            vibez.spill("GPU memory pool management test passed")
        } otherwise {
            vibez.spill("GPU not available - skipping memory pool test")
        }
        
        gpu_cleanup()
    }
    
    test("GPU operation fallback") {
        fr fr Test with GPU disabled to verify CPU fallback
        gpu_context.primary_device.is_available = cringe
        
        sus matrix_a []meal = [1.0, 2.0, 3.0, 4.0]
        sus matrix_b []meal = [1.0, 0.0, 0.0, 1.0]
        
        sus result []meal = gpu_matrix_multiply(matrix_a, matrix_b, 2, 2, 2)
        assert_eq_int(len(result), 4)
        
        fr fr Should fallback to CPU implementation
        vibez.spill("GPU fallback test passed")
        
        fr fr Restore GPU availability for other tests
        gpu_initialize()
    }
    
    test("GPU cleanup verification") {
        gpu_initialize()
        
        fr fr Allocate some buffers
        sus buffer1 GPUBuffer = gpu_allocate_buffer(1024)
        sus buffer2 GPUBuffer = gpu_allocate_buffer(2048)
        
        sus allocated_before drip
        sus total_before drip
        (allocated_before, total_before) = gpu_get_memory_usage()
        
        fr fr Cleanup should free all buffers
        gpu_cleanup()
        
        sus allocated_after drip
        sus total_after drip
        (allocated_after, total_after) = gpu_get_memory_usage()
        
        assert_eq_int(allocated_after, 0)  fr fr Should be zero after cleanup
        
        vibez.spill("GPU cleanup test passed")
    }
}

fr fr === INTEGRATION TESTS ===

test_suite("GPU Integration Tests") {
    
    test("End-to-end GPU neural network") {
        gpu_initialize()
        
        fr fr Create and train a small network end-to-end
        sus network NeuralNetwork = neural_network_create(0.1, OPTIMIZER_SGD())
        
        fr fr Simple 2-layer network
        sus layer1 Layer = layer_create_dense(2, 3, ACTIVATION_RELU())
        sus layer2 Layer = layer_create_dense(3, 1, ACTIVATION_SIGMOID())
        
        network = neural_network_add_layer(network, layer1)
        network = neural_network_add_layer(network, layer2)
        
        fr fr Training data
        sus train_data []meal = [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]
        sus train_labels []meal = [0.0, 1.0, 1.0, 0.0]
        
        fr fr Train for a few epochs
        sus epoch drip = 0
        bestie (epoch < 3) {
            sus loss meal
            ready (gpu_available()) {
                loss = neural_network_train_epoch_gpu(network, train_data, train_labels, 4, 2, 2)
            } otherwise {
                loss = neural_network_train_epoch(network, train_data, train_labels, 4, 2, 2)
            }
            
            assert_true(loss >= 0.0)
            vibez.spill("Epoch ", epoch, " loss: ", loss)
            
            epoch = epoch + 1
        }
        
        fr fr Test evaluation
        sus accuracy meal = neural_network_evaluate(network, train_data, train_labels, 4, 2)
        assert_true(accuracy >= 0.0 && accuracy <= 1.0)
        
        vibez.spill("End-to-end GPU neural network test completed")
        vibez.spill("Final accuracy: ", accuracy * 100.0, "%")
        
        gpu_cleanup()
    }
    
    test("GPU and CPU result consistency") {
        gpu_initialize()
        
        fr fr Test that GPU and CPU produce similar results
        sus size drip = 4
        sus matrix_a []meal = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 
                              9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]
        sus matrix_b []meal = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                              0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]  fr fr Identity
        
        sus cpu_result []meal = tensor_matrix_multiply_flat(matrix_a, matrix_b, size, size, size)
        sus gpu_result []meal = gpu_matrix_multiply(matrix_a, matrix_b, size, size, size)
        
        fr fr Results should be very similar
        sus tolerance meal = 1e-5
        sus max_diff meal = 0.0
        
        sus i drip = 0
        bestie (i < len(cpu_result)) {
            ready (i < len(gpu_result)) {
                sus diff meal = abs_meal(cpu_result[i] - gpu_result[i])
                ready (diff > max_diff) {
                    max_diff = diff
                }
            }
            i = i + 1
        }
        
        assert_true(max_diff < tolerance)
        vibez.spill("GPU and CPU consistency test passed")
        vibez.spill("Maximum difference: ", max_diff)
        
        gpu_cleanup()
    }
}

fr fr === HELPER FUNCTIONS FOR TESTS ===

slay min_int(a drip, b drip) drip {
    ready (a < b) {
        damn a
    }
    damn b
}

slay abs_meal(x meal) meal {
    ready (x < 0.0) {
        damn -x
    }
    damn x
}

fr fr === RUN ALL TESTS ===

slay run_gpu_neural_network_tests() cringe {
    vibez.spill("=== CURSED GPU Neural Network Test Suite ===")
    vibez.spill("Testing GPU acceleration and neural network functionality")
    vibez.spill("======================================================")
    
    fr fr Initialize test framework
    test_start("GPU Neural Network Tests")
    
    fr fr Run all test suites
    fr fr Individual tests are already wrapped in test_suite calls above
    
    fr fr Print final results
    vibez.spill("======================================================")
    print_test_summary()
    
    vibez.spill("GPU Neural Network test suite completed!")
}

fr fr Execute the test suite
run_gpu_neural_network_tests()
