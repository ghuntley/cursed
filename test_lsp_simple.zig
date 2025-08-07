const std = @import("std");

pub fn main() !void {
    std.log.info("CURSED LSP Server v1.0.0", .{});
    
    _ = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();
    
    // Simple initialization response
    const init_response = 
        \\{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"textDocumentSync":1,"completionProvider":{"triggerCharacters":["."]},"hoverProvider":true}}}
    ;
    
    try stdout.print("Content-Length: {}\r\n\r\n{s}", .{ init_response.len, init_response });
}
