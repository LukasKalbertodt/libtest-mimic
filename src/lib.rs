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
    pub fn exit(&self) -> ! {
        self.exit_if_failed();
        process::exit(0);
    }

    pub fn exit_if_failed(&self) {
        if self.has_failed {
            process::exit(101)
        }
    }

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
/// Currently, the following CLI args are properly understood:
/// - `color`
/// - `format`
/// - `logfile`
/// - `quiet`
///
/// The others are ignored.
pub fn run_tests<D>(
    args: &Arguments,
    tests: &[Test<D>],
    run_test: impl Fn(&Test<D>) -> TestOutcome,
) -> Conclusion {
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
