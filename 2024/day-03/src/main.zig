const std = @import("std");

const ArrayList = std.ArrayList;

fn read_digit(memory: []const u8, out: *i32) !usize {
    for (memory, 0..) |c, i| {
        if (c < '0' or c > '9') {
            out.* = try std.fmt.parseInt(i32, memory[0..i], 10);
            return i;
        }
    }

    out.* = try std.fmt.parseInt(i32, memory[0..], 10);
    return memory.len;
}

fn get_next_mul_params(memory: []const u8, a: *i32, b: *i32, end: *usize) !bool {
    for (0..memory.len) |i| {
        if (i + 4 >= memory.len) {
            return false;
        }

        if (!std.mem.eql(u8, memory[i .. i + 4], "mul(")) {
            continue;
        }

        var j = i + 4;
        j += try read_digit(memory[j..], a);
        if (memory[j] != ',') {
            continue;
        }

        j += 1 + try read_digit(memory[j + 1 ..], b);
        if (memory[j] != ')') {
            continue;
        }

        end.* += j + 1;
        return true;
    }

    return false;
}

fn part_1(contents: []const u8) !i32 {
    var sum: i32 = 0;
    var lines = std.mem.split(u8, contents, "\n");
    while (lines.next()) |line| {
        var a: i32 = undefined;
        var b: i32 = undefined;
        var end: usize = 0;
        while (try get_next_mul_params(line[end..], &a, &b, &end)) {
            sum += a * b;
        }
    }

    return sum;
}

fn update_enabled(memory: []const u8, enabled: *bool) void {
    for (0..memory.len) |i| {
        if (i + 4 >= memory.len) {
            return;
        }

        if (std.mem.eql(u8, memory[i .. i + 4], "do()")) {
            enabled.* = true;
            continue;
        }

        if (i + 7 >= memory.len) {
            return;
        }

        if (std.mem.eql(u8, memory[i .. i + 7], "don't()")) {
            enabled.* = false;
            continue;
        }
    }
}

fn part_2(contents: []const u8) !i32 {
    var sum: i32 = 0;
    var lines = std.mem.split(u8, contents, "\n");
    var enabled = true;
    while (lines.next()) |line| {
        var a: i32 = undefined;
        var b: i32 = undefined;
        var end: usize = 0;

        while (true) {
            const prev_end = end;
            const has_mul = try get_next_mul_params(line[end..], &a, &b, &end);
            if (!has_mul) {
                break;
            }

            update_enabled(line[prev_end..end], &enabled);
            if (enabled) {
                sum += a * b;
            }
        }
    }

    return sum;
}

pub fn main() !void {
    const result = try part_2(@embedFile("./input.txt"));
    std.debug.print("{}\n", .{result});
}

test "part 1" {
    const result = try part_1(@embedFile("./test_input.txt"));
    try std.testing.expectEqual(161, result);
}

test "part 2" {
    const result = try part_2(@embedFile("./test_input_2.txt"));
    try std.testing.expectEqual(48, result);
}
