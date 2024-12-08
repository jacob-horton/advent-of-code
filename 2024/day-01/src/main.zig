const std = @import("std");

const ArrayList = std.ArrayList;

fn part_1(contents: []const u8) !i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    var left_list = ArrayList(i32).init(allocator);
    defer left_list.deinit();
    var right_list = ArrayList(i32).init(allocator);
    defer right_list.deinit();

    var lines = std.mem.split(u8, contents, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) continue;
        var it = std.mem.split(u8, line, "   ");

        const left = try std.fmt.parseInt(i32, it.next().?, 10);
        const right = try std.fmt.parseInt(i32, it.next().?, 10);
        try left_list.append(left);
        try right_list.append(right);
    }

    const left = try left_list.toOwnedSlice();
    const right = try right_list.toOwnedSlice();
    defer allocator.free(left);
    defer allocator.free(right);

    std.mem.sort(i32, left, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, right, {}, comptime std.sort.asc(i32));

    var sum: i32 = 0;
    for (left, right) |l, r| {
        sum += @intCast(@abs(r - l));
    }

    return sum;
}

fn part_2(contents: []const u8) !i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    var left_list = ArrayList(i32).init(allocator);
    defer left_list.deinit();
    var right_list = ArrayList(i32).init(allocator);
    defer right_list.deinit();

    var lines = std.mem.split(u8, contents, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) continue;
        var it = std.mem.split(u8, line, "   ");

        const left = try std.fmt.parseInt(i32, it.next().?, 10);
        const right = try std.fmt.parseInt(i32, it.next().?, 10);
        try left_list.append(left);
        try right_list.append(right);
    }

    var sum: i32 = 0;
    for (left_list.items) |left| {
        var count: i32 = 0;
        for (right_list.items) |right| {
            if (right == left) {
                count += 1;
            }
        }

        sum += count * left;
    }

    return sum;
}

pub fn main() !void {
    const result = try part_2(@embedFile("./input.txt"));
    std.debug.print("{}\n", .{result});
}

test "part 1" {
    const result = try part_1(@embedFile("./test_input.txt"));
    try std.testing.expectEqual(result, 11);
}

test "part 2" {
    const result = try part_2(@embedFile("./test_input.txt"));
    try std.testing.expectEqual(result, 31);
}
