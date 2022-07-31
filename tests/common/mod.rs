use std::{path::Path, iter::repeat_with, collections::HashSet};
use pretty_assertions::assert_eq;

use libtest_mimic::{run, Arguments, Conclusion, Test};


const TEMPDIR: &str = env!("CARGO_TARGET_TMPDIR");

pub fn do_run(mut args: Arguments, tests: Vec<Test>) -> (Conclusion, String) {
    // Create path to temporary file.
    let suffix = repeat_with(fastrand::alphanumeric).take(10).collect::<String>();
    let path = Path::new(&TEMPDIR).join(format!("libtest_mimic_output_{suffix}.txt"));

    args.logfile = Some(path.display().to_string());

    let c = run(&args, tests);
    let output = std::fs::read_to_string(&path)
        .expect("Can't read temporary logfile");
    std::fs::remove_file(&path)
        .expect("Can't remove temporary logfile");
    (c, output)
}

pub fn clean_expected_log(s: &str) -> String {
    let shared_indent = s.lines()
        .filter(|l| l.contains(|c| c != ' '))
        .map(|l| l.bytes().take_while(|b| *b == b' ').count())
        .min()
        .expect("empty expected");

    let mut out = String::new();
    for line in s.lines() {
        use std::fmt::Write;
        let cropped = if line.len() <= shared_indent {
            line
        } else {
            &line[shared_indent..]
        };
        writeln!(out, "{}", cropped).unwrap();
    }

    out
}

/// Best effort tool to check certain things about a log that might have all
/// tests randomly ordered.
pub fn assert_reordered_log(actual: &str, num: u64, expected_lines: &[&str], tail: &str) {
    let actual = actual.trim();
    let (first_line, rest) = actual.split_once('\n').expect("log has too few lines");
    let (middle, last_line) = rest.rsplit_once('\n').expect("log has too few lines");


    assert_eq!(first_line, &format!("running {} test{}", num, if num == 1 { "" } else { "s" }));
    assert_eq!(last_line, tail);

    let mut actual_lines = middle.lines().map(|l| l.trim()).collect::<HashSet<_>>();
    for expected_line in expected_lines {
        if !actual_lines.remove(expected_line.trim()) {
            panic!("expected line '{expected_line}' not in log");
        }
    }
}

/// Like `assert_eq`, but cleans the expected string (removes indendation).
macro_rules! assert_log {
    ($actual:expr, $expected:expr) => {
        let actual = $actual;
        let expected = crate::common::clean_expected_log($expected);

        pretty_assertions::assert_eq!(actual.trim(), expected.trim());
    };
}
