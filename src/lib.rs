#[macro_use]
extern crate structopt;
extern crate termcolor;

use std::{
    process,
};

mod args;
mod printer;

pub use args::{Arguments, ColorSetting, FormatSetting};


/// Description of a single test.
pub struct Test<D = ()> {
    /// The name of the test. It's displayed in the output and used for all
    /// kinds of filtering.
    pub name: String,

    /// Optional string to describe the kind of test. If this string is not
    /// empty, it is printed in brackets before the test name (e.g.
    /// `test [my-kind] test_name`).
    pub kind: String,

    /// Custom data. This field is not used by this library and can instead be
    /// used to store arbitrary data per test.
    pub data: D,
}

impl<D: Default> Test<D> {
    /// Creates a test description with the given name, an empty `kind` and
    /// default data.
    pub fn from_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: String::new(),
            data: D::default(),
        }
    }
}

/// The outcome of performing a test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestOutcome {
    Passed,
    Failed,
}

#[must_use]
pub struct Conclusion {
    has_failed: bool,
}

impl Conclusion {
    /// Exits the application with an appropriate error code (0 if all tests
    /// have passed, 101 if there have been failures).
    pub fn exit(&self) -> ! {
        self.exit_if_failed();
        process::exit(0);
    }

    /// Exits the application with error code 101 if there were any failures.
    pub fn exit_if_failed(&self) {
        if self.has_failed {
            process::exit(101)
        }
    }

    /// Returns whether or not there have been any failures.
    pub fn has_failed(&self) -> bool {
        self.has_failed
    }
}

/// Runs all given tests with the given test runner.
///
/// This is the central function of this crate. It provides the framework for
/// the testing harness. It does all the printing and house keeping.
///
/// This function exits the application with an appropriate error code and
/// never returns!
///
/// This function tries to respect most options configured via CLI args. For
/// example, output format and coloring are respected. However, some things
/// cannot be handled by this function and you as a user need to take care to
/// respect the CLI parameters. The following options are ignored by this
/// function and need to be manually checked:
///
/// - `--nocapture` and capturing in general. It is expected that during the
///   test, nothing writes to `stdout` and `stderr`, unless `--nocapture` was
///   specified. If the test is ran as a seperate process, this is fairly easy.
///   If however, the test is part of the current application and it uses
///   `println!()` and friends, it might be impossible to capture the output.
///
/// Currently, the following CLI args are properly understood:
/// - `color`
/// - `format`
/// - `logfile`
/// - `quiet`
///
/// The others are ignored for now.
pub fn run_tests<D>(
    args: &Arguments,
    tests: &[Test<D>],
    run_test: impl Fn(&Test<D>) -> TestOutcome,
) -> Conclusion {
    // TODO:
    // - ignored tests
    // - test filtering (normal filter-in and filter-out) (with `--exact` flag)
    // - print failures
    // - JSON
    // - `--ignored` flag
    // - decide how to deal with `--test` and `--bench` flags
    // - `--list` flag
    // - multiple threads
    // - Better formatting by determining max test name len

    let mut printer = printer::Printer::new(args);

    // Print number of tests
    printer.print_title(tests.len() as u64);

    // Execute all tests
    let mut failed_count = 0;
    for test in tests {
        printer.print_test(&test.name, &test.kind);

        // Run the given function to run the test.
        let outcome = run_test(&test);

        // Handle outcome
        printer.print_single_outcome(outcome);
        match outcome {
            TestOutcome::Passed => {}
            TestOutcome::Failed => {
                failed_count += 1;
            }
        }
    }

    // Handle overall results
    let overall_outcome = if failed_count > 0 {
        TestOutcome::Failed
    } else {
        TestOutcome::Passed
    };

    printer.print_summary(overall_outcome, tests.len() as u64 - failed_count, failed_count);

    Conclusion {
        has_failed: overall_outcome == TestOutcome::Failed,
    }
}
