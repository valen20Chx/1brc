const std = @import("std");

const Measurement = @import("./measurement.zig").Measurement;

const LineError = error{Incomplete};

pub fn read_file() !Measurement {
    var max = Measurement{
        .temp = 0,
        .city = "init",
    };

    const file_path = "../measurements_e3.txt";

    const file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();

    const buffer_size = 1024;
    var buffer: [buffer_size]u8 = undefined;

    var offset: usize = 0;

    while (true) {
        try file.seekTo(offset);

        const bytes_read = try file.read(&buffer);

        if (bytes_read == 0) break;

        std.debug.print("Read {d} bytes:\n{s}\n\n", .{ bytes_read, buffer[0..bytes_read] });

        offset += bytes_read;

        // TODO : Go to the start of a new line if we are in the middle of one
        const linesIt = std.mem.splitSequence(u8, &buffer, "\n");

        while (linesIt.next()) |line| {
            const fieldsIt = std.mem.splitSequence(u8, line, ";");

            const city_name = fieldsIt.next() orelse return LineError.Incomplete;
            const city_temp = fieldsIt.next() orelse return LineError.Incomplete;

            const measurement = Measurement{
                .temp = city_temp,
                .city = city_name,
            };

            if (measurement.temp > max.temp) {
                max = measurement;
            }

            // TODO : If we are at the end of the buffer and it isn't a new line, query the next line.
        }
    }

    std.debug.print("Hottest city: {} ({})\n", .{ max.city, max.temp });
}
