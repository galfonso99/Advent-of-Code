const std = @import("std");
const print = @import("std").debug.print;

const length: i32 = 103;
const width: i32 = 101;
const len: usize = 500;

const movement = struct {
    positions: [len][2]i32,
    velocities: [len][2]i32
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const struct_of_arrays = try parseFile(allocator);
    part2(struct_of_arrays);
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
        var parts = std.mem.splitSequence(u8, line, " ");
        const pos_part = parts.next() orelse return error.InvalidFormat;
        const vel_part = parts.next() orelse return error.InvalidFormat;
        var pos = std.mem.splitSequence(u8, pos_part[2..], ","); // Skip "p="
        const pos_x = try std.fmt.parseInt(i32, pos.next() orelse return error.InvalidFormat, 10);
        const pos_y = try std.fmt.parseInt(i32, pos.next() orelse return error.InvalidFormat, 10);
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

// What I did was start from the safety factor from part 1 print every arrangement that
// had a lower safety factor while making that the new lowest safety factor
// Then ideally you would end up with the minimum safety factor (aka entropy) when the pattern
// was formed

fn part2(mov: movement) void {
    var pos = mov.positions;
    const vel = mov.velocities;
    var quadrants = [_]i32{0} ** 5;
    const middle_x = @divTrunc(width, 2);
    const middle_y = @divTrunc(length, 2);
    var map: [width][length]u8 = undefined;
    var lowest: i32 = 218433348;
    for (0..map.len) |i| {
        for (0..map[0].len) |j| {
            map[i][j] = ' ';
        }
    }
    for (1..10000) |j| {
        quadrants = [_]i32{0} ** 5;
        for (0..len) |i| {
            const pos_x = @mod(pos[i][0] + vel[i][0] + width, width);
            const pos_y = @mod(pos[i][1] + vel[i][1] + length, length);
            pos[i][0] = pos_x;
            pos[i][1] = pos_y;
            const quadrant_index = whichQuadrant(pos_x, pos_y, middle_x, middle_y);
            quadrants[quadrant_index - 1] += 1;
        }
        if (quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3] < lowest) {
            print("Found a possible one at index {}\n", .{j});
            lowest = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
            for (0..len) |x| {
                map[@intCast(pos[x][0])][@intCast(pos[x][1])] = '.';
            }
            print("\n", .{});
            for (0..map.len) |x| {
                for (0..map[0].len) |y| {
                    print("{c}", .{map[x][y]});
                }
                print("\n", .{});
            }
            for (0..map.len) |x| {
                for (0..map[0].len) |y| {
                    map[x][y] = ' ';
                }
            }
        }
    }
}
