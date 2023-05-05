const std = @import("std");
const lib = @import("lib.zig");

pub fn main() anyerror!void {
    const client = try lib.ImapClient.init();
    _ = client;
    while (true) {}
}
