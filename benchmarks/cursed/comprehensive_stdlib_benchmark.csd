fr fr Comprehensive CURSED Stdlib Performance Benchmark

yeet "stdlib/testz"
yeet "stdlib/math"
yeet "stdlib/string"
yeet "stdlib/collections"
yeet "stdlib/crypto"
yeet "stdlib/time"
yeet "stdlib/memory"

fr fr Benchmark timing utilities
sus benchmark_start_time thicc = 0
sus benchmark_iterations normie = 1000

slay benchmark_start(name tea) {
    vibez.spill("Starting benchmark: " + name)
    benchmark_start_time = timez.now()
}

slay benchmark_end(name tea) {
    sus elapsed thicc = timez.now() - benchmark_start_time
    vibez.spill("Benchmark " + name + " completed in " + string.from_int(elapsed) + "ms")
}

fr fr Math Module Benchmarks
slay benchmark_math_operations() {
    benchmark_start("Math Operations")
    
    sus total drip = 0.0
    bestie i := 0; i < benchmark_iterations; i++ {
        sus x drip = drip.(i)
        total = total + math.sqrt(x)
        total = total + math.sin(x)
        total = total + math.cos(x)
        total = total + math.log(x + 1.0)
        total = total + math.exp(x / 1000.0)
    }
    
    vibez.spill("Math operations total: " + string.from_float(total))
    benchmark_end("Math Operations")
}

fr fr String Module Benchmarks
slay benchmark_string_operations() {
    benchmark_start("String Operations")
    
    sus test_string tea = "Hello World! This is a test string for benchmarking."
    sus result tea = ""
    
    bestie i := 0; i < benchmark_iterations; i++ {
        sus upper tea = string.to_upper(test_string)
        sus lower tea = string.to_lower(upper)
        sus concat tea = string.concat(lower, " - iteration " + string.from_int(i))
        sus substring tea = string.substring(concat, 0, 20)
        result = string.concat(result, substring)
    }
    
    vibez.spill("String operations result length: " + string.from_int(string.length(result)))
    benchmark_end("String Operations")
}

fr fr Collections Module Benchmarks
slay benchmark_collections_operations() {
    benchmark_start("Collections Operations")
    
    sus map collections.HashMap = collections.new_hashmap()
    sus list collections.Vector = collections.new_vector()
    
    fr fr HashMap operations
    bestie i := 0; i < benchmark_iterations; i++ {
        sus key tea = "key_" + string.from_int(i)
        sus value normie = i * 2
        collections.hashmap_put(map, key, value)
        
        lowkey i % 2 == 0 {
            sus retrieved normie = collections.hashmap_get(map, key)
        }
    }
    
    fr fr Vector operations
    bestie i := 0; i < benchmark_iterations; i++ {
        collections.vector_push(list, i)
        
        lowkey i % 3 == 0 {
            sus popped normie = collections.vector_pop(list)
        }
    }
    
    vibez.spill("HashMap size: " + string.from_int(collections.hashmap_size(map)))
    vibez.spill("Vector size: " + string.from_int(collections.vector_size(list)))
    benchmark_end("Collections Operations")
}

fr fr Crypto Module Benchmarks
slay benchmark_crypto_operations() {
    benchmark_start("Crypto Operations")
    
    sus test_data tea = "This is test data for cryptographic operations benchmarking."
    sus iterations normie = benchmark_iterations / 10  fr fr Crypto operations are expensive
    
    bestie i := 0; i < iterations; i++ {
        fr fr SHA256 hashing
        sus hash tea = crypto.sha256(test_data + string.from_int(i))
        
        fr fr Base64 encoding/decoding
        sus encoded tea = crypto.base64_encode(test_data)
        sus decoded tea = crypto.base64_decode(encoded)
        
        fr fr AES encryption/decryption (using a test key)
        sus key tea = "test_key_1234567890123456"  fr fr 32 bytes
        sus encrypted tea = crypto.aes_encrypt(test_data, key)
        sus decrypted tea = crypto.aes_decrypt(encrypted, key)
    }
    
    vibez.spill("Crypto operations completed: " + string.from_int(iterations))
    benchmark_end("Crypto Operations")
}

fr fr Memory Module Benchmarks
slay benchmark_memory_operations() {
    benchmark_start("Memory Operations")
    
    sus allocations normie = benchmark_iterations / 5  fr fr Memory operations are expensive
    sus pointers []memory.Pointer = memory.new_pointer_array(allocations)
    
    fr fr Allocation benchmark
    bestie i := 0; i < allocations; i++ {
        sus size normie = 1024 + (i % 1024)  fr fr Variable size allocations
        sus ptr memory.Pointer = memory.allocate(size)
        pointers[i] = ptr
    }
    
    fr fr Deallocation benchmark
    bestie i := 0; i < allocations; i++ {
        memory.deallocate(pointers[i])
    }
    
    fr fr GC stats
    sus gc_stats memory.GCStats = memory.gc_stats()
    vibez.spill("GC Collections: " + string.from_int(gc_stats.collections))
    vibez.spill("Total Allocated: " + string.from_int(gc_stats.total_allocated))
    
    benchmark_end("Memory Operations")
}

fr fr Async Operations Benchmark
slay benchmark_async_operations() {
    benchmark_start("Async Operations")
    
    sus channel_size normie = 1000
    sus ch async.Channel = async.new_channel(channel_size)
    
    fr fr Spawn producer goroutine
    yolo producer_task(ch, benchmark_iterations)
    
    fr fr Consume messages
    sus received normie = 0
    bestie i := 0; i < benchmark_iterations; i++ {
        sus msg normie = async.channel_receive(ch)
        received = received + msg
    }
    
    vibez.spill("Async operations - messages received: " + string.from_int(received))
    benchmark_end("Async Operations")
}

slay producer_task(ch async.Channel, count normie) {
    bestie i := 0; i < count; i++ {
        async.channel_send(ch, i)
    }
}

fr fr Comprehensive benchmark suite
slay run_comprehensive_benchmarks() {
    vibez.spill("=== CURSED Stdlib Comprehensive Performance Benchmark ===")
    vibez.spill("Iterations per benchmark: " + string.from_int(benchmark_iterations))
    vibez.spill("")
    
    sus total_start_time thicc = timez.now()
    
    benchmark_math_operations()
    benchmark_string_operations()
    benchmark_collections_operations()
    benchmark_crypto_operations()
    benchmark_memory_operations()
    benchmark_async_operations()
    
    sus total_elapsed thicc = timez.now() - total_start_time
    vibez.spill("")
    vibez.spill("=== Benchmark Suite Complete ===")
    vibez.spill("Total time: " + string.from_int(total_elapsed) + "ms")
    vibez.spill("Average time per benchmark: " + string.from_int(total_elapsed / 6) + "ms")
    
    fr fr Memory usage summary
    sus memory_stats memory.GCStats = memory.gc_stats()
    vibez.spill("Final memory stats:")
    vibez.spill("  GC Collections: " + string.from_int(memory_stats.collections))
    vibez.spill("  Total Allocated: " + string.from_int(memory_stats.total_allocated) + " bytes")
    vibez.spill("  Peak Memory: " + string.from_int(memory_stats.peak_memory) + " bytes")
}

slay main() {
    run_comprehensive_benchmarks()
}
