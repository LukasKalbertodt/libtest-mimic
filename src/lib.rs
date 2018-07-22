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

    /// Whether or not this test should be ignored. If the `--ignored` flag is
    /// set, ignored tests are executed, too.
    pub is_ignored: bool,

    /// Whether this test is actually a benchmark.
    pub is_bench: bool,

    /// Custom data. This field is not used by this library and can instead be
    /// used to store arbitrary data per test.
    pub data: D,
}

impl<D: Default> Test<D> {
    /// Creates a test with the given name, an empty `kind` and default data.
    /// The test is not ignored and is not a benchmark.
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: String::new(),
            is_ignored: false,
            is_bench: false,
            data: D::default(),
        }
    }

    /// Creates a benchmark with the given name, an empty `kind` and default
    /// data. The benchmark is not ignored.
    pub fn bench(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: String::new(),
            is_ignored: false,
            is_bench: true,
            data: D::default(),
        }
    }
}

/// The outcome of performing a test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Passed,
    Failed {
        /// A message that is shown after all tests have been run.
        msg: Option<String>,
    },
    Ignored,
    Measured {
        avg: u64,
        variance: u64,
    }
}

#[must_use]
pub struct Conclusion {
    has_failed: bool,
    num_filtered_out: u64,
    num_passed: u64,
    num_failed: u64,
    num_ignored: u64,
    num_benches: u64,
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

    /// Returns how many tests were filtered out (either by the filter-in
    /// pattern or by `--skip` arguments).
    pub fn num_filtered_out(&self) -> u64 {
        self.num_filtered_out
    }

    /// Returns how many tests passed.
    pub fn num_passed(&self) -> u64 {
        self.num_passed
    }

    /// Returns how many tests failed.
    pub fn num_failed(&self) -> u64 {
        self.num_failed
    }

    /// Returns how many tests were ignored.
    pub fn num_ignored(&self) -> u64 {
        self.num_ignored
    }

    /// Returns how many benchmark were successfully run.
    pub fn num_benches(&self) -> u64 {
        self.num_benches
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
/// Currently, the following CLI args are ignored, but are planned to be used
/// in the future:
/// - `--test-threads`
///
/// All other flags and options are used properly.
pub fn run_tests<D>(
    args: &Arguments,
    tests: Vec<Test<D>>,
    run_test: impl Fn(&Test<D>) -> Outcome,
) -> Conclusion {
    // TODO:
    // - JSON
    // - multiple threads

    // Apply filtering
    let (tests, num_filtered_out) = if args.filter_string.is_some() || !args.skip.is_empty() {
        let len_before = tests.len() as u64;
        let mut tests = tests;
        tests.retain(|t| {
            // If a filter was specified, apply this
            if let Some(filter) = &args.filter_string {
                match args.exact {
                    true if &t.name != filter => return false,
                    false if !t.name.contains(filter) => return false,
                    _ => {}
                };
            }

            // If any skip pattern were specified, test for all patterns.
            for skip_filter in &args.skip {
                match args.exact {
                    true if &t.name == skip_filter => return false,
                    false if t.name.contains(skip_filter) => return false,
                    _ => {}
                }
            }

            true
        });

        let num_filtered_out = len_before - tests.len() as u64;
        (tests, num_filtered_out)
    } else {
        (tests, 0)
    };

    // Create printer which is used for all output.
    let mut printer = printer::Printer::new(args, &tests);

    if args.list {
        printer.print_list(&tests);
        return Conclusion {
            has_failed: false,
            num_filtered_out: 0,
            num_passed: 0,
            num_failed: 0,
            num_ignored: 0,
            num_benches: 0,
        };
    }

    // Print number of tests
    printer.print_title(tests.len() as u64);

    // Execute all tests
    let mut failed_tests = Vec::new();
    let mut num_ignored = 0;
    let mut num_benches = 0;
    for test in &tests {
        if test.is_bench {
            num_benches += 1;
        }

        printer.print_test(&test.name, &test.kind);

        let is_ignored = (test.is_ignored && !args.ignored)
            || (test.is_bench && args.test)
            || (!test.is_bench && args.bench);

        let outcome = if is_ignored {
            Outcome::Ignored
        } else {
            // Run the given function
            run_test(&test)
        };

        // Handle outcome
        printer.print_single_outcome(&outcome);
        match outcome {
            Outcome::Failed { msg } => failed_tests.push((test, msg)),
            Outcome::Ignored => num_ignored += 1,
            _ => {}
        }
    }

    // Print failures if there were any
    if !failed_tests.is_empty() {
        printer.print_failures(&failed_tests);
    }

    // Handle overall results
    let num_failed = failed_tests.len() as u64;
    let conclusion = Conclusion {
        has_failed: num_failed != 0,
        num_filtered_out,
        num_passed: tests.len() as u64 - num_failed - num_ignored - num_benches,
        num_failed,
        num_ignored,
        num_benches,
    };

    printer.print_summary(&conclusion);

    conclusion
}
