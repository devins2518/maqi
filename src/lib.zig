const std = @import("std");

pub const ImapClient = @import("imap/Client.zig");
pub const ImapParser = @import("imap/Parser.zig");

test "static analysis" {
    @setEvalBranchQuota(std.math.maxInt(u32));
    std.testing.refAllDeclsRecursive(@This());
}
