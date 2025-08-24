# Concurrency Modules Functionality Test
yeet "concurrenz"
yeet "asyncz"
yeet "channelz"

vibez.spill("=== CONCURRENCY MODULES TEST ===")

# Test concurrenz module
vibez.spill("Testing concurrenz.spawn...")
sus task_id drip = concurrenz.spawn({
    vibez.spill("Goroutine executing!")
})
vibez.spill("concurrenz.spawn task_id:", task_id)

vibez.spill("Testing concurrenz.wait...")
sus wait_result lit = concurrenz.wait(task_id)
vibez.spill("concurrenz.wait result:", wait_result)

vibez.spill("Testing concurrenz.get_thread_count...")
sus thread_count drip = concurrenz.get_thread_count()
vibez.spill("concurrenz.get_thread_count:", thread_count)

# Test channelz module  
vibez.spill("Testing channelz.create...")
sus channel chan<drip> = channelz.create()
vibez.spill("channelz.create completed")

vibez.spill("Testing channelz.send...")
sus send_result lit = channelz.send(channel, 42)
vibez.spill("channelz.send result:", send_result)

vibez.spill("Testing channelz.receive...")
sus received_value drip = channelz.receive(channel)
vibez.spill("channelz.receive result:", received_value)

vibez.spill("Testing channelz.close...")
sus close_result lit = channelz.close(channel)
vibez.spill("channelz.close result:", close_result)

# Test asyncz module
vibez.spill("Testing asyncz.create_task...")
sus async_task dict = asyncz.create_task({
    vibez.spill("Async task executing!")
    damn 123
})
vibez.spill("asyncz.create_task completed")

vibez.spill("Testing asyncz.await...")
sus await_result drip = asyncz.await(async_task)
vibez.spill("asyncz.await result:", await_result)

vibez.spill("Testing asyncz.sleep...")
sus sleep_result lit = asyncz.sleep(100)  # 100ms
vibez.spill("asyncz.sleep result:", sleep_result)

vibez.spill("=== CONCURRENCY MODULES TEST COMPLETE ===")
