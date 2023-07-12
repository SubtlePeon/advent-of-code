const std = @import("std");
const print = std.debug.print;

const Play = enum {
    rock,
    paper,
    scissors,
    pub fn from_char(char: u8) ?Play {
        return switch (char) {
            'A', 'X' => Play.rock,
            'B', 'Y' => Play.paper,
            'C', 'Z' => Play.scissors,
            else => null,
        };
    }
    pub fn value(self: Play) u32 {
        return switch (self) {
            .rock => 1,
            .paper => 2,
            .scissors => 3,
        };
    }
    pub fn points_won(self: Play, opp: Play) u32 {
        // wish this could be more organized, but it is what it is
        return switch (self) {
            .rock => switch (opp) {
                .rock => 3,
                .paper => 0,
                .scissors => 6,
            },
            .paper => switch (opp) {
                .rock => 6,
                .paper => 3,
                .scissors => 0,
            },
            .scissors => switch (opp) {
                .rock => 0,
                .paper => 6,
                .scissors => 3,
            },
        };
    }
};

const Result = enum {
    win,
    lose,
    draw,
    pub fn from_char(char: u8) ?Result {
        return switch (char) {
            'X' => .lose,
            'Y' => .draw,
            'Z' => .win,
            else => null,
        };
    }

    pub fn against(self: Result, opp: Play) Play {
        return switch (self) {
            .win => switch (opp) {
                .rock => .paper,
                .paper => .scissors,
                .scissors => .rock,
            },
            .lose => switch (opp) {
                .rock => .scissors,
                .paper => .rock,
                .scissors => .paper,
            },
            .draw => switch (opp) {
                .rock => .rock,
                .paper => .paper,
                .scissors => .scissors,
            },
        };
    }
};

const file_loc = "../inputs/day02.txt";

pub fn main() !void {
    const file = try std.fs.cwd().openFile(
        file_loc,
        std.fs.File.OpenFlags{},
    );
    const f = file.reader();
    var buf: [4]u8 = undefined;
    var total: u32 = 0;
    while (f.readUntilDelimiter(&buf, '\n')) |line| {
        var opp_play = Play.from_char(line[0]) orelse unreachable;
        var result = Result.from_char(line[2]) orelse unreachable;
        var you_play = result.against(opp_play);

        // Add points for choice
        total += you_play.value();
        // Add points for win/loss
        total += you_play.points_won(opp_play);
    } else |_| {}
    print("Final score: {}\n", .{total});
}

pub fn part1() !void {
    const file = try std.fs.cwd().openFile(
        file_loc,
        std.fs.File.OpenFlags{},
    );
    const f = file.reader();
    var buf: [4]u8 = undefined;
    var total: u32 = 0;
    while (f.readUntilDelimiter(&buf, '\n')) |line| {
        var opp_play = Play.from_char(line[0]).?;
        var you_play = Play.from_char(line[2]).?;

        // Add points for choice
        total += you_play.value();
        // Add points for win/loss
        total += you_play.points_won(opp_play);
    } else |_| {}
    print("Final score: {}\n", .{total});
}
