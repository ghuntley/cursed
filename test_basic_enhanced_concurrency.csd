fr fr Basic Enhanced Concurrency Test - Validate Core Functionality

yeet "concurrenz"  
yeet "vibez"

vibez.spill("🚀 Testing Enhanced Concurrency Core Functions...")

fr fr Test 1: Real goroutine count (should be at least 1 for main)
sus goroutine_count normie = concurrenz.num_goroutines()
vibez.spill("Active goroutines:", goroutine_count)

fr fr Test 2: Real timing mechanism
sus start_time thicc = concurrenz.get_time_ns()
vibez.spill("High-resolution timestamp:", start_time)

fr fr Test 3: Channel creation with real registry
sus channel_id thicc = concurrenz.make_channel()
vibez.spill("Created channel with ID:", channel_id)

fr fr Test 4: Buffered channel creation
sus buffered_id thicc = concurrenz.make_buffered_channel(3)
vibez.spill("Created buffered channel with ID:", buffered_id)

fr fr Test 5: Channel state checking  
vibez.spill("Channel", channel_id, "closed?", concurrenz.is_channel_closed(channel_id))
concurrenz.close_channel(channel_id)
vibez.spill("Channel", channel_id, "closed after close?", concurrenz.is_channel_closed(channel_id))

fr fr Test 6: Real sleep with timing
sus pre_sleep thicc = concurrenz.get_time_ns()
concurrenz.sleep_ms(5)
sus post_sleep thicc = concurrenz.get_time_ns()
sus sleep_duration thicc = post_sleep - pre_sleep
vibez.spill("Sleep duration:", sleep_duration, "nanoseconds")

fr fr Test 7: CPU yield (should not crash)
concurrenz.runtime_yield()
vibez.spill("CPU yield successful")

fr fr Test 8: Memory fence operation
concurrenz.memory_fence()
vibez.spill("Memory fence successful")

vibez.spill("✅ All enhanced concurrency core functions tested successfully!")
