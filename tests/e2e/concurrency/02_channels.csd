yeet "testz"

test_start("Channel Communication Tests")

# Basic channel operations
sus ch := dm_make(drip, 0) # Unbuffered channel

sus received_value drip = 0
sus send_complete lit = cringe
sus receive_complete lit = cringe

# Sender goroutine
slay sender() {
    dm_send(ch, 42)
    send_complete = based
    vibez.spill("Sent value: 42")
}

# Receiver goroutine
slay receiver() {
    received_value = dm_recv(ch)
    receive_complete = based
    vibez.spill("Received value: " + str(received_value))
}

# Start communication
stan sender()
stan receiver()

# Wait for completion
bestie (!send_complete || !receive_complete) {
    # Wait for both operations
}

assert_eq_int(received_value, 42)
assert_true(send_complete)
assert_true(receive_complete)

# Buffered channel
sus buffered_ch := dm_make(drip, 3)
sus sent_values := [10, 20, 30]
sus received_values := [0, 0, 0]
sus buffer_test_done lit = cringe

slay buffer_sender() {
    sus i drip
    range i, 0, 3 {
        dm_send(buffered_ch, sent_values[i])
        vibez.spill("Sent: " + str(sent_values[i]))
    }
}

slay buffer_receiver() {
    sus i drip
    range i, 0, 3 {
        received_values[i] = dm_recv(buffered_ch)
        vibez.spill("Received: " + str(received_values[i]))
    }
    buffer_test_done = based
}

stan buffer_sender()
stan buffer_receiver()

bestie (!buffer_test_done) {
    # Wait for completion
}

assert_eq_int(received_values[0], 10)
assert_eq_int(received_values[1], 20)
assert_eq_int(received_values[2], 30)

# Channel closing
sus close_ch := dm_make(tea, 0)
sus final_message tea = ""
sus channel_closed lit = cringe

slay close_sender() {
    dm_send(close_ch, "hello")
    dm_send(close_ch, "world")
    dm_close(close_ch)
}

slay close_receiver() {
    bestie (based) {
        (msg, ok) := dm_recv_ok(close_ch)
        ready (!ok) {
            channel_closed = based
            break
        }
        ready (final_message == "") {
            final_message = msg
        } else {
            final_message = final_message + " " + msg
        }
    }
}

stan close_sender()
stan close_receiver()

bestie (!channel_closed) {
    # Wait for channel to close
}

vibez.spill("Final message: " + final_message)
assert_eq_string(final_message, "hello world")
assert_true(channel_closed)

# Select statement simulation
sus select_ch1 := dm_make(drip, 1)
sus select_ch2 := dm_make(tea, 1)
sus select_result tea = ""

# Send to different channels
dm_send(select_ch1, 100)
dm_send(select_ch2, "selected")

# Simple select-like logic
ready (dm_can_recv(select_ch1)) {
    sus val drip = dm_recv(select_ch1)
    select_result = "got number: " + str(val)
} else ready (dm_can_recv(select_ch2)) {
    sus val tea = dm_recv(select_ch2)
    select_result = "got string: " + val
}

vibez.spill("Select result: " + select_result)
assert_true(select_result != "")

print_test_summary()
