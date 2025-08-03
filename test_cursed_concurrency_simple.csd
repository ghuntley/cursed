fr fr Simple CURSED concurrency test
yeet "testz"

test_start("Simple Concurrency Test")

fr fr Test basic channel operations
sus ch dm<normie> = dm_create<normie>(2)
dm_send(ch, 42)
dm_send(ch, 43)

sus value1 normie = dm_recv(ch)
sus value2 normie = dm_recv(ch)

assert_eq_int(value1, 42)
assert_eq_int(value2, 43)

vibez.spill("Channel operations working!")

fr fr Test simple goroutine
sus result normie = 0

stan {
    result = 99
    vibez.spill("Goroutine executed!")
}

fr fr Wait for goroutine
yolo()

assert_eq_int(result, 99)

vibez.spill("Goroutine execution working!")

print_test_summary()
