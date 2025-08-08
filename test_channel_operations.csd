# Test 2: Channel operations with dm<T> syntax
yeet "concurrenz"

vibez.spill("Testing channel operations")

# Create a channel with buffer size 3
sus ch dm<drip> = make_channel(3)

# Test sending data
send_channel(ch, 42)
send_channel(ch, 100)
vibez.spill("Sent values to channel")

# Test receiving data
sus value1 drip = recv_channel(ch)
sus value2 drip = recv_channel(ch)

vibez.spill("Received values:", value1, value2)

# Close the channel
close_channel(ch)
vibez.spill("Channel operations complete")
