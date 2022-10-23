use pretty_assertions::assert_eq;
use libtest_mimic::{Trial, Conclusion, Measurement};
use crate::common::{args, check, do_run};

#[macro_use]
mod common;


fn tests() -> Vec<Trial> {
    fn meas(avg: u64, variance: u64) -> Option<Measurement> {
        Some(Measurement { avg, variance })
    }

    vec![
        Trial::test("cat", || Ok(())),
        Trial::test("dog", || Err("was not a good boy".into())),
        Trial::test("fox", || Ok(())).with_kind("apple"),
        Trial::test("bunny", || Err("jumped too high".into())).with_kind("apple"),
        Trial::test("frog", || Ok(())).with_ignored_flag(true),
        Trial::test("owl", || Err("broke neck".into())).with_ignored_flag(true),
        Trial::test("fly", || Ok(())).with_ignored_flag(true).with_kind("banana"),
        Trial::test("bear", || Err("no honey".into())).with_ignored_flag(true).with_kind("banana"),

        Trial::bench("red", |_| Ok(meas(32, 3))),
        Trial::bench("blue", |_| Err("sky fell down".into())),
        Trial::bench("yellow", |_| Ok(meas(64, 4))).with_kind("kiwi"),
        Trial::bench("green", |_| Err("was poisoned".into())).with_kind("kiwi"),
        Trial::bench("purple", |_| Ok(meas(100, 5))).with_ignored_flag(true),
        Trial::bench("cyan", |_| Err("not creative enough".into())).with_ignored_flag(true),
        Trial::bench("orange", |_| Ok(meas(17, 6))).with_ignored_flag(true).with_kind("banana"),
        Trial::bench("pink", |_| Err("bad".into())).with_ignored_flag(true).with_kind("banana"),
    ]
}

#[test]
fn normal() {
    check(args([]), tests, 16,
        Conclusion {
            num_filtered_out: 0,
            num_passed: 4,
            num_failed: 4,
            num_ignored: 8,
            num_measured: 0,
        },
        "
            test          cat    ... ok
            test          dog    ... FAILED
            test [apple]  fox    ... ok
            test [apple]  bunny  ... FAILED
            test          frog   ... ignored
            test          owl    ... ignored
            test [banana] fly    ... ignored
            test [banana] bear   ... ignored
            test          red    ... ok
            test          blue   ... FAILED
            test [kiwi]   yellow ... ok
            test [kiwi]   green  ... FAILED
            test          purple ... ignored
            test          cyan   ... ignored
            test [banana] orange ... ignored
            test [banana] pink   ... ignored

            failures:

            ---- dog ----
            was not a good boy

            ---- bunny ----
            jumped too high

            ---- blue ----
            sky fell down

            ---- green ----
            was poisoned


            failures:
                dog
                bunny
                blue
                green
        ",
    );
}

#[test]
fn test_mode() {
    check(args(["--test"]), tests, 16,
        Conclusion {
            num_filtered_out: 0,
            num_passed: 2,
            num_failed: 2,
            num_ignored: 12,
            num_measured: 0,
        },
        "
            test          cat    ... ok
            test          dog    ... FAILED
            test [apple]  fox    ... ok
            test [apple]  bunny  ... FAILED
            test          frog   ... ignored
            test          owl    ... ignored
            test [banana] fly    ... ignored
            test [banana] bear   ... ignored
            test          red    ... ignored
            test          blue   ... ignored
            test [kiwi]   yellow ... ignored
            test [kiwi]   green  ... ignored
            test          purple ... ignored
            test          cyan   ... ignored
            test [banana] orange ... ignored
            test [banana] pink   ... ignored

            failures:

            ---- dog ----
            was not a good boy

            ---- bunny ----
            jumped too high


            failures:
                dog
                bunny
        ",
    );
}

