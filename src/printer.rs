//! Definition of the `Printer`.
//!
//! This is just an abstraction for everything that is printed to the screen
//! (or logfile, if specified). These parameters influence printing:
//! - `color`
//! - `format` (and `quiet`)
//! - `logfile`

use std::{fs::File, time::Duration};

use termcolor::{Ansi, Color, ColorChoice, ColorSpec, NoColor, StandardStream, WriteColor};

use crate::{
    Arguments, ColorSetting, Conclusion, FormatSetting, Outcome, Trial, Failed,
    Measurement, TestInfo,
};

pub(crate) struct Printer {
    out: Box<dyn WriteColor>,
    format: FormatSetting,
    name_width: usize,
    kind_width: usize,
}

impl Printer {
    /// Creates a new printer configured by the given arguments (`format`,
    /// `quiet`, `color` and `logfile` options).
    pub(crate) fn new(args: &Arguments, tests: &[Trial]) -> Self {
        let color_arg = args.color.unwrap_or(ColorSetting::Auto);

        // Determine target of all output
        let out = if let Some(logfile) = &args.logfile {
            let f = File::create(logfile).expect("failed to create logfile");
            if color_arg == ColorSetting::Always {
                Box::new(Ansi::new(f)) as Box<dyn WriteColor>
            } else {
                Box::new(NoColor::new(f))
            }
        } else {
            let choice = match color_arg {
                ColorSetting::Auto => ColorChoice::Auto,
                ColorSetting::Always => ColorChoice::Always,
                ColorSetting::Never => ColorChoice::Never,
            };
            Box::new(StandardStream::stdout(choice))
        };

        // Determine correct format
        let format = if args.quiet {
            FormatSetting::Terse
        } else {
            args.format.unwrap_or(FormatSetting::Pretty)
        };

        // Determine max test name length to do nice formatting later.
        //
        // Unicode is hard and there is no way we can properly align/pad the
        // test names and outcomes. Counting the number of code points is just
        // a cheap way that works in most cases. Usually, these names are
        // ASCII.
        let name_width = tests.iter()
            .map(|test| test.info.name.chars().count())
            .max()
            .unwrap_or(0);

        let kind_width = tests.iter()
            .map(|test| {
                if test.info.kind.is_empty() {
                    0
                } else {
                    // The two braces [] and one space
                    test.info.kind.chars().count() + 3
                }
            })
            .max()
            .unwrap_or(0);

        Self {
            out,
            format,
            name_width,
            kind_width,
        }
    }

    /// Prints the first line "running 3 tests".
    pub(crate) fn print_title(&mut self, num_tests: u64) {
        match self.format {
            FormatSetting::Pretty | FormatSetting::Terse => {
                let plural_s = if num_tests == 1 { "" } else { "s" };

                writeln!(self.out).unwrap();
                writeln!(self.out, "running {} test{}", num_tests, plural_s).unwrap();
            }
        }
    }

    /// Prints the text announcing the test (e.g. "test foo::bar ... "). Prints
    /// nothing in terse mode.
    pub(crate) fn print_test(&mut self, info: &TestInfo) {
        let TestInfo { name, kind, .. } = info;
        match self.format {
            FormatSetting::Pretty => {
                let kind = if kind.is_empty() {
                    format!("")
                } else {
                    format!("[{}] ", kind)
                };

                write!(
                    self.out,
                    "test {: <2$}{: <3$} ... ",
                    kind,
                    name,
                    self.kind_width,
                    self.name_width,
                ).unwrap();
                self.out.flush().unwrap();
            }
            FormatSetting::Terse => {
                // In terse mode, nothing is printed before the job. Only
                // `print_single_outcome` prints one character.
            }
        }
    }

    /// Prints the outcome of a single tests. `ok` or `FAILED` in pretty mode
    /// and `.` or `F` in terse mode.
    pub(crate) fn print_single_outcome(&mut self, outcome: &Outcome) {
        match self.format {
            FormatSetting::Pretty => {
                self.print_outcome_pretty(outcome);
                writeln!(self.out).unwrap();
            }
            FormatSetting::Terse => {
                let c = match outcome {
                    Outcome::Passed => '.',
                    Outcome::Failed { .. } => 'F',
                    Outcome::Ignored => 'i',
                    Outcome::Measured { .. } => {
                        // Benchmark are never printed in terse mode... for
                        // some reason.
                        self.print_outcome_pretty(outcome);
                        writeln!(self.out).unwrap();
                        return;
                    }
                };

                self.out.set_color(&color_of_outcome(outcome)).unwrap();
                write!(self.out, "{}", c).unwrap();
                self.out.reset().unwrap();
            }
        }
    }

