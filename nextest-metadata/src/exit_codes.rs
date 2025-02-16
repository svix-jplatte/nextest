// Copyright (c) The nextest Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Documented exit codes for `cargo nextest` failures.
///
/// `cargo nextest` runs may fail for a variety of reasons. This structure documents the exit codes
/// that may occur in case of expected failures.
///
/// Unknown/unexpected failures will always result in exit code 1.
pub enum NextestExitCode {}

impl NextestExitCode {
    /// Running `cargo metadata` produced an error.
    pub const CARGO_METADATA_FAILED: i32 = 102;

    /// Building tests produced an error.
    pub const BUILD_FAILED: i32 = 101;

    /// An error was encountered while attempting to double-spawn a nextest process.
    pub const DOUBLE_SPAWN_ERROR: i32 = 70;

    /// No tests were selected to run, but no other errors occurred.
    ///
    /// This is an advisory exit code generated if nextest is run with `--no-tests=fail` (soon to
    /// become the default). See [discussion #1646] for more.
    ///
    /// [discussion #1646]: https://github.com/nextest-rs/nextest/discussions/1646
    pub const NO_TESTS_RUN: i32 = 4;

    /// One or more tests failed.
    pub const TEST_RUN_FAILED: i32 = 100;

    /// Creating an archive produced an error.
    pub const ARCHIVE_CREATION_FAILED: i32 = 103;

    /// Creating a test list produced an error.
    pub const TEST_LIST_CREATION_FAILED: i32 = 104;

    /// A setup script failed.
    pub const SETUP_SCRIPT_FAILED: i32 = 105;

    /// Writing data to stdout or stderr produced an error.
    pub const WRITE_OUTPUT_ERROR: i32 = 110;

    /// Downloading an update resulted in an error.
    pub const UPDATE_ERROR: i32 = 90;

    /// An update was available and `--check` was requested.
    pub const UPDATE_AVAILABLE: i32 = 80;

    /// A downgrade was requested but not performed.
    pub const UPDATE_DOWNGRADE_NOT_PERFORMED: i32 = 81;

    /// An update was available but the user canceled it.
    pub const UPDATE_CANCELED: i32 = 82;

    /// A user issue happened while setting up a nextest invocation.
    pub const SETUP_ERROR: i32 = 96;

    /// An experimental feature was used without the environment variable to enable it.
    pub const EXPERIMENTAL_FEATURE_NOT_ENABLED: i32 = 95;

    /// A filtering expression failed to parse.
    pub const INVALID_FILTER_EXPRESSION: i32 = 94;

    /// A self-update was requested but this version of cargo-nextest cannot perform self-updates.
    pub const SELF_UPDATE_UNAVAILABLE: i32 = 93;

    /// The current version of nextest did not meet repository or tool requirements.
    ///
    /// *Since nextest 0.9.55*.
    pub const REQUIRED_VERSION_NOT_MET: i32 = 92;

    /// The current version of nextest is older than the minimum recommended version.
    ///
    /// This advisory exit code is only produced by `cargo nextest show-config version`.
    ///
    /// *Since nextest 0.9.55*.
    pub const RECOMMENDED_VERSION_NOT_MET: i32 = 10;
}
