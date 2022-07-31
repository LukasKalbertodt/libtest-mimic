extern crate libtest_mimic;

use libtest_mimic::{Arguments, Test, run_tests, Failed};


fn main() {
    // Parse CLI args
    let args = Arguments::from_args();

    // Generate 100 dummy tests
    let tests = (0..100)
        .map(|i| {
            Test::test(format!("test-{:03}", i), move || dummy_test(i))
                .with_ignored_flag(i % 23 == 0)
        })
        .collect::<Vec<_>>();

    // Run tests
    run_tests(&args, tests).exit();
}


// Dummy test logic that requires at least a little bit of CPU time per test.

fn dummy_test(i: u64) -> Result<(), Failed> {
    let num = collatz(771432521 + i) + i;
    let is_prime = (2..num).all(|d| num % d != 0);

    if is_prime {
        Err("our stupid condition was not true".into())
    } else {
        Ok(())
    }
}

fn collatz(mut n: u64) -> u64 {
    let mut steps = 0;
    while n != 1 {
        steps += 1;
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
    }
    steps
}
