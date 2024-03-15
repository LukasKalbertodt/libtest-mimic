use common::{args, check};
use libtest_mimic::{Trial, Conclusion};
use pretty_assertions::assert_eq;

use crate::common::{assert_reordered_log, conclusion_to_output, do_run};

#[macro_use]
mod common;


fn tests() -> Vec<Trial> {
    vec![
        Trial::test("foo", || Ok(())),
        Trial::test("bar", || Ok(())),
        Trial::test("barro", || Ok(())),
        Trial::skippable_test("baz", || Ok(Some("Can't find a quux".into()))),
    ]
}

#[test]
fn normal() {
    check(args([]), tests, 4,
        Conclusion {
            num_filtered_out: 0,
            num_passed: 3,
            num_failed: 0,
            num_ignored: 1,
            num_measured: 0,
        },
        "
            test foo   ... ok
            test bar   ... ok
            test barro ... ok
            test baz   ... skipped
        "
    );
}

#[test]
fn filter_one() {
    check(args(["foo"]), tests, 1,
        Conclusion {
            num_filtered_out: 3,
            num_passed: 1,
            num_failed: 0,
            num_ignored: 0,
            num_measured: 0,
        },
        "test foo ... ok",
    );
}

#[test]
fn filter_two() {
    check(args(["bar"]), tests, 2,
        Conclusion {
            num_filtered_out: 2,
            num_passed: 2,
            num_failed: 0,
            num_ignored: 0,
            num_measured: 0,
        },
        "
            test bar   ... ok
            test barro ... ok
        ",
    );
}


#[test]
fn filter_exact() {
    check(args(["bar", "--exact"]), tests, 1,
        Conclusion {
            num_filtered_out: 3,
            num_passed: 1,
            num_failed: 0,
            num_ignored: 0,
            num_measured: 0,
        },
        "test bar ... ok",
    );
}

#[test]
fn filter_two_and_skip() {
    check(args(["--skip", "barro", "bar"]), tests, 1,
        Conclusion {
            num_filtered_out: 3,
            num_passed: 1,
            num_failed: 0,
            num_ignored: 0,
            num_measured: 0,
        },
        "test bar ... ok",
    );
}

#[test]
fn filter_runtime_ignored() {
    check(args(["baz", "--exact"]), tests, 1,
        Conclusion {
            num_filtered_out: 3,
            num_passed: 0,
            num_failed: 0,
            num_ignored: 1,
            num_measured: 0,
        },
        "test baz ... skipped",
    );
}

#[test]
fn skip_nothing() {
    check(args(["--skip", "peter"]), tests, 4,
        Conclusion {
            num_filtered_out: 0,
            num_passed: 3,
            num_failed: 0,
            num_ignored: 1,
            num_measured: 0,
        },
        "
            test foo   ... ok
            test bar   ... ok
            test barro ... ok
            test baz   ... skipped
        "
    );
}

#[test]
fn skip_two() {
    check(args(["--skip", "bar"]), tests, 2,
        Conclusion {
            num_filtered_out: 2,
            num_passed: 1,
            num_failed: 0,
            num_ignored: 1,
            num_measured: 0,
        },
        "
            test foo ... ok
            test baz ... skipped
        "
    );
}

#[test]
fn skip_exact() {
    check(args(["--exact", "--skip", "bar"]), tests, 3,
        Conclusion {
            num_filtered_out: 1,
            num_passed: 2,
            num_failed: 0,
            num_ignored: 1,
            num_measured: 0,
        },
        "
            test foo   ... ok
            test barro ... ok
            test baz   ... skipped
        "
    );
}

#[test]
fn terse_output() {
    let (c, out) = do_run(args(["--format", "terse"]), tests());
    assert_eq!(c, Conclusion {
        num_filtered_out: 0,
        num_passed: 3,
        num_failed: 0,
        num_ignored: 1,
        num_measured: 0,
    });
    assert_reordered_log(out.as_str(), 4, &["...S"], &conclusion_to_output(&c), true);
}
