const std = @import("std");
const print = @import("std").debug.print;
const Queue = @import("queue.zig").Queue;

// Could not figure out how to get the right result for part 2
// Would pass every test that I came across on the internet
// But could not find the edge-case in my code Oh well

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
    try part2(input);
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

// On top of part 1 I need 3 additional things
// modify the board to be double
// Find a way to check if movement is possible with the double boxes
// If it is possible then find a way to move all the affected boxes

fn part2(input: Input) !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    const original_warehouse = input.warehouse;
    const dirs = input.dirs;
    var pos = [2]i32{0, 0};
    var sum: usize = 0;

    var warehouse = try allocator.alloc([]u8, original_warehouse.len);
    for (0..original_warehouse.len) |i| {
        warehouse[i] = try allocator.alloc(u8, original_warehouse[0].len * 2);
    }
    defer allocator.free(warehouse);
    defer for (warehouse) |row| {
        allocator.free(row);
    };

    for (0..original_warehouse.len) |i| {
        for (0..original_warehouse[0].len) |j| {
            const ch = original_warehouse[i][j];
            if (ch == '#') {
                warehouse[i][j*2] = '#';
                warehouse[i][j*2+1] = '#';
            } else if (ch == 'O') {
                warehouse[i][j*2] = '[';
                warehouse[i][j*2+1] = ']';
            } else if (ch == '.') {
                warehouse[i][j*2] = '.';
                warehouse[i][j*2+1] = '.';
            } else {
                warehouse[i][j*2] = '@';
                warehouse[i][j*2+1] = '.';
            }
        }
    }
    for (warehouse, 0..) |rows, i| {
        for (rows, 0..) |ch, j| {
            if (ch == '@') {
                pos = [2]i32 {@intCast(i), @intCast(j)};
            }
        }
    }
    for (dirs) |char_dir| {
        if (char_dir == '\n') { continue; }
        // print("Processing dir {c} \n", .{char_dir});
        const dir = try charDirToInt(char_dir);
        if (char_dir == '<' or char_dir == '>') {
            const details = hasFreeSpace(warehouse, pos, dir);
            // print("Can it move? {}\n", .{details.foundFreeSpace});
            if (details.foundFreeSpace) {
                shiftPositions(warehouse, pos, dir, details.placesToShift);
                pos = .{pos[0] + dir[0], pos[1] + dir[1]};
            }
        } else {
            const result = try isMovePossible(warehouse, pos, dir, allocator);
            // print("Can it move? {}\n", .{result.isMovePossible});
            if (result.isMovePossible) {
                shiftBoxes(warehouse, pos, dir, &result.boxesPerLevel, &result.startingPointPerLevel);
                pos = .{pos[0] + dir[0], pos[1] + dir[1]};
            }
            result.boxesPerLevel.deinit();
            result.startingPointPerLevel.deinit();
        }
    }    
    for (warehouse, 0..) |rows, i| {
        for (rows, 0..) |_, j| {
            if (warehouse[i][j] == '[') {
                sum += i * 100 + j;
            } 
        }
    }
    std.debug.print("\n", .{});
    std.debug.print("\n", .{});
    for (warehouse) |rows| {
        for (rows) |ch| {
            print("{c}", .{ch});
        }
        std.debug.print("\n", .{});
    }
    print("{}\n", .{sum});
}

const CheckResult = struct {
    isMovePossible: bool,
    boxesPerLevel: std.ArrayList(i32),
    startingPointPerLevel: std.ArrayList([2]i32)
};

fn isMovePossible(warehouse: [][]u8, pos: [2]i32, dir: [2]i32, allocator: std.mem.Allocator) !CheckResult {
    const y = pos[0] + dir[0];
    const x = pos[1] + dir[1];
    var queue = try Queue([2]i32).init(allocator);
    defer queue.deinit();
    var boxesPerLevel = std.ArrayList(i32).init(allocator);
    var startingPointPerLevel = std.ArrayList([2]i32).init(allocator);
    if (warehouse[ind(y)][ind(x)] == '#') {
        return .{.isMovePossible = false, .boxesPerLevel = boxesPerLevel, .startingPointPerLevel = startingPointPerLevel};
    } else if (warehouse[ind(y)][ind(x)] == '.') {
        return .{.isMovePossible = true, .boxesPerLevel = boxesPerLevel, .startingPointPerLevel = startingPointPerLevel};
    } 
    var starting_pos = [_]i32{y, x};
    if (warehouse[ind(y)][ind(x)] == ']') { starting_pos = [2]i32{y, x - 1}; }
    const second_pos = [2]i32{y, starting_pos[1] + 1};
    try queue.enqueue(starting_pos);
    try queue.enqueue(second_pos);
    while (!queue.isEmpty()) {
        const length = queue.count();
        try boxesPerLevel.append(@intCast(length));
        for (0..length) |i| {
            if (! try checkOneOver(warehouse, &queue, &startingPointPerLevel, dir, i, length)) {
                return .{.isMovePossible = false, .boxesPerLevel = boxesPerLevel, .startingPointPerLevel = startingPointPerLevel};
            }
        }
    }
    return .{.isMovePossible = true, .boxesPerLevel = boxesPerLevel, .startingPointPerLevel = startingPointPerLevel};
}