    /// Prints the summary line after all tests have been executed.
    pub(crate) fn print_summary(&mut self, conclusion: &Conclusion, execution_time: Duration) {
        match self.format {
            FormatSetting::Pretty | FormatSetting::Terse => {
                let outcome = if conclusion.has_failed() {
                    Outcome::Failed(Failed { msg: None })
                } else {
                    Outcome::Passed
                };

                writeln!(self.out).unwrap();
                write!(self.out, "test result: ").unwrap();
                self.print_outcome_pretty(&outcome);
                writeln!(
                    self.out,
                    ". {} passed; {} failed; {} ignored; {} measured; \
                        {} filtered out; finished in {:.2}s",
                    conclusion.num_passed,
                    conclusion.num_failed,
                    conclusion.num_ignored,
                    conclusion.num_measured,
                    conclusion.num_filtered_out,
                    execution_time.as_secs_f64()
                ).unwrap();
                writeln!(self.out).unwrap();
            }
        }
    }

    /// Prints a list of all tests. Used if `--list` is set.
    pub(crate) fn print_list(&mut self, tests: &[Trial], ignored: bool) {
        Self::write_list(tests, ignored, &mut self.out).unwrap();
    }

    pub(crate) fn write_list(
        tests: &[Trial],
        ignored: bool,
        mut out: impl std::io::Write,
    ) -> std::io::Result<()> {
        for test in tests {
            // libtest prints out:
            // * all tests without `--ignored`
            // * just the ignored tests with `--ignored`
            if ignored && !test.info.is_ignored {
                continue;
            }

            let kind = if test.info.kind.is_empty() {
                format!("")
            } else {
                format!("[{}] ", test.info.kind)
            };

            writeln!(
                out,
                "{}{}: {}",
                kind,
                test.info.name,
                if test.info.is_bench { "bench" } else { "test" },
            )?;
        }

        Ok(())
    }

    /// Prints a list of failed tests with their messages. This is only called
    /// if there were any failures.
    pub(crate) fn print_failures(&mut self, fails: &[(TestInfo, Option<String>)]) {
        writeln!(self.out).unwrap();
        writeln!(self.out, "failures:").unwrap();
        writeln!(self.out).unwrap();

        // Print messages of all tests
        for (test_info, msg) in fails {
            writeln!(self.out, "---- {} ----", test_info.name).unwrap();
            if let Some(msg) = msg {
                writeln!(self.out, "{}", msg).unwrap();
            }
            writeln!(self.out).unwrap();
        }

        // Print summary list of failed tests
        writeln!(self.out).unwrap();
        writeln!(self.out, "failures:").unwrap();
        for (test_info, _) in fails {
            writeln!(self.out, "    {}", test_info.name).unwrap();
        }
    }

    /// Prints a colored 'ok'/'FAILED'/'ignored'/'bench'.
    fn print_outcome_pretty(&mut self, outcome: &Outcome) {
        let s = match outcome {
            Outcome::Passed => "ok",
            Outcome::Failed { .. } => "FAILED",
            Outcome::Ignored => "ignored",
            Outcome::Measured { .. } => "bench",
        };

        self.out.set_color(&color_of_outcome(outcome)).unwrap();
        write!(self.out, "{}", s).unwrap();
        self.out.reset().unwrap();

        if let Outcome::Measured(Measurement { avg, variance }) = outcome {
            write!(
                self.out,
                ": {:>11} ns/iter (+/- {})",
                fmt_with_thousand_sep(*avg),
                fmt_with_thousand_sep(*variance),
            ).unwrap();
        }
    }
}

/// Formats the given integer with `,` as thousand separator.
pub fn fmt_with_thousand_sep(mut v: u64) -> String {
    let mut out = String::new();
    while v >= 1000 {
        out = format!(",{:03}{}", v % 1000, out);
        v /= 1000;
    }
    out = format!("{}{}", v, out);

    out
}

/// Returns the `ColorSpec` associated with the given outcome.
fn color_of_outcome(outcome: &Outcome) -> ColorSpec {
    let mut out = ColorSpec::new();
    let color = match outcome {
        Outcome::Passed => Color::Green,
        Outcome::Failed { .. } => Color::Red,
        Outcome::Ignored => Color::Yellow,
        Outcome::Measured { .. } => Color::Cyan,
    };
    out.set_fg(Some(color));
    out
}
