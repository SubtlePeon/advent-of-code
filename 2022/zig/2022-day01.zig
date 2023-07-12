const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const f = try std.fs.cwd().openFile(
        "../inputs/day01.txt",
        std.fs.File.OpenFlags{ .mode = .read_only, .lock = .None },
    );
    const reader = f.reader();

    var buf: [128]u8 = undefined;

    var most = [3]i32{ 0, 0, 0 };
    var current: i32 = 0;

    while (reader.readUntilDelimiter(&buf, '\n')) |input| : (buf = undefined) {
        if (input.len == 0) {
            for (most) |_, i| {
                if (current > most[i]) {
                    var tmp = current;
                    current = most[i];
                    most[i] = tmp;
                }
            }
            current = 0;
            continue;
        }

        current += try std.fmt.parseInt(i32, input, 10);
    } else |_| {}

    var total: i32 = 0;
    for (most) |item| {
        total += item;
    }

    try stdout.print("{} + {} + {} = {}\n", .{ most[0], most[1], most[2], total });
}
