use pretty_assertions::assert_eq;
use libtest_mimic::{Test, Arguments, Conclusion};
use crate::common::assert_reordered_log;

#[macro_use]
mod common;


fn tests() -> Vec<Test> {
    vec![
        Test::test("foo", || Ok(())),
        Test::test("bar", || Ok(())),
        Test::test("baz", || Ok(())),
    ]
}

#[test]
fn normal() {
    let (c, out) = common::do_run(Arguments::default(), tests());
    assert_eq!(c, Conclusion {
        num_filtered_out: 0,
        num_passed: 3,
        num_failed: 0,
        num_ignored: 0,
        num_benches: 0,
    });
    assert_reordered_log(
        &out,
        3,
        &[
            "test foo ... ok",
            "test bar ... ok",
            "test baz ... ok",
        ],
        "test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; \
            0 filtered out; finished in 0.00s",
    );
}

#[test]
fn single_threaded_exact() {
    let args = Arguments {
        num_threads: Some(1),
        ..Default::default()
    };
    let (c, out) = common::do_run(args, tests());
    assert_eq!(c, Conclusion {
        num_filtered_out: 0,
        num_passed: 3,
        num_failed: 0,
        num_ignored: 0,
        num_benches: 0,
    });
    assert_log!(&out, "
        running 3 tests
        test foo ... ok
        test bar ... ok
        test baz ... ok

        test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; \
            0 filtered out; finished in 0.00s
    ");
}

#[test]
fn filter_one() {
    let args = Arguments {
        filter: Some("foo".into()),
        ..Default::default()
    };
    let (c, out) = common::do_run(args, tests());
    assert_eq!(c, Conclusion {
        num_filtered_out: 2,
        num_passed: 1,
        num_failed: 0,
        num_ignored: 0,
        num_benches: 0,
    });
    assert_reordered_log(
        &out,
        1,
        &["test foo ... ok"],
        "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; \
            2 filtered out; finished in 0.00s",
    );
}

#[test]
fn filter_two() {
    let args = Arguments {
        filter: Some("ba".into()),
        ..Default::default()
    };
    let (c, out) = common::do_run(args, tests());
    assert_eq!(c, Conclusion {
        num_filtered_out: 1,
        num_passed: 2,
        num_failed: 0,
        num_ignored: 0,
        num_benches: 0,
    });
    assert_reordered_log(
        &out,
        2,
        &[
            "test bar ... ok",
            "test baz ... ok",
        ],
        "test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; \
            1 filtered out; finished in 0.00s",
    );
}
