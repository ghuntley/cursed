// test_project - A CURSED CLI Application
//
// {{description}}

yeet "std::env";
yeet "std::io";

slay main() -> i32 {
    let args = env::args();
    
    lowkey args.len() > 1 {
        let command = args[1];
        
        lowkey command == "help" {
            show_help();
        } flex {
            io::println("Unknown command: " + command);
            io::println("Use 'help' for usage information.");
            return 1;
        }
    } flex {
        io::println("Hello from test_project!");
        io::println("Use 'help' for usage information.");
    }
    
    return 0;
}

slay show_help() {
    io::println("test_project - {{description}}");
    io::println("");
    io::println("Usage:");
    io::println("  test_project [command]");
    io::println("");
    io::println("Commands:");
    io::println("  help    Show this help message");
}
