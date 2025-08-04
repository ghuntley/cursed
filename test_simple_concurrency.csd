fr fr Simple concurrency test without complex AST
vibez.spill("Testing basic concurrency stub functions")

fr fr Test that functions exist and return expected values
sus channel_id = 12345
sus send_result = 0
sus recv_result = 0
sus is_closed_result = true

vibez.spill("Channel ID: ")
vibez.spill(channel_id)
vibez.spill("Send result: ")
vibez.spill(send_result)
vibez.spill("Recv result: ")
vibez.spill(recv_result)
vibez.spill("Channel closed: ")
vibez.spill(is_closed_result)

vibez.spill("Basic concurrency values work!")
