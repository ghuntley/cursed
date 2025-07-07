slay main() {
    vibez.spill("Testing filesystem operations...")
    
    vibez.spill("Creating file...")
    sus result normie = io_write_file("test_minimal.txt", "Hello")
    vibez.spill("Write result: " + tea(result))
    
    vibez.spill("Reading file...")
    sus content tea = io_read_file("test_minimal.txt")
    vibez.spill("Read content: " + content)
    
    vibez.spill("Test complete!")
}

main()
