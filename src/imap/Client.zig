const std = @import("std");
const Self = @This();
const TcpClient = std.x.net.tcp.Client;
const TcpDomain = std.x.net.tcp.Domain;
const IpAddress = std.x.os.IPv4.Address;

tcp: TcpClient,

pub fn init() !Self {
    @panic("todo");
}

pub fn deinit(self: *Self) void {
    self.tcp.deinit();
}
