slay test_syscalls() {
    sus content tea = fs_real.read_file("test_file.txt")
    vibez.spill("File content:")
    vibez.spill(content)
    
    sus result lit = fs_real.write_file("output.txt", "Hello from CURSED!")
    vibez.spillf("Write result: {}", result)
}

test_syscalls()
