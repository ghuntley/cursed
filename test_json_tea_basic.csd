# Simple test without imports
vibez.spill("Testing JSON Tea basic functions...")

# Simple marshal function test
slay simple_marshal(data tea) tea {
    bestie data == "based" {
        damn "true"
    } else bestie data == "cap" {
        damn "false"
    } else {
        damn "\"" + data + "\""
    }
}

# Test the function
sus result1 tea = simple_marshal("hello")
vibez.spill("Marshal result:")
vibez.spill(result1)

sus result2 tea = simple_marshal("based")
vibez.spill("Boolean marshal result:")
vibez.spill(result2)

vibez.spill("Test complete!")
