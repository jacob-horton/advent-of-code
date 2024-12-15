const std = @import("std");

const ArrayList = std.ArrayList;

fn part_1(contents: []const u8) !i32 {
    var count: i32 = 0;
    var lines = std.mem.split(u8, contents, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) continue;
        var it = std.mem.split(u8, line, " ");

        var safe = true;
        var prev: ?i32 = null;
        var negative: ?bool = null;
        while (it.next()) |level_str| {
            const level = try std.fmt.parseInt(i32, level_str, 10);

            if (prev) |p| {
                const diff = level - p;

                // Unsafe
                if (@abs(diff) > 3 or diff == 0) {
                    safe = false;
                    break;
                }

                if (negative) |n| {
                    // Unsafe
                    if ((n and diff > 0) or (!n and diff < 0)) {
                        safe = false;
                        break;
                    }
                } else {
                    negative = diff < 0;
                }
            }

            prev = level;
        }

        if (safe) {
            count += 1;
        }
    }

    return count;
}

fn is_diff_safe(prev: i32, current: i32, negative: *?bool) bool {
    const diff = current - prev;

    // Unsafe
    if (@abs(diff) > 3 or diff == 0) {
        return false;
    }

    if (negative.*) |n| {
        // Unsafe
        if ((n and diff > 0) or (!n and diff < 0)) {
            return false;
        }
    } else {
        negative.* = diff < 0;
    }

    return true;
}

fn is_report_safe(levels: ArrayList(i32), allow_removal: bool) !bool {
    var negative: ?bool = null;
    for (levels.items[1..], 1..) |level, i| {
        if (!is_diff_safe(levels.items[i - 1], level, &negative)) {
            if (allow_removal) {
                // Check removing i-2, i-1, and ith element to see if that produces a safe report
                for ((@max(i, 2) - 2)..(i + 1)) |j| {
                    if (j >= levels.items.len) {
                        continue;
                    }

                    var with_val_removed = try levels.clone();
                    defer with_val_removed.deinit();
                    _ = with_val_removed.orderedRemove(j);

                    if (try is_report_safe(with_val_removed, false)) {
                        return true;
                    }
                }
            }

            return false;
        }
    }

    return true;
}

fn part_2(contents: []const u8) !i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    var levels = ArrayList(i32).init(allocator);
    defer levels.deinit();

    var count: i32 = 0;
    var lines = std.mem.split(u8, contents, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) continue;
        var it = std.mem.split(u8, line, " ");

        while (it.next()) |level_str| {
            const level = try std.fmt.parseInt(i32, level_str, 10);
            try levels.append(level);
        }

        // std.debug.print("\nLEVELS: {s}\n", .{line});
        if (try is_report_safe(levels, true)) {
            count += 1;
        }

        levels.clearRetainingCapacity();
    }

    return count;
}

pub fn main() !void {
    const result = try part_2(@embedFile("./input.txt"));
    std.debug.print("{}\n", .{result});
}

test "part 1" {
    const result = try part_1(@embedFile("./test_input.txt"));
    try std.testing.expectEqual(2, result);
}

test "part 2" {
    const result = try part_2(@embedFile("./test_input.txt"));
    try std.testing.expectEqual(4, result);
}
