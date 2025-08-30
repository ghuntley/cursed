fr fr Simple test of expanded stdlib modules
yeet "fs"
yeet "io"

slay test_fs_basic() {
    sus content tea = fs.read_file("test.txt")
    io.println(content)
}

slay test_fs_exists() {
    sus exists lit = fs.file_exists("config.json")
    io.println(exists)
}

slay test_fs_size() {
    sus size thicc = fs.get_file_size("data.csv")
    io.println(size)
}

slay main_character() {
    test_fs_basic()
    test_fs_exists()
    test_fs_size()
}