fn checkOneOver(warehouse: [][]u8, queue: *Queue([2]i32), startingPointPerLevel: *std.ArrayList([2]i32), 
                                                 dir: [2]i32, i: usize, length: usize) !bool {
    const curr_pos = queue.dequeue() orelse [2]i32{0,0};
    const new_pos = [2]i32{curr_pos[0] + dir[0], curr_pos[1] + dir[1]};
    if (i == 0) {
        try startingPointPerLevel.append(curr_pos);
    }
    if (warehouse[ind(new_pos[0])][ind(new_pos[1])] == '#') {
        return false;
    } else if (warehouse[ind(new_pos[0])][ind(new_pos[1])] == '.') {
        return true;
    } else if (warehouse[ind(new_pos[0])][ind(new_pos[1])] == ']') {
        const left_side = [2]i32{new_pos[0], new_pos[1] - 1};
        // or (i > 0 and i < length - 1 and warehouse[ind(curr_pos[0])][ind(curr_pos[1] - 1)] == '.')
        if (i == 0 or (i > 0 and i < length - 1 and warehouse[ind(curr_pos[0])][ind(curr_pos[1] - 1)] == '.')) {
            try queue.enqueue(left_side);
        }
        try queue.enqueue(new_pos);
        return true;
    } else {
        const right_side = [2]i32{new_pos[0], new_pos[1] + 1};
        try queue.enqueue(new_pos);
        // or (i > 0 and i < length - 1 and warehouse[ind(curr_pos[0])][ind(curr_pos[1] + 1)] == '.')
        if (i == length - 1 or (i > 0 and i < length - 1 and warehouse[ind(curr_pos[0])][ind(curr_pos[1] + 1)] == '.')) {
            try queue.enqueue(right_side);
        }
        return true;
    }
}

// This function will only run on vertical directional movements
fn shiftBoxes(warehouse: [][]u8, pos: [2]i32, dir: [2]i32, boxesPerLevel: *const std.ArrayList(i32), 
                                         startingPointPerLevel: *const std.ArrayList([2]i32)) void {

    var height: usize = boxesPerLevel.items.len;
    var y = pos[0] + dir[0] * @as(i32, @intCast(height));
    for (0..boxesPerLevel.items.len) |_| {
        height -= 1;
        const start_x: usize = @intCast(startingPointPerLevel.items[height][1]);
        const boxesCount: usize = @intCast(boxesPerLevel.items[height]);
        std.debug.assert(boxesPerLevel.items.len == startingPointPerLevel.items.len);
        if (boxesCount > 0 and (std.mem.eql(i32, &dir, &.{-1, 0}) or std.mem.eql(i32, &dir, &.{-1, 0}))) {
            // print("Boxes count {}\n", .{boxesCount});
            // print("Starting pos {} {}\n", .{startingPointPerLevel.items[height][0], start_x});
        }
        var i: usize = 0;
        var x: usize = start_x;
        while (i < boxesCount) {
            if (warehouse[ind(y)][x] == '[' or warehouse[ind(y)][x] == ']') {
                i += 1;
            }
            const temp = warehouse[ind(y)][x];
            warehouse[ind(y)][x] = warehouse[ind(y + dir[0])][x];
            warehouse[ind(y + dir[0])][x] = temp;
            x += 1;
        }
        y -= dir[0];
    }
    // Do one last swap to swap the robot position with the next position
    const temp = warehouse[ind(y)][ind(pos[1])];
    warehouse[ind(y)][ind(pos[1])] = warehouse[ind(y + dir[0])][ind(pos[1])];
    warehouse[ind(y + dir[0])][ind(pos[1])] = temp;
}


// if (boxesCount > 0 and (std.mem.eql(i32, &dir, &.{-1, 0}) or std.mem.eql(i32, &dir, &.{-1, 0}))) {
//     print("Before swap \n", .{});
//     for (0..warehouse.len) |i| {
//         // for (warehouse[ind(y + dir[0] + @as(i32, @intCast(i)) - 1)]) |col| {
//         for (warehouse[i]) |col| {
//             print("{c}", .{col});
//         }
//         print("\n", .{});
//     }
//     print("\n", .{});
// }


// if (boxesCount > 0 and (std.mem.eql(i32, &dir, &.{-1, 0}) or std.mem.eql(i32, &dir, &.{-1, 0}))) {
//     print("After swap \n", .{});
//     for (0..warehouse.len) |j| {
//         for (warehouse[j]) |col| {
//             print("{c}", .{col});
//         }
//         print("\n", .{});
//     }
//     print("\n", .{});
// }

// if (boxesPerLevel.items.len > 0) {
//     print("After robot swap \n", .{});
//         for (0..warehouse.len) |i| {
//             for (warehouse[i]) |col| {
//             print("{c}", .{col});
//         }
//         print("\n", .{});
//     }
//     print("\n", .{});
// }

