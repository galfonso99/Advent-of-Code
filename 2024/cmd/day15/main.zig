const std = @import("std");
const print = @import("std").debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const input = try parseFile(allocator);
    defer allocator.free(input.warehouse);
    defer for (input.warehouse) |row| {
        allocator.free(row);
    };
    defer allocator.free(input.dirs);
    try part1(input);
}

const Input = struct {
    warehouse: [][]u8,
    dirs: []const u8,
};

pub fn parseFile(allocator: std.mem.Allocator) !Input {
    // const path = "../../input/test.txt";
    const path = "../../input/day15.txt";
    const content = try std.fs.cwd().readFileAlloc(allocator, path, 1024 * 1024);
    defer allocator.free(content);
    var regions = std.mem.splitSequence(u8, content, "\n\n");
    const warehouse_region = regions.next() orelse return error.InvalidFormat;
    const movement_region = regions.next() orelse return error.InvalidFormat;
    var warehouse_rows = std.mem.splitSequence(u8, warehouse_region, "\n");
    const warehouse_len = getCountOfChar(warehouse_region, '\n') + 1;
    const warehouse_width = warehouse_rows.peek().?.len;
    var warehouse = try allocator.alloc([]u8, warehouse_len);
    for (0..warehouse_len) |i| {
        warehouse[i] = try allocator.alloc(u8, warehouse_width);
    }

    var i : usize = 0;
    while (warehouse_rows.next()) |row| : (i += 1) {
        for (row, 0..) |ch, j| {
            warehouse[i][j] = ch;
        }
    }
    const dupe = try allocator.dupe(u8, movement_region);
    return .{.warehouse = warehouse, .dirs = dupe};
}


// Firstly parse the input which will be a pain
// Find the initial position of the robot
// Create a function that converts the char dir into a int dir
// Create a function for checking if the movement is available
// If it is a box check the next spot, and on and on until you see an empty spot
// or a wall, if empty spot then shift everything down one from the robot until the last
// box
// For the shifting create a function with a range

fn part1(input: Input) !void {
    const warehouse = input.warehouse;
    const dirs = input.dirs;
    var pos = [2]i32{0, 0};
    var sum: usize = 0;
    for (warehouse, 0..) |rows, i| {
        for (rows, 0..) |ch, j| {
            if (ch == '@') {
                pos = [2]i32 {@intCast(i), @intCast(j)};
            }
        }
    }
    for (dirs) |char_dir| {
        if (char_dir == '\n') { continue; }
        const dir = try charDirToInt(char_dir);
        const details = hasFreeSpace(warehouse, pos, dir);
        if (details.foundFreeSpace) {
            shiftPositions(warehouse, pos, dir, details.placesToShift);
            pos = .{pos[0] + dir[0], pos[1] + dir[1]};
        }
    }    
    for (warehouse, 0..) |rows, i| {
        for (rows, 0..) |_, j| {
            if (warehouse[i][j] == 'O') {
                sum += i * 100 + j;
            }
        }
    }
    // for (warehouse) |rows| {
    //     for (rows) |ch| {
    //         print("{c}", .{ch});
    //     }
    //     std.debug.print("\n", .{});
    // }
    print("{}\n", .{sum});
}


fn getCountOfChar(str: []const u8, byte: u8) u8 {
    var count: u8 = 0;
    for (str) |ch| {
        if (ch == byte) {
            count += 1;
        }
    }
    return count;
}

fn charDirToInt(ch: u8) ![2]i32 {
    // Remember to skip new lines before even calling this function
    if (ch == '^') {
        return .{-1, 0};
    } else if (ch == '>') {
        return .{0, 1};
    } else if (ch == 'v') {
        return .{1, 0};
    } else if (ch == '<') {
        return .{0, -1};
    }
    return error.InvalidFormat;
}

const FreeSpaceDetails = struct {
    placesToShift: i32,
    foundFreeSpace: bool,
};

fn hasFreeSpace(warehouse: [][]u8, pos: [2]i32, dir: [2]i32) FreeSpaceDetails {
    var count: i32 = 0;
    var found_space = false;
    var y = pos[0];
    var x = pos[1];
    while (warehouse[ind(y)][ind(x)] != '#') {
        if (warehouse[ind(y)][ind(x)] == '.') {
            found_space = true;
            break;
        }
        count += 1;
        y += dir[0];
        x += dir[1];
    }
    return .{.placesToShift = count, .foundFreeSpace = found_space};
}

fn shiftPositions(warehouse: [][]u8, pos: [2]i32, dir: [2]i32, amount: i32) void {
    // var y = pos[0] + dir[0] * @as(i32, @intCast(amount));
    // var x = pos[1] + dir[1] * @as(i32, @intCast(amount));
    var y = pos[0] + dir[0] * amount;
    var x = pos[1] + dir[1] * amount;
    for (0..@intCast(amount)) |_| {
        const temp = warehouse[ind(y - dir[0])][ind(x - dir[1])];
        warehouse[ind(y - dir[0])][ind(x - dir[1])] = warehouse[ind(y)][ind(x)];
        warehouse[ind(y)][ind(x)] = temp;
        y -= dir[0];
        x -= dir[1];
    }
}

fn ind(x: i32) usize {
    return @intCast(x);
}

fn part2(input: Input) void {

}
