const std = @import("std");

// Usage
// var queue = try Queue(i32).init(allocator);
// defer queue.deinit();
//
// try queue.enqueue(1);
// try queue.enqueue(2);
// try queue.enqueue(3);

pub fn Queue(comptime T: type) type {
    return struct {
        const Self = @This();
        const GROWTH_FACTOR = 2;
        const INITIAL_CAPACITY = 8;
        
        buffer: []T,
        head: usize,
        tail: usize,
        len: usize,
        allocator: std.mem.Allocator,

        pub fn init(allocator: std.mem.Allocator) !Self {
            return Self{
                .buffer = try allocator.alloc(T, INITIAL_CAPACITY),
                .head = 0,
                .tail = 0,
                .len = 0,
                .allocator = allocator,
            };
        }

        pub fn deinit(self: *Self) void {
            self.allocator.free(self.buffer);
        }

        fn grow(self: *Self) !void {
            const new_capacity = self.buffer.len * GROWTH_FACTOR;
            var new_buffer = try self.allocator.alloc(T, new_capacity);
            
            // Copy existing elements in order
            var i: usize = 0;
            while (i < self.len) : (i += 1) {
                const old_index = (self.head + i) % self.buffer.len;
                new_buffer[i] = self.buffer[old_index];
            }
            
            // Free old buffer and update state
            self.allocator.free(self.buffer);
            self.buffer = new_buffer;
            self.head = 0;
            self.tail = self.len;
        }

        pub fn enqueue(self: *Self, item: T) !void {
            if (self.len == self.buffer.len) {
                try self.grow();
            }
            
            self.buffer[self.tail] = item;
            self.tail = (self.tail + 1) % self.buffer.len;
            self.len += 1;
        }

        pub fn dequeue(self: *Self) ?T {
            if (self.len == 0) return null;
            
            const item = self.buffer[self.head];
            self.head = (self.head + 1) % self.buffer.len;
            self.len -= 1;
            return item;
        }

        pub fn peek(self: Self) ?T {
            if (self.len == 0) return null;
            return self.buffer[self.head];
        }

        pub fn isEmpty(self: Self) bool {
            return self.len == 0;
        }

        pub fn count(self: Self) usize {
            return self.len;
        }

        pub fn capacity(self: Self) usize {
            return self.buffer.len;
        }

        pub fn clear(self: *Self) void {
            self.head = 0;
            self.tail = 0;
            self.len = 0;
        }
    };
}
