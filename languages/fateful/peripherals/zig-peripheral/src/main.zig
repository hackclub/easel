const std = @import("std");

var STATE: u8 = 0;

export fn init(ports: u8) void {
    _ = ports;
    std.debug.print("Zig init\n", .{});
}

export fn read(port: u8) u8 {
    _ = port;
    std.debug.print("Zig read\n", .{});
    return STATE;
}

export fn write(port: u8, data: u8) void {
    _ = port;
    std.debug.print("Zig write\n", .{});
    STATE = data;  
}

export fn reset() void {
    STATE = 0;
}

export fn name() [*:0] const u8 {
    return "Zig Example";
}
