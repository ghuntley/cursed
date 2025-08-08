# Test 4: Different channel types testing
yeet "concurrenz"

vibez.spill("Testing different channel types")

# Integer channel
sus int_ch dm<drip> = make_channel(2)
send_channel(int_ch, 42)
sus int_val drip = recv_channel(int_ch)
vibez.spill("Integer channel:", int_val)

# String channel (if supported)
sus str_ch dm<tea> = make_channel(2)
send_channel(str_ch, "hello")
sus str_val tea = recv_channel(str_ch)
vibez.spill("String channel:", str_val)

# Boolean channel  
sus bool_ch dm<lit> = make_channel(2)
send_channel(bool_ch, based)
sus bool_val lit = recv_channel(bool_ch)
vibez.spill("Boolean channel:", bool_val)

vibez.spill("Channel types test complete")
