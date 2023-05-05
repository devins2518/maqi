const std = @import("std");
const pkgs = @import("deps.zig").pkgs;

pub fn build(b: *std.build.Builder) void {
    const target = b.standardTargetOptions(.{});
    const mode = b.standardReleaseOptions();

    const lib = b.addStaticLibrary("libmaqi", "src/lib.zig");
    pkgs.addAllTo(lib);
    lib.setTarget(target);
    lib.setBuildMode(mode);

    const exe = b.addExecutable("maqi", "src/main.zig");
    pkgs.addAllTo(exe);
    exe.setTarget(target);
    exe.setBuildMode(mode);
    exe.install();

    const run_cmd = exe.run();
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const lib_tests = b.addTest("src/lib.zig");
    pkgs.addAllTo(lib_tests);
    lib_tests.setTarget(target);
    lib_tests.setBuildMode(mode);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&lib_tests.step);
}
