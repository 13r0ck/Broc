const std = @import("std");
const builtin = @import("builtin");

const Atomic = std.atomic.Atomic;

const O_RDWR: c_int = 2;
const O_CREAT: c_int = 64;

pub const PROT_WRITE: c_int = 2;
pub const MAP_SHARED: c_int = 0x0001;

// IMPORTANT: shared memory object names must begin with / and contain no other slashes!
var SHARED_BUFFER: []u8 = undefined;

pub fn setSharedBuffer(ptr: [*]u8, length: usize) callconv(.C) usize {
    SHARED_BUFFER = ptr[0..length];

    // the rust side expects that a pointer is returned
    return 0;
}

pub fn expectFailedStartSharedBuffer() callconv(.C) [*]u8 {
    return SHARED_BUFFER.ptr;
}

pub fn expectFailedStartSharedFile() callconv(.C) [*]u8 {
    // IMPORTANT: shared memory object names must begin with / and contain no other slashes!
    var name: [100]u8 = undefined;
    _ = std.fmt.bufPrint(name[0..100], "/broc_expect_buffer_{}\x00", .{broc_getppid()}) catch unreachable;

    if (builtin.os.tag == .macos or builtin.os.tag == .linux) {
        const shared_fd = broc_shm_open(@ptrCast(*const i8, &name), O_RDWR | O_CREAT, 0o666);

        const length = 4096;

        const shared_ptr = broc_mmap(
            null,
            length,
            PROT_WRITE,
            MAP_SHARED,
            shared_fd,
            0,
        );

        const ptr = @ptrCast([*]u8, shared_ptr);

        return ptr;
    } else {
        unreachable;
    }
}

extern fn broc_shm_open(name: *const i8, oflag: c_int, mode: c_uint) c_int;
extern fn broc_mmap(addr: ?*anyopaque, length: c_uint, prot: c_int, flags: c_int, fd: c_int, offset: c_uint) *anyopaque;
extern fn broc_getppid() c_int;

pub fn readSharedBufferEnv() callconv(.C) void {
    if (builtin.os.tag == .macos or builtin.os.tag == .linux) {

        // IMPORTANT: shared memory object names must begin with / and contain no other slashes!
        var name: [100]u8 = undefined;
        _ = std.fmt.bufPrint(name[0..100], "/broc_expect_buffer_{}\x00", .{broc_getppid()}) catch unreachable;

        const shared_fd = broc_shm_open(@ptrCast(*const i8, &name), O_RDWR | O_CREAT, 0o666);
        const length = 4096;

        const shared_ptr = broc_mmap(
            null,
            length,
            PROT_WRITE,
            MAP_SHARED,
            shared_fd,
            0,
        );

        const ptr = @ptrCast([*]u8, shared_ptr);

        SHARED_BUFFER = ptr[0..length];
    }
}

pub fn notifyParent(shared_buffer: [*]u8, tag: u32) callconv(.C) void {
    if (builtin.os.tag == .macos or builtin.os.tag == .linux) {
        const usize_ptr = @ptrCast([*]u32, @alignCast(@alignOf(usize), shared_buffer));
        const atomic_ptr = @ptrCast(*Atomic(u32), &usize_ptr[5]);
        atomic_ptr.storeUnchecked(tag);

        // wait till the parent is done before pbroceeding
        const Ordering = std.atomic.Ordering;
        while (atomic_ptr.load(Ordering.Acquire) != 0) {
            std.atomic.spinLoopHint();
        }
    }
}

pub fn notifyParentExpect(shared_buffer: [*]u8) callconv(.C) void {
    notifyParent(shared_buffer, 1);
}

pub fn notifyParentDbg(shared_buffer: [*]u8) callconv(.C) void {
    notifyParent(shared_buffer, 2);
}
