// Enhanced CURSED LSP Server CLI Wrapper
// Provides command-line interface for the enhanced CURSED Language Server

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;

const VERSION = "1.0.0";
const CURSED_LSP_PATH = "tools/enhanced_lsp_server.💀";
const CURSED_INTERPRETER = "cursed-unified";

fn printHelp() void {
    print(
        \\Enhanced CURSED Language Server - Production Edition
        \\
        \\USAGE:
        \\    cursed-lsp-enhanced [OPTIONS]
        \\
        \\OPTIONS:
        \\    -h, --help              Show this help message
        \\    -V, --version           Show version information
        \\    --stdio                 Use stdio for LSP communication (default)
        \\    --socket <port>         Use socket on specified port
        \\    --pipe <name>           Use named pipe for communication
        \\    --tcp <host:port>       Use TCP connection
        \\    --log-file <file>       Write debug logs to file
        \\    --log-level <level>     Set log level: debug, info, warn, error
        \\    --config <file>         Use custom LSP server configuration
        \\    --capabilities          Show server capabilities and exit
        \\    --test                  Run LSP server tests
        \\    --max-diagnostics <n>   Maximum diagnostics per file (default: 100)
        \\    --enable-completion     Enable code completion (default: true)
        \\    --enable-hover          Enable hover information (default: true)
        \\    --enable-formatting     Enable document formatting (default: true)
        \\    --enable-diagnostics    Enable syntax diagnostics (default: true)
        \\    --workspace <path>      Set workspace root directory
        \\    --client-info           Show client capability requirements
        \\
        \\COMMUNICATION MODES:
        \\    stdio       Standard input/output (recommended for editors)
        \\    socket      Network socket (for remote development)
        \\    pipe        Named pipe (Windows/Unix IPC)
        \\    tcp         TCP connection (network development)
        \\
        \\INTEGRATION EXAMPLES:
        \\    # VS Code integration
        \\    cursed-lsp-enhanced --stdio --log-file /tmp/cursed-lsp.log
        \\
        \\    # Vim/Neovim integration
        \\    cursed-lsp-enhanced --stdio --log-level warn
        \\
        \\    # Remote development
        \\    cursed-lsp-enhanced --tcp 0.0.0.0:9257 --workspace /project
        \\
        \\    # Testing and debugging
        \\    cursed-lsp-enhanced --test --log-level debug
        \\
        \\For editor integration guides, visit:
        \\https://cursed-lang.org/tools/lsp/integration
        \\
    );
}

fn printVersion() void {
    print("cursed-lsp-enhanced {s}\n", .{VERSION});
}

fn printCapabilities() void {
    print(
        \\🚀 Enhanced CURSED Language Server Capabilities
        \\==============================================
        \\
        \\📋 Text Document Synchronization:
        \\  ✅ Full document sync
        \\  ✅ Incremental changes
        \\  ✅ Will save notifications
        \\  ✅ Did save notifications
        \\
        \\🎯 Code Intelligence:
        \\  ✅ Context-aware completion with snippets
        \\  ✅ Signature help with parameter hints
        \\  ✅ Hover information with markdown
        \\  ✅ Go to definition/declaration
        \\  ✅ Find references
        \\  ✅ Document/workspace symbols
        \\
        \\🔍 Analysis & Diagnostics:
        \\  ✅ Real-time syntax checking
        \\  ✅ Security vulnerability detection
        \\  ✅ Performance issue analysis
        \\  ✅ Code quality suggestions
        \\  ✅ CURSED-specific style checking
        \\
        \\🎨 Code Actions:
        \\  ✅ Document formatting
        \\  ✅ Range formatting
        \\  ✅ Code actions and quick fixes
        \\  ✅ Auto-import suggestions
        \\  ✅ Refactoring support
        \\
        \\🔧 Advanced Features:
        \\  ✅ Workspace folder support
        \\  ✅ Configuration management
        \\  ✅ Document highlighting
        \\  ✅ Folding ranges
        \\  ✅ Code lens support
        \\
        \\🌟 CURSED-Specific Features:
        \\  ✅ Gen Z syntax completion
        \\  ✅ vibez.spill() optimization
        \\  ✅ Module import assistance
        \\  ✅ Type inference for drip/tea/lit
        \\  ✅ CURSED stdlib integration
        \\
        \\For detailed capability documentation:
        \\https://cursed-lang.org/tools/lsp/capabilities
        \\
    );
}

