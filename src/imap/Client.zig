const std = @import("std");
const Self = @This();
const TcpClient = std.x.net.tcp.Client;
const TcpDomain = std.x.net.tcp.Domain;

tcp: TcpClient,

pub fn init(domain: TcpDomain) !Self {
    return Self{
        .tcp = try TcpClient.init(domain, .{ .nonblocking = true }),
    };
}

pub fn deinit(self: *Self) void {
    self.tcp.deinit();
}
