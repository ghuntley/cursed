// Generated build integration for CURSED package manager
// This file is auto-generated, do not edit manually

const std = @import("std");

pub fn addDependencies(b: *std.build.Builder) void {
    // Package dependencies
    // testz
    b.addPackagePath("testz", ".cursed/packages/testz/mod.csd");
}

pub const dependencies = struct {
    pub const testz = ".cursed/packages/testz/mod.csd";
};
