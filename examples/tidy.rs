extern crate libtest_mimic;

use libtest_mimic::{Arguments, Test, Outcome, run_tests};

use std::{
    env,
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};


fn main() {
    let args = Arguments::from_args();

    let tests = collect_tests();
    run_tests(&args, tests, run_test).exit();
}

/// Creates one test for each `.rs` file in the current directory or
/// sub-directories of the current directory.
fn collect_tests() -> Vec<Test<PathBuf>> {
    fn visit_dir(path: &Path, tests: &mut Vec<Test<PathBuf>>) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            // Handle files
            let path = entry.path();
            if file_type.is_file() {
                if path.extension() == Some(OsStr::new("rs")) {
                    let name = path
                        .strip_prefix(env::current_dir()?)?
                        .display()
                        .to_string();

                    tests.push(Test {
                        name,
                        kind: "tidy".into(),
                        is_ignored: false,
                        is_bench: false,
                        data: path,
                    })
                }
            } else if file_type.is_dir() {
                // Handle directories
                visit_dir(&path, tests)?;
            }
        }

        Ok(())
    }

    // We recursively look for `.rs` files, starting from the current
    // directory.
    let mut tests = Vec::new();
    let current_dir = env::current_dir().expect("invalid working directory");
    visit_dir(&current_dir, &mut tests).expect("unexpected IO error");

    tests
}

/// Performs a couple of tidy tests.
fn run_test(test: &Test<PathBuf>) -> Outcome {
    let path = &test.data;
    let content = fs::read(path).expect("io error");

    // Check that the file is valid UTF-8
    let content = match String::from_utf8(content) {
        Err(_) => {
            return Outcome::Failed {
                msg: Some("The file's contents are not a valid UTF-8 string!".into()),
            };
        }
        Ok(s) => s,
    };

    // Check for `\r`: we only want `\n` line breaks!
    if content.contains('\r') {
        return Outcome::Failed {
            msg: Some("Contains '\\r' chars. Please use ' \\n' line breaks only!".into()),
        };
    }

    // Check for tab characters `\t`
    if content.contains('\t') {
        return Outcome::Failed {
            msg: Some("Contains tab characters ('\\t'). Indent with four spaces!".into()),
        };
    }

    // Check for too long lines
    if content.lines().any(|line| line.chars().count() > 100) {
        return Outcome::Failed {
            msg: Some("Contains lines longer than 100 codepoints!".into()),
        };
    }

    Outcome::Passed
}
