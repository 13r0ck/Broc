const std = @import("std");
const builtin = @import("builtin");
const str = @import("str");
const BrocStr = str.BrocStr;
const testing = std.testing;
const expectEqual = testing.expectEqual;
const expect = testing.expect;
const maxInt = std.math.maxInt;

comptime {
    // This is a workaround for https://github.com/ziglang/zig/issues/8218
    // which is only necessary on macOS.
    //
    // Once that issue is fixed, we can undo the changes in
    // 177cf12e0555147faa4d436e52fc15175c2c4ff0 and go back to passing
    // -fcompiler-rt in link.rs instead of doing this. Note that this
    // workaround is present in many host.zig files, so make sure to undo
    // it everywhere!
    if (builtin.os.tag == .macos) {
        _ = @import("compiler_rt");
    }
}

const mem = std.mem;
const Allocator = mem.Allocator;

const Program = extern struct { init: BrocStr, update: Unit, view: Unit };

extern fn broc__mainForHost_1_exposed() Program;
extern fn broc__mainForHost_size() i64;

const ConstModel = [*]const u8;
const MutModel = [*]u8;

extern fn broc__mainForHost_0_caller([*]u8, [*]u8, MutModel) void;
extern fn broc__mainForHost_0_size() i64;
extern fn broc__mainForHost_0_result_size() i64;

fn allocate_model(allocator: *Allocator) MutModel {
    const size = broc__mainForHost_0_result_size();
    const raw_output = allocator.allocAdvanced(u8, @alignOf(u64), @intCast(usize, size), .at_least) catch unreachable;
    var output = @ptrCast([*]u8, raw_output);

    return output;
}

fn init(allocator: *Allocator) ConstModel {
    const closure: [*]u8 = undefined;
    const output = allocate_model(allocator);

    broc__mainForHost_0_caller(closure, closure, output);

    return output;
}

extern fn broc__mainForHost_1_caller(ConstModel, *const BrocStr, [*]u8, MutModel) void;
extern fn broc__mainForHost_1_size() i64;
extern fn broc__mainForHost_1_result_size() i64;

fn update(allocator: *Allocator, model: ConstModel, msg: BrocStr) ConstModel {
    const closure: [*]u8 = undefined;
    const output = allocate_model(allocator);

    broc__mainForHost_1_caller(model, &msg, closure, output);

    return output;
}

extern fn broc__mainForHost_2_caller(ConstModel, [*]u8, *BrocStr) void;
extern fn broc__mainForHost_2_size() i64;
extern fn broc__mainForHost_2_result_size() i64;

fn view(input: ConstModel) BrocStr {
    const closure: [*]u8 = undefined;
    var output: BrocStr = undefined;

    broc__mainForHost_2_caller(input, closure, &output);

    return output;
}

fn print_output(viewed: BrocStr) void {
    const stdout = std.io.getStdOut().writer();

    for (viewed.asSlice()) |char| {
        stdout.print("{c}", .{char}) catch unreachable;
    }

    stdout.print("\n", .{}) catch unreachable;
}

const Align = 2 * @alignOf(usize);
extern fn malloc(size: usize) callconv(.C) ?*align(Align) anyopaque;
extern fn realloc(c_ptr: [*]align(Align) u8, size: usize) callconv(.C) ?*anyopaque;
extern fn free(c_ptr: [*]align(Align) u8) callconv(.C) void;
extern fn memcpy(dst: [*]u8, src: [*]u8, size: usize) callconv(.C) void;
extern fn memset(dst: [*]u8, value: i32, size: usize) callconv(.C) void;

const DEBUG: bool = false;

export fn broc_alloc(size: usize, alignment: u32) callconv(.C) ?*anyopaque {
    if (DEBUG) {
        var ptr = malloc(size);
        const stdout = std.io.getStdOut().writer();
        stdout.print("alloc:   {d} (alignment {d}, size {d})\n", .{ ptr, alignment, size }) catch unreachable;
        return ptr;
    } else {
        return malloc(size);
    }
}

export fn broc_realloc(c_ptr: *anyopaque, new_size: usize, old_size: usize, alignment: u32) callconv(.C) ?*anyopaque {
    if (DEBUG) {
        const stdout = std.io.getStdOut().writer();
        stdout.print("realloc: {d} (alignment {d}, old_size {d})\n", .{ c_ptr, alignment, old_size }) catch unreachable;
    }

    return realloc(@alignCast(Align, @ptrCast([*]u8, c_ptr)), new_size);
}

export fn broc_dealloc(c_ptr: *anyopaque, alignment: u32) callconv(.C) void {
    if (DEBUG) {
        const stdout = std.io.getStdOut().writer();
        stdout.print("dealloc: {d} (alignment {d})\n", .{ c_ptr, alignment }) catch unreachable;
    }

    free(@alignCast(Align, @ptrCast([*]u8, c_ptr)));
}

export fn broc_panic(c_ptr: *anyopaque, tag_id: u32) callconv(.C) void {
    _ = tag_id;

    const stderr = std.io.getStdErr().writer();
    const msg = @ptrCast([*:0]const u8, c_ptr);
    stderr.print("Application crashed with message\n\n    {s}\n\nShutting down\n", .{msg}) catch unreachable;
    std.process.exit(0);
}

export fn broc_memcpy(dst: [*]u8, src: [*]u8, size: usize) callconv(.C) void {
    return memcpy(dst, src, size);
}

