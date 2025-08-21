// Import the minimal main for now during Oracle P2 migration
const minimal = @import("minimal_main.zig");

pub fn main() !void {
    return minimal.main();
}