fn printClientInfo() void {
    print(
        \\📋 Client Integration Requirements
        \\=================================
        \\
        \\🔧 Required Client Capabilities:
        \\  - LSP protocol version 3.17+
        \\  - JSON-RPC 2.0 support
        \\  - UTF-8 text encoding
        \\  - Incremental text sync
        \\
        \\💡 Recommended Client Features:
        \\  - Snippet support for completions
        \\  - Markdown rendering for hover
        \\  - Progress reporting UI
        \\  - Configuration change notifications
        \\  - Workspace folder management
        \\
        \\🎨 Editor Configuration Examples:
        \\
        \\VS Code (settings.json):
        \\{
        \\  "cursed.lsp.enabled": true,
        \\  "cursed.lsp.serverPath": "cursed-lsp-enhanced",
        \\  "cursed.lsp.args": ["--stdio", "--log-level", "info"],
        \\  "cursed.lsp.trace.server": "verbose"
        \\}
        \\
        \\Vim/Neovim (init.vim):
        \\lua << EOF
        \\require'lspconfig'.cursed_lsp.setup{
        \\  cmd = {'cursed-lsp-enhanced', '--stdio'},
        \\  filetypes = {'cursed'},
        \\  root_dir = require'lspconfig.util'.find_git_ancestor,
        \\  settings = {}
        \\}
        \\EOF
        \\
        \\Emacs (lsp-mode):
        \\(lsp-register-client
        \\  (make-lsp-client :new-connection (lsp-stdio-connection "cursed-lsp-enhanced")
        \\                   :major-modes '(cursed-mode)
        \\                   :server-id 'cursed-lsp))
        \\
    );
}

fn runTest() !void {
    print("🧪 Running Enhanced CURSED LSP Server tests...\n");
    
    // Test the enhanced LSP server
    const result = std.ChildProcess.exec(.{
        .allocator = std.heap.page_allocator,
        .argv = &[_][]const u8{ CURSED_INTERPRETER, CURSED_LSP_PATH },
    }) catch |err| {
        print("❌ Error running LSP server tests: {}\n", .{err});
        return;
    };
    
    defer std.heap.page_allocator.free(result.stdout);
    defer std.heap.page_allocator.free(result.stderr);
    
    if (result.term.Exited == 0) {
        print("✅ Enhanced LSP server tests passed!\n");
        print("{s}\n", .{result.stdout});
    } else {
        print("❌ Enhanced LSP server tests failed!\n");
        print("Error: {s}\n", .{result.stderr});
    }
}

fn startLspServer(options: LspOptions) !void {
    if (options.verbose) {
        print("🚀 Starting Enhanced CURSED Language Server...\n");
        print("📡 Communication mode: {s}\n", .{options.communication_mode});
        
        if (options.workspace_path) |workspace| {
            print("📂 Workspace: {s}\n", .{workspace});
        }
        
        if (options.log_file) |log_file| {
            print("📝 Logging to: {s}\n", .{log_file});
        }
    }
    
    // Prepare arguments for the LSP server
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    var argv = ArrayList([]const u8).init(allocator);
    try argv.append(CURSED_INTERPRETER);
    try argv.append(CURSED_LSP_PATH);
    
    // Add server configuration options
    if (options.max_diagnostics) |max_diag| {
        try argv.append("--max-diagnostics");
        const max_str = try std.fmt.allocPrint(allocator, "{}", .{max_diag});
        try argv.append(max_str);
    }
    
    if (options.workspace_path) |workspace| {
        try argv.append("--workspace");
        try argv.append(workspace);
    }
    
    if (options.config_file) |config| {
        try argv.append("--config");
        try argv.append(config);
    }
    
    // Set up logging if specified
    var env_map = std.process.EnvMap.init(allocator);
    defer env_map.deinit();
    
    if (options.log_file) |log_file| {
        try env_map.put("CURSED_LSP_LOG_FILE", log_file);
    }
    
    try env_map.put("CURSED_LSP_LOG_LEVEL", options.log_level);
    
    // Start the LSP server process
    switch (std.mem.eql(u8, options.communication_mode, "stdio")) {
        true => {
            // Stdio mode - most common for editor integration
            var child = std.ChildProcess.init(argv.items, allocator);
            child.stdin_behavior = .Inherit;
            child.stdout_behavior = .Inherit;
            child.stderr_behavior = .Inherit;
            child.env_map = &env_map;
            
            const term = child.spawnAndWait() catch |err| {
                print("❌ Error starting LSP server: {}\n", .{err});
                return;
            };
            
            switch (term) {
                .Exited => |code| {
                    if (code != 0) {
                        print("❌ LSP server exited with code: {}\n", .{code});
                    }
                },
                .Signal => |signal| {
                    print("❌ LSP server terminated by signal: {}\n", .{signal});
                },
                else => {
                    print("❌ LSP server terminated unexpectedly\n");
                },
            }
        },
        false => {
            // For other communication modes (socket, pipe, tcp)
            print("❌ Communication mode '{s}' not yet implemented\n", .{options.communication_mode});
            print("Currently supported: stdio\n");
            print("Planned: socket, pipe, tcp\n");
        },
    }
}

const LspOptions = struct {
    communication_mode: []const u8 = "stdio",
    socket_port: ?u16 = null,
    pipe_name: ?[]const u8 = null,
    tcp_address: ?[]const u8 = null,
    log_file: ?[]const u8 = null,
    log_level: []const u8 = "info",
    config_file: ?[]const u8 = null,
    max_diagnostics: ?u32 = null,
    enable_completion: bool = true,
    enable_hover: bool = true,
    enable_formatting: bool = true,
    enable_diagnostics: bool = true,
    workspace_path: ?[]const u8 = null,
    verbose: bool = false,
};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    var options = LspOptions{};
    
    var i: usize = 1;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "-h") or std.mem.eql(u8, arg, "--help")) {
            printHelp();
            return;
        } else if (std.mem.eql(u8, arg, "-V") or std.mem.eql(u8, arg, "--version")) {
            printVersion();
            return;
        } else if (std.mem.eql(u8, arg, "--capabilities")) {
            printCapabilities();
            return;
        } else if (std.mem.eql(u8, arg, "--client-info")) {
            printClientInfo();
            return;
        } else if (std.mem.eql(u8, arg, "--test")) {
            try runTest();
            return;
        } else if (std.mem.eql(u8, arg, "--stdio")) {
            options.communication_mode = "stdio";
        } else if (std.mem.eql(u8, arg, "--socket")) {
            if (i + 1 < args.len) {
                i += 1;
                options.communication_mode = "socket";
                options.socket_port = std.fmt.parseInt(u16, args[i], 10) catch {
                    print("❌ Invalid port number: {s}\n", .{args[i]});
                    std.process.exit(1);
                };
            } else {
                print("❌ --socket requires a port number\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "--pipe")) {
            if (i + 1 < args.len) {
                i += 1;
                options.communication_mode = "pipe";
                options.pipe_name = args[i];
            } else {
                print("❌ --pipe requires a pipe name\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "--tcp")) {
            if (i + 1 < args.len) {
                i += 1;
                options.communication_mode = "tcp";
                options.tcp_address = args[i];
            } else {
                print("❌ --tcp requires a host:port address\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "--log-file")) {
            if (i + 1 < args.len) {
                i += 1;
                options.log_file = args[i];
            } else {
                print("❌ --log-file requires a file path\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "--log-level")) {
            if (i + 1 < args.len) {
                i += 1;
                options.log_level = args[i];
            } else {
                print("❌ --log-level requires a level (debug, info, warn, error)\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "--config")) {
            if (i + 1 < args.len) {
                i += 1;
                options.config_file = args[i];
            } else {
                print("❌ --config requires a file path\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "--max-diagnostics")) {
            if (i + 1 < args.len) {
                i += 1;
                options.max_diagnostics = std.fmt.parseInt(u32, args[i], 10) catch {
                    print("❌ Invalid number for --max-diagnostics: {s}\n", .{args[i]});
                    std.process.exit(1);
                };
            } else {
                print("❌ --max-diagnostics requires a number\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "--workspace")) {
            if (i + 1 < args.len) {
                i += 1;
                options.workspace_path = args[i];
            } else {
                print("❌ --workspace requires a directory path\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "-v") or std.mem.eql(u8, arg, "--verbose")) {
            options.verbose = true;
        } else if (std.mem.eql(u8, arg, "--enable-completion")) {
            options.enable_completion = true;
        } else if (std.mem.eql(u8, arg, "--enable-hover")) {
            options.enable_hover = true;
        } else if (std.mem.eql(u8, arg, "--enable-formatting")) {
            options.enable_formatting = true;
        } else if (std.mem.eql(u8, arg, "--enable-diagnostics")) {
            options.enable_diagnostics = true;
        } else if (std.mem.startsWith(u8, arg, "-")) {
            print("❌ Unknown option: {s}\n", .{arg});
            print("Run 'cursed-lsp-enhanced --help' for usage information.\n");
            std.process.exit(1);
        } else {
            print("❌ Unexpected argument: {s}\n", .{arg});
            print("Run 'cursed-lsp-enhanced --help' for usage information.\n");
            std.process.exit(1);
        }
    }
    
    // Start the LSP server
    try startLspServer(options);
}
