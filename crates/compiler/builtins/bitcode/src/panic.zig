const std = @import("std");
const BrocStr = @import("str.zig").BrocStr;
const always_inline = std.builtin.CallOptions.Modifier.always_inline;

// Signals to the host that the program has panicked
extern fn broc_panic(msg: *const BrocStr, tag_id: u32) callconv(.C) void;

pub fn panic_help(msg: []const u8, tag_id: u32) void {
    var str = BrocStr.init(msg.ptr, msg.len);
    broc_panic(&str, tag_id);
}

// must export this explicitly because right now it is not used from zig code
pub fn panic(msg: *const BrocStr, alignment: u32) callconv(.C) void {
    return @call(.{ .modifier = always_inline }, broc_panic, .{ msg, alignment });
}
