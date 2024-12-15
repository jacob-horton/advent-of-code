const std = @import("std");

const ArrayList = std.ArrayList;

fn check_horizontal(grid: [][]const u8, i: usize, j: usize) bool {
    if (j + 3 >= grid[i].len) {
        return false;
    }

    const in_order = [4]u8{ grid[i][j], grid[i][j + 1], grid[i][j + 2], grid[i][j + 3] };
    return std.mem.eql(u8, &in_order, "XMAS") or std.mem.eql(u8, &in_order, "SAMX");
}

fn check_vertical(grid: [][]const u8, i: usize, j: usize) bool {
    if (i + 3 >= grid.len) {
        return false;
    }

    const in_order = [4]u8{ grid[i][j], grid[i + 1][j], grid[i + 2][j], grid[i + 3][j] };
    return std.mem.eql(u8, &in_order, "XMAS") or std.mem.eql(u8, &in_order, "SAMX");
}

fn check_diagonal_down_right(grid: [][]const u8, i: usize, j: usize) bool {
    if (i + 3 >= grid.len or j + 3 >= grid[i].len) {
        return false;
    }

    const in_order = [4]u8{ grid[i][j], grid[i + 1][j + 1], grid[i + 2][j + 2], grid[i + 3][j + 3] };
    return std.mem.eql(u8, &in_order, "XMAS") or std.mem.eql(u8, &in_order, "SAMX");
}

fn check_diagonal_up_right(grid: [][]const u8, i: usize, j: usize) bool {
    if (i < 3 or j + 3 >= grid[i].len) {
        return false;
    }

    const in_order = [4]u8{ grid[i][j], grid[i - 1][j + 1], grid[i - 2][j + 2], grid[i - 3][j + 3] };
    return std.mem.eql(u8, &in_order, "XMAS") or std.mem.eql(u8, &in_order, "SAMX");
}

fn count_matches(grid: [][]const u8, i: usize, j: usize) i32 {
    var count: i32 = 0;

    if (check_horizontal(grid, i, j)) count += 1;
    if (check_vertical(grid, i, j)) count += 1;
    if (check_diagonal_down_right(grid, i, j)) count += 1;
    if (check_diagonal_up_right(grid, i, j)) count += 1;

    return count;
}

// Caller owns the returned memory
fn generate_grid(contents: []const u8, allocator: std.mem.Allocator) ![][]const u8 {
    var grid = ArrayList([]const u8).init(allocator);

    var lines = std.mem.split(u8, contents, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) continue;
        try grid.append(line);
    }

    return grid.toOwnedSlice();
}

fn part_1(contents: []const u8) !i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const grid: [][]const u8 = try generate_grid(contents, allocator);
    defer allocator.free(grid);

    var count: i32 = 0;
    for (grid, 0..) |row, i| {
        for (row, 0..) |_, j| {
            count += count_matches(grid, i, j);
        }
    }

    return count;
}

fn is_m_s_pairing(a: u8, b: u8) bool {
    if (a == 'M' and b == 'S') return true;
    if (a == 'S' and b == 'M') return true;

    return false;
}

fn is_xmas(grid: [][]const u8, i: usize, j: usize) bool {
    if (i < 1 or i >= grid.len - 1) return false;
    if (j < 1 or j >= grid[i].len - 1) return false;

    if (grid[i][j] != 'A') return false;

    if (!is_m_s_pairing(grid[i - 1][j - 1], grid[i + 1][j + 1])) return false;
    if (!is_m_s_pairing(grid[i + 1][j - 1], grid[i - 1][j + 1])) return false;

    return true;
}

fn part_2(contents: []const u8) !i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const grid: [][]const u8 = try generate_grid(contents, allocator);
    defer allocator.free(grid);

    var count: i32 = 0;
    for (grid, 0..) |row, i| {
        for (row, 0..) |_, j| {
            if (is_xmas(grid, i, j)) {
                count += 1;
            }
        }
    }

    return count;
}

pub fn main() !void {
    const result = try part_2(@embedFile("./input.txt"));
    std.debug.print("{}\n", .{result});
}

test "part 1" {
    const result = try part_1(@embedFile("./test_input.txt"));
    try std.testing.expectEqual(18, result);
}

test "part 2" {
    const result = try part_2(@embedFile("./test_input.txt"));
    try std.testing.expectEqual(9, result);
}
