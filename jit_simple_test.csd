// Simple JIT test to verify execution engine functionality

vibez.spill("Starting JIT simple test")

sus x normie = 42
sus y normie = 24
sus result normie = x + y

vibez.spillf("x = {}, y = {}, x + y = {}", x, y, result)

sus message tea = "Hello"
sus name tea = "JIT"
sus greeting tea = message + " " + name + "!"
vibez.spill(greeting)

vibez.spill("JIT simple test complete")