#[test]
fn bench_mode() {
    check(args(["--bench"]), tests, 16,
        Conclusion {
            num_filtered_out: 0,
            num_passed: 0,
            num_failed: 2,
            num_ignored: 12,
            num_measured: 2,
        },
        "
            test          cat    ... ignored
            test          dog    ... ignored
            test [apple]  fox    ... ignored
            test [apple]  bunny  ... ignored
            test          frog   ... ignored
            test          owl    ... ignored
            test [banana] fly    ... ignored
            test [banana] bear   ... ignored
            test          red    ... bench:          32 ns/iter (+/- 3)
            test          blue   ... FAILED
            test [kiwi]   yellow ... bench:          64 ns/iter (+/- 4)
            test [kiwi]   green  ... FAILED
            test          purple ... ignored
            test          cyan   ... ignored
            test [banana] orange ... ignored
            test [banana] pink   ... ignored

            failures:

            ---- blue ----
            sky fell down

            ---- green ----
            was poisoned


            failures:
                blue
                green
        ",
    );
}

#[test]
fn list() {
    let (c, out) = common::do_run(args(["--list"]), tests());
    assert_log!(out, "
        cat: test
        dog: test
        [apple] fox: test
        [apple] bunny: test
        frog: test
        owl: test
        [banana] fly: test
        [banana] bear: test
        red: bench
        blue: bench
        [kiwi] yellow: bench
        [kiwi] green: bench
        purple: bench
        cyan: bench
        [banana] orange: bench
        [banana] pink: bench
    ");
    assert_eq!(c, Conclusion {
        num_filtered_out: 0,
        num_passed: 0,
        num_failed: 0,
        num_ignored: 0,
        num_measured: 0,
     });
}

#[test]
fn list_ignored() {
    let (c, out) = common::do_run(args(["--list", "--ignored"]), tests());
    assert_log!(out, "
        frog: test
        owl: test
        [banana] fly: test
        [banana] bear: test
        purple: bench
        cyan: bench
        [banana] orange: bench
        [banana] pink: bench
    ");
    assert_eq!(c, Conclusion {
        num_filtered_out: 0,
        num_passed: 0,
        num_failed: 0,
        num_ignored: 0,
        num_measured: 0,
     });
}

#[test]
fn list_with_filter() {
    let (c, out) = common::do_run(args(["--list", "a"]), tests());
    assert_log!(out, "
        cat: test
        [banana] bear: test
        cyan: bench
        [banana] orange: bench
    ");
    assert_eq!(c, Conclusion {
        num_filtered_out: 0,
        num_passed: 0,
        num_failed: 0,
        num_ignored: 0,
        num_measured: 0,
     });
}

#[test]
fn filter_c() {
    check(args(["c"]), tests, 2,
        Conclusion {
            num_filtered_out: 14,
            num_passed: 1,
            num_failed: 0,
            num_ignored: 1,
            num_measured: 0,
        },
        "
            test cat  ... ok
            test cyan ... ignored
        ",
    );
}

#[test]
fn filter_o_test() {
    check(args(["--test", "o"]), tests, 6,
        Conclusion {
            num_filtered_out: 10,
            num_passed: 1,
            num_failed: 1,
            num_ignored: 4,
            num_measured: 0,
        },
        "
            test          dog    ... FAILED
            test [apple]  fox    ... ok
            test          frog   ... ignored
            test          owl    ... ignored
            test [kiwi]   yellow ... ignored
            test [banana] orange ... ignored

            failures:

            ---- dog ----
            was not a good boy


            failures:
                dog
        ",
    );
}

#[test]
fn filter_o_test_include_ignored() {
    check(args(["--test", "--include-ignored", "o"]), tests, 6,
        Conclusion {
            num_filtered_out: 10,
            num_passed: 2,
            num_failed: 2,
            num_ignored: 2,
            num_measured: 0,
        },
        "
            test          dog    ... FAILED
            test [apple]  fox    ... ok
            test          frog   ... ok
            test          owl    ... FAILED
            test [kiwi]   yellow ... ignored
            test [banana] orange ... ignored

            failures:

            ---- dog ----
            was not a good boy

            ---- owl ----
            broke neck


            failures:
                dog
                owl
        ",
    );
}

#[test]
fn filter_o_test_ignored() {
    check(args(["--test", "--ignored", "o"]), tests, 3,
        Conclusion {
            num_filtered_out: 13,
            num_passed: 1,
            num_failed: 1,
            num_ignored: 1,
            num_measured: 0,
        },
        "
            test          frog   ... ok
            test          owl    ... FAILED
            test [banana] orange ... ignored

            failures:

            ---- owl ----
            broke neck


            failures:
                owl
        ",
    );
}

#[test]
fn normal_include_ignored() {
    check(args(["--include-ignored"]), tests, 16,
        Conclusion {
            num_filtered_out: 0,
            num_passed: 8,
            num_failed: 8,
            num_ignored: 0,
            num_measured: 0,
        },
        "
            test          cat    ... ok
            test          dog    ... FAILED
            test [apple]  fox    ... ok
            test [apple]  bunny  ... FAILED
            test          frog   ... ok
            test          owl    ... FAILED
            test [banana] fly    ... ok
            test [banana] bear   ... FAILED
            test          red    ... ok
            test          blue   ... FAILED
            test [kiwi]   yellow ... ok
            test [kiwi]   green  ... FAILED
            test          purple ... ok
            test          cyan   ... FAILED
            test [banana] orange ... ok
            test [banana] pink   ... FAILED

            failures:

            ---- dog ----
            was not a good boy

            ---- bunny ----
            jumped too high

            ---- owl ----
            broke neck

            ---- bear ----
            no honey

            ---- blue ----
            sky fell down

            ---- green ----
            was poisoned

            ---- cyan ----
            not creative enough

            ---- pink ----
            bad


            failures:
                dog
                bunny
                owl
                bear
                blue
                green
                cyan
                pink
        ",
    );
}

#[test]
fn normal_ignored() {
    check(args(["--ignored"]), tests, 8,
        Conclusion {
            num_filtered_out: 8,
            num_passed: 4,
            num_failed: 4,
            num_ignored: 0,
            num_measured: 0,
        },
        "
            test          frog   ... ok
            test          owl    ... FAILED
            test [banana] fly    ... ok
            test [banana] bear   ... FAILED
            test          purple ... ok
            test          cyan   ... FAILED
            test [banana] orange ... ok
            test [banana] pink   ... FAILED

            failures:

            ---- owl ----
            broke neck

            ---- bear ----
            no honey

            ---- cyan ----
            not creative enough

            ---- pink ----
            bad


            failures:
                owl
                bear
                cyan
                pink
        ",
    );
}

#[test]
fn lots_of_flags() {
    check(args(["--include-ignored", "--skip", "g", "--test", "o"]), tests, 3,
        Conclusion {
            num_filtered_out: 13,
            num_passed: 1,
            num_failed: 1,
            num_ignored: 1,
            num_measured: 0,
        },
        "
            test [apple] fox    ... ok
            test         owl    ... FAILED
            test [kiwi]  yellow ... ignored

            failures:

            ---- owl ----
            broke neck


            failures:
                owl
        ",
    );
}

#[test]
fn terse_output() {
    let (c, out) = do_run(args(["--format", "terse", "--test-threads", "1"]), tests());
    assert_eq!(c, Conclusion {
        num_filtered_out: 0,
        num_passed: 4,
        num_failed: 4,
        num_ignored: 8,
        num_measured: 0,
    });
    assert_log!(out, "
        running 16 tests
        .F.Fiiii.F.Fiiii
        failures:

        ---- dog ----
        was not a good boy

        ---- bunny ----
        jumped too high

        ---- blue ----
        sky fell down

        ---- green ----
        was poisoned


        failures:
            dog
            bunny
            blue
            green

        test result: FAILED. 4 passed; 4 failed; 8 ignored; 0 measured; 0 filtered out; \
            finished in 0.00s
    ");
}
