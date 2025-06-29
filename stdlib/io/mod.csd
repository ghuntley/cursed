// Standard I/O library
fn print(message: string) -> void {
    println!(message);
}

fn read_line() -> string {
    return input();
}

fn write_file(path: string, content: string) -> bool {
    return write(path, content);
}

fn read_file(path: string) -> string {
    return read(path);
}
