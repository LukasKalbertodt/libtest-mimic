extern crate test_cli;
extern crate structopt;

use test_cli::{Arguments, Test, Outcome, run_tests};


fn main() {
    // Parse CLI args
    let args = Arguments::from_args();

    // Generate 100 tests with dummy names
    let tests = (0..100)
        .map(|i| {
            Test {
                name: format!("test-{:03}", i),
                kind: String::new(),
                data: i,
            }
        })
        .collect::<Vec<_>>();

    // Run tests
    run_tests(&args, tests, |test| {
        // We want this one test to fail
        if test.data == 53 {
            Outcome::Failed
        } else {
            Outcome::Passed
        }
    }).exit();
}