export fn broc_memset(dst: [*]u8, value: i32, size: usize) callconv(.C) void {
    return memset(dst, value, size);
}

extern fn kill(pid: c_int, sig: c_int) c_int;
extern fn shm_open(name: *const i8, oflag: c_int, mode: c_uint) c_int;
extern fn mmap(addr: ?*anyopaque, length: c_uint, prot: c_int, flags: c_int, fd: c_int, offset: c_uint) *anyopaque;
extern fn getppid() c_int;

fn broc_getppid() callconv(.C) c_int {
    return getppid();
}

fn broc_getppid_windows_stub() callconv(.C) c_int {
    return 0;
}

fn broc_shm_open(name: *const i8, oflag: c_int, mode: c_uint) callconv(.C) c_int {
    return shm_open(name, oflag, mode);
}
fn broc_mmap(addr: ?*anyopaque, length: c_uint, prot: c_int, flags: c_int, fd: c_int, offset: c_uint) callconv(.C) *anyopaque {
    return mmap(addr, length, prot, flags, fd, offset);
}

comptime {
    if (builtin.os.tag == .macos or builtin.os.tag == .linux) {
        @export(broc_getppid, .{ .name = "broc_getppid", .linkage = .Strong });
        @export(broc_mmap, .{ .name = "broc_mmap", .linkage = .Strong });
        @export(broc_shm_open, .{ .name = "broc_shm_open", .linkage = .Strong });
    }

    if (builtin.os.tag == .windows) {
        @export(broc_getppid_windows_stub, .{ .name = "broc_getppid", .linkage = .Strong });
    }
}

const Unit = extern struct {};

pub export fn main() callconv(.C) u8 {
    var timer = std.time.Timer.start() catch unreachable;

    const program = broc__mainForHost_1_exposed();

    call_the_closure(program);

    const nanos = timer.read();
    const seconds = (@intToFloat(f64, nanos) / 1_000_000_000.0);

    const stderr = std.io.getStdErr().writer();
    stderr.print("runtime: {d:.3}ms\n", .{seconds * 1000}) catch unreachable;

    return 0;
}

fn to_seconds(tms: std.os.timespec) f64 {
    return @intToFloat(f64, tms.tv_sec) + (@intToFloat(f64, tms.tv_nsec) / 1_000_000_000.0);
}

fn call_the_closure(program: Program) void {
    _ = program;

    var allocator = std.heap.page_allocator;
    const stdin = std.io.getStdIn().reader();

    var buf: [1000]u8 = undefined;

    var model = init(&allocator);

    while (true) {
        print_output(view(model));

        const line = (stdin.readUntilDelimiterOrEof(buf[0..], '\n') catch unreachable) orelse return;

        if (line.len == 1 and line[0] == 'q') {
            return;
        }

        const to_append = BrocStr.init(line.ptr, line.len);

        model = update(&allocator, model, to_append);
    }

    // The closure returns result, nothing interesting to do with it
    return;
}

pub export fn broc_fx_putInt(int: i64) i64 {
    const stdout = std.io.getStdOut().writer();

    stdout.print("{d}", .{int}) catch unreachable;

    stdout.print("\n", .{}) catch unreachable;

    return 0;
}

export fn broc_fx_putLine(brocPath: str.BrocStr) callconv(.C) void {
    const stdout = std.io.getStdOut().writer();

    for (brocPath.asSlice()) |char| {
        stdout.print("{c}", .{char}) catch unreachable;
    }

    stdout.print("\n", .{}) catch unreachable;
}

const GetInt = extern struct {
    value: i64,
    error_code: bool,
    is_error: bool,
};

comptime {
    if (@sizeOf(usize) == 8) {
        @export(broc_fx_getInt_64bit, .{ .name = "broc_fx_getInt" });
    } else {
        @export(broc_fx_getInt_32bit, .{ .name = "broc_fx_getInt" });
    }
}

fn broc_fx_getInt_64bit() callconv(.C) GetInt {
    if (broc_fx_getInt_help()) |value| {
        const get_int = GetInt{ .is_error = false, .value = value, .error_code = false };
        return get_int;
    } else |err| switch (err) {
        error.InvalidCharacter => {
            return GetInt{ .is_error = true, .value = 0, .error_code = false };
        },
        else => {
            return GetInt{ .is_error = true, .value = 0, .error_code = true };
        },
    }

    return 0;
}

fn broc_fx_getInt_32bit(output: *GetInt) callconv(.C) void {
    if (broc_fx_getInt_help()) |value| {
        const get_int = GetInt{ .is_error = false, .value = value, .error_code = false };
        output.* = get_int;
    } else |err| switch (err) {
        error.InvalidCharacter => {
            output.* = GetInt{ .is_error = true, .value = 0, .error_code = false };
        },
        else => {
            output.* = GetInt{ .is_error = true, .value = 0, .error_code = true };
        },
    }

    return;
}

fn broc_fx_getInt_help() !i64 {
    const stdin = std.io.getStdIn().reader();
    var buf: [40]u8 = undefined;

    // make sure to strip `\r` on windows
    const raw_line: []u8 = (try stdin.readUntilDelimiterOrEof(&buf, '\n')) orelse "";
    const line = std.mem.trimRight(u8, raw_line, &std.ascii.spaces);

    return std.fmt.parseInt(i64, line, 10);
}
