# Basic channel communication test for CURSED concurrency system

slay main() {
    vibez.spill("Starting basic channel test")
    
    # Create a buffered channel
    sus ch dm<drip> = dm_new<drip>(2)
    
    # Send values to channel
    dm_send(ch, 42)
    dm_send(ch, 43)
    vibez.spill("Sent values to channel")
    
    # Receive values from channel
    sus val1 drip = dm_recv(ch)
    sus val2 drip = dm_recv(ch)
    
    expect val1 == 42
    expect val2 == 43
    
    vibez.spill("Basic channel test passed!")
}
