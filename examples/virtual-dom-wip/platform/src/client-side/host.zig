const std = @import("std");
const str = @import("str");
const builtin = @import("builtin");
const BrocStr = str.BrocStr;

const Align = extern struct { a: usize, b: usize };
extern fn malloc(size: usize) callconv(.C) ?*align(@alignOf(Align)) anyopaque;
extern fn realloc(c_ptr: [*]align(@alignOf(Align)) u8, size: usize) callconv(.C) ?*anyopaque;
extern fn free(c_ptr: [*]align(@alignOf(Align)) u8) callconv(.C) void;
extern fn memcpy(dest: *anyopaque, src: *anyopaque, count: usize) *anyopaque;

export fn broc_alloc(size: usize, alignment: u32) callconv(.C) ?*anyopaque {
    _ = alignment;

    return malloc(size);
}

export fn broc_realloc(c_ptr: *anyopaque, new_size: usize, old_size: usize, alignment: u32) callconv(.C) ?*anyopaque {
    _ = old_size;
    _ = alignment;

    return realloc(@alignCast(@alignOf(Align), @ptrCast([*]u8, c_ptr)), new_size);
}

export fn broc_dealloc(c_ptr: *anyopaque, alignment: u32) callconv(.C) void {
    _ = alignment;

    free(@alignCast(@alignOf(Align), @ptrCast([*]u8, c_ptr)));
}

export fn broc_memcpy(dest: *anyopaque, src: *anyopaque, count: usize) callconv(.C) void {
    _ = memcpy(dest, src, count);
}

export fn broc_panic(message: BrocStr, tag_id: u32) callconv(.C) void {
    _ = tag_id;
    const msg = @ptrCast([*:0]const u8, c_ptr);
    const stderr = std.io.getStdErr().writer();
    stderr.print("Application crashed with message\n\n    {s}\n\nShutting down\n", .{msg}) catch unreachable;
    std.process.exit(0);
}

const BrocList = extern struct {
    bytes: ?[*]u8,
    length: usize,
    capacity: usize,
};

const FromHost = extern struct {
    eventHandlerId: usize,
    eventJsonList: ?BrocList,
    eventPlatformState: ?*anyopaque,
    initJson: BrocList,
    isInitEvent: bool,
};

const ToHost = extern struct {
    platformState: *anyopaque,
    eventPreventDefault: bool,
    eventStopPropagation: bool,
};

extern fn broc__main_1_exposed(FromHost) callconv(.C) ToHost;

var platformState: ?*anyopaque = null;

// Called from JS
export fn broc_vdom_init(init_pointer: ?[*]u8, init_length: usize, init_capacity: usize) callconv(.C) void {
    const init_json = BrocList{
        .bytes = init_pointer,
        .length = init_length,
        .capacity = init_capacity,
    };
    const from_host = FromHost{
        .eventHandlerId = std.math.maxInt(usize),
        .eventJsonList = null,
        .eventPlatformState = null,
        .initJson = init_json,
        .isInitEvent = true,
    };
    const to_host = broc__main_1_exposed(from_host);
    platformState = to_host.platformState;
}

// Called from JS
export fn broc_dispatch_event(list_ptr: ?[*]u8, list_length: usize, handler_id: usize) usize {
    const json_list = BrocList{
        .bytes = list_ptr,
        .length = list_length,
        .capacity = list_length,
    };
    const from_host = FromHost{
        .eventHandlerId = handler_id,
        .eventJsonList = json_list,
        .eventPlatformState = platformState,
        .initJson = null,
        .isInitEvent = false,
    };
    const to_host = broc__main_1_exposed(from_host);
    platformState = to_host.platformState;
    return to_host.eventPreventDefault << 1 | to_host.eventStopPropagation;
}
