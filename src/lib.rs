#[macro_use]
extern crate structopt;
extern crate termcolor;

use std::{
    fmt,
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
    /// used to store more data per test.
    pub data: D,
}

/// The outcome of performing a test.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TestOutcome {
    Passed,
    Failed,
}

impl fmt::Display for TestOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestOutcome::Passed => "ok",
            TestOutcome::Failed => "FAILED",
        }.fmt(f)
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
/// Currently, the following CLI args are properly understood:
/// - `color`
/// - `format`
/// - `logfile`
///
/// The others are ignored.
pub fn run_tests<D>(
    args: &Arguments,
    tests: &[Test<D>],
    run_test: impl Fn(&Test<D>) -> TestOutcome,
) -> ! {
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

    // Exit application
    if overall_outcome == TestOutcome::Passed {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
