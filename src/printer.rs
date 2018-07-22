use std::fs::File;

use termcolor::{Ansi, Color, ColorChoice, ColorSpec, NoColor, StandardStream, WriteColor};

use ::{Arguments, ColorSetting, FormatSetting, TestOutcome};

pub(crate) struct Printer {
    out: Box<dyn WriteColor>,
    format: FormatSetting,
}

impl Printer {
    /// Creates a new printer configured by the given arguments (`format`,
    /// `color` and `logfile` options).
    pub(crate) fn new(args: &Arguments) -> Self {
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
                ColorSetting::Auto=> ColorChoice::Auto,
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

        Self {
            out,
            format,
        }
    }

    /// Prints the first line "running 3 tests".
    pub(crate) fn print_title(&mut self, num_tests: u64) {
        match self.format {
            FormatSetting::Pretty | FormatSetting::Terse => {
                let plural_s = if num_tests == 1 {
                    ""
                } else {
                    "s"
                };

                writeln!(self.out).unwrap();
                writeln!(self.out, "running {} test{}", num_tests, plural_s).unwrap();
            }
            FormatSetting::Json => unimplemented!(),
        }
    }

    /// Prints the text announcing the test (e.g. "test foo::bar ... "). Prints
    /// nothing in terse mode.
    pub(crate) fn print_test(&mut self, name: &str, kind: &str) {
        match self.format {
            FormatSetting::Pretty => {
                let kind_str = if kind.is_empty() {
                    format!("")
                } else {
                    format!("[{}] ", kind)
                };

                write!(self.out, "test {}{} ... ", kind_str, name).unwrap();
            }
            FormatSetting::Terse => {
                // In terse mode, nothing is printed before the job. Only
                // `print_single_outcome` prints one character.
            }
            FormatSetting::Json => unimplemented!(),
        }
    }

    /// Prints the outcome of a single tests. `ok` or `FAILED` in pretty mode
    /// and `.` or `F` in terse mode.
    pub(crate) fn print_single_outcome(&mut self, outcome: TestOutcome) {
        match self.format {
            FormatSetting::Pretty => {
                self.print_outcome_pretty(outcome);
                writeln!(self.out).unwrap();
            }
            FormatSetting::Terse => {
                self.out.set_color(&color_of_outcome(outcome)).unwrap();
                let c = if outcome == TestOutcome::Failed { 'F' } else { '.' };
                write!(self.out, "{}", c).unwrap();
                self.out.reset().unwrap();
            }
            FormatSetting::Json => unimplemented!(),
        }
    }

    /// Prints the summary line after all tests have been executed.
    pub(crate) fn print_summary(
        &mut self,
        overall_outcome: TestOutcome,
        passed_count: u64,
        failed_count: u64,
    ) {
        match self.format {
            FormatSetting::Pretty | FormatSetting::Terse => {
                writeln!(self.out).unwrap();
                write!(self.out, "test result: ").unwrap();
                self.print_outcome_pretty(overall_outcome);
                writeln!(
                    self.out,
                    ". {} passed; {} failed",
                    passed_count,
                    failed_count,
                ).unwrap();
                writeln!(self.out).unwrap();
            }
            FormatSetting::Json => unimplemented!(),
        }
    }

    fn print_outcome_pretty(&mut self, outcome: TestOutcome) {
        let s = match outcome {
            TestOutcome::Passed => "ok",
            TestOutcome::Failed => "FAILED",
        };

        self.out.set_color(&color_of_outcome(outcome)).unwrap();
        write!(self.out, "{}", s).unwrap();
        self.out.reset().unwrap();
    }
}

fn color_of_outcome(outcome: TestOutcome) -> ColorSpec {
    let mut out = ColorSpec::new();
    let color = match outcome {
        TestOutcome::Passed => Color::Green,
        TestOutcome::Failed => Color::Red,
    };
    out.set_fg(Some(color));
    out
}
