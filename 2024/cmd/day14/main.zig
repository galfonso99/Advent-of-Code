const std = @import("std");

const length: i32 = 103;
const width: i32 = 101;
const len: usize = 500;

const movement = struct {
    positions: [500][2]i32,
    velocities: [500][2]i32
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const struct_of_arrays = try parseFile(allocator);
    part1(struct_of_arrays);
}

pub fn parseFile(allocator: std.mem.Allocator) !movement {
    // const path = "../../input/test.txt";
    const path = "../../input/day14.txt";
    const content = try std.fs.cwd().readFileAlloc(allocator, path, 1024 * 1024);
    defer allocator.free(content);
    var lines = std.mem.splitSequence(u8, content, "\n");
    var positions: [len][2]i32 = undefined;
    var velocities: [len][2]i32 = undefined;
    var i: usize = 0;
    while (lines.next()) |line| {
        if (line.len == 0) {break;}
        // Split into position and velocity parts
        var parts = std.mem.splitSequence(u8, line, " ");
        const pos_part = parts.next() orelse return error.InvalidFormat;
        const vel_part = parts.next() orelse return error.InvalidFormat;
        
        // Parse position
        var pos = std.mem.splitSequence(u8, pos_part[2..], ","); // Skip "p="
        const pos_x = try std.fmt.parseInt(i32, pos.next() orelse return error.InvalidFormat, 10);
        const pos_y = try std.fmt.parseInt(i32, pos.next() orelse return error.InvalidFormat, 10);
        
        // Parse velocity
        var vel = std.mem.splitSequence(u8, vel_part[2..], ","); // Skip "v="
        const vel_x = try std.fmt.parseInt(i32, vel.next() orelse return error.InvalidFormat, 10);
        const vel_y = try std.fmt.parseInt(i32, vel.next() orelse return error.InvalidFormat, 10);

        positions[i][0] = pos_x;
        positions[i][1] = pos_y;
        velocities[i][0] = vel_x;
        velocities[i][1] = vel_y;

        i += 1;
    }
    return .{.positions = positions, .velocities = velocities };
}

// For each robot multiply its velocity by 100 for x and y
// Then take the modulo of the result by the width/length
// Then treat this remainder as one movement
// Then using the obtained new position calculate where it would be in quadrant 1, 2, 3 or 4
// Then add one to that quadrant count

fn part1(mov: movement) void {
    const pos = mov.positions;
    const vel = mov.velocities;
    var quadrants = [_]i32{0} ** 5;
    const middle_x = @divTrunc(width, 2);
    const middle_y = @divTrunc(length, 2);
    for (0..len) |i| {
        const overflow_pos_x = vel[i][0] * 100; 
        const overflow_pos_y = vel[i][1] * 100; 
        const mov_x = @mod(overflow_pos_x, width);
        const mov_y = @mod(overflow_pos_y, length);
        const pos_x = @mod(pos[i][0] + mov_x + width, width);
        const pos_y = @mod(pos[i][1] + mov_y + length, length);
        const quadrant_index = whichQuadrant(pos_x, pos_y, middle_x, middle_y);
        quadrants[quadrant_index - 1] += 1;
    }
    for (0..4) |i| {
        std.debug.print("quadrant {d}: {d}\n", .{i, quadrants[i]});
    }
    std.debug.print("Factor {d}\n", .{quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]});
}

fn whichQuadrant(x: i32, y: i32, mid_x: i32, mid_y: i32) usize {
    if (x < mid_x and y < mid_y) { 
        return 1;
    } else if (x > mid_x and y < mid_y) {
        return 2;
    } else if (x < mid_x and y > mid_y) {
        return 3;
    } else if (x > mid_x and y > mid_y) {
        return 4;
    } else {
        return 5;
    }
}

// For each second that passes check if quadrant 1 equals quadrant 2 and quadrant 3 
// equals quadrant 4
// If so print which second it occurred at and print the robot map
// Do this only 3 map prints at a time to not overwhelm the pc and make sure to create some
// boundary

fn part2(mov: movement) void {
    _ = mov;
    return;
}

