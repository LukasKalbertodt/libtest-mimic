#[macro_use]
extern crate structopt;

use std::str::FromStr;


#[derive(StructOpt, Debug)]
#[structopt(
    template = "USAGE: [FLAGS] [OPTIONS] [FILTER]\n\n{all-args}\n\n\n{after-help}",
    raw(setting = "structopt::clap::AppSettings::DisableVersion"),
    after_help = "By default, all tests are run in parallel. This can be altered with the \n\
        --test-threads flag or the RUST_TEST_THREADS environment variable when running \n\
        tests (set it to 1).\n\
        \n\
        All tests have their standard output and standard error captured by default. \n\
        This can be overridden with the --nocapture flag or setting RUST_TEST_NOCAPTURE \n\
        environment variable to a value other than \"0\". Logging is not captured by default.",
)]
pub struct Settings {
    // ============== FLAGS ===================================================
    #[structopt(long = "--ignored", help = "Run ignored tests")]
    ignored: bool,

    #[structopt(long = "--test", help = "Run tests and not benchmarks")]
    test: bool,

    #[structopt(long = "--bench", help = "Run benchmarks instead of tests")]
    bench: bool,

    #[structopt(long = "--list", help = "List all tests and benchmarks")]
    list: bool,

    #[structopt(
        long = "--nocapture",
        help = "don't capture stdout/stderr of each task, allow printing directly",
    )]
    nocapture: bool,

    #[structopt(
        long = "--exact",
        help = "Exactly match filters rather than by substring",
    )]
    exact: bool,

    #[structopt(
        short = "q",
        long = "--quiet",
        help = "Display one character per test instead of one line. Alias to --format=terse",
    )]
    quiet: bool,

    // ============== OPTIONS =================================================
    #[structopt(
        long = "--test-threads",
        help = "Number of threads used for running tests in parallel"
    )]
    num_threads: Option<u32>,

    #[structopt(
        long = "--logfile",
        value_name = "PATH",
        help = "Write logs to the specified file instead of stdout",
    )]
    logfile: Option<String>,

    #[structopt(
        long = "--skip",
        value_name = "FILTER",
        raw(number_of_values = "1"),
        help = "Skip tests whose names contain FILTER (this flag can be used multiple times)",
    )]
    skip: Vec<String>,

    #[structopt(
        long = "--color",
        raw(possible_values = r#"&["auto", "always", "never"]"#),
        value_name = "auto|always|never",
        help = "Configure coloring of output: \n\
            - auto = colorize if stdout is a tty and tests are run on serially (default)\n\
            - always = always colorize output\n\
            - never = never colorize output\n",
    )]
    color: Option<ColorSetting>,

    #[structopt(
        long = "--format",
        raw(possible_values = r#"&["pretty", "terse", "json"]"#),
        value_name = "pretty|terse|json",
        help = "Configure formatting of output: \n\
            - pretty = Print verbose output\n\
            - terse = Display one character per test\n\
            - json = Output a json document\n",
    )]
    format: Option<FormatSetting>,

    #[structopt(
        name = "FILTER",
        help = "The FILTER string is tested against the name of all tests, and only those tests \
            whose names contain the filter are run.",
    )]
    filter_string: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSetting {
    Auto,
    Always,
    Never,
}

impl FromStr for ColorSetting {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ColorSetting::Auto),
            "always" => Ok(ColorSetting::Always),
            "never" => Ok(ColorSetting::Never),
            _ => Err("foo"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatSetting {
    Pretty,
    Terse,
    Json,
}

impl FromStr for FormatSetting {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pretty" => Ok(FormatSetting::Pretty),
            "terse" => Ok(FormatSetting::Terse),
            "json" => Ok(FormatSetting::Json),
            _ => Err("foo"),
        }
    }
}
