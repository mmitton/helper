use std::path::Path;

#[derive(Debug, Default)]
pub(crate) enum Run {
    #[default]
    Today,
    All,
    Year {
        year: usize,
    },
    Day {
        year: usize,
        day: usize,
    },
}

impl Run {
    pub(crate) fn matches(&self, year: usize, day: usize, most_recent_day: (usize, usize)) -> bool {
        match self {
            Self::Today => year == most_recent_day.0 && day == most_recent_day.1,
            Self::All => true,
            Self::Year { year: y } => *y == year,
            Self::Day { year: y, day: d } => *y == year && *d == day,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Args {
    pub(crate) sample: bool,
    pub(crate) run: Run,
    pub(crate) times: bool,
    pub(crate) md: bool,
    pub(crate) no_capture: bool,
}

impl Args {
    fn help(exec: String, err: Option<&str>) -> ! {
        let exec = Path::new(&exec);
        std::eprintln!(
            "Usage: {exec} [OPTIONS] [COMMAND]",
            exec = exec.file_name().unwrap().to_str().unwrap()
        );
        std::eprintln!();
        std::eprintln!("Commands:");
        std::eprintln!("  today                Run latest day available");
        std::eprintln!("  all                  Run all days");
        std::eprintln!("  day {{year}} {{day}}     Run a given day");
        std::eprintln!("  year {{year}}          Run all days in a given year");
        std::eprintln!();
        std::eprintln!("Options:");
        std::eprintln!("      --sample");
        std::eprintln!("      --sample-data    Run Sample Data");
        std::eprintln!("      --release");
        std::eprintln!("      --real");
        std::eprintln!("      --real-data      Run Real Data");
        std::eprintln!("      --times          Generate Times Table");
        std::eprintln!("      --md             Format Times Table as Markdown");
        std::eprintln!("      --nocapture      Do not capture output");
        std::eprintln!("  -h, --help           Print help");

        if let Some(err) = err {
            std::eprintln!();
            std::eprintln!("Error: {err}");
        }

        std::process::exit(1);
    }

    pub(crate) fn new() -> Self {
        let mut arg = Self::default();
        arg.parse();
        arg
    }

    fn parse(&mut self) {
        let mut remaining = Vec::new();
        let mut args = std::env::args().peekable();

        self.sample = cfg!(debug_assertions);

        let exec = if let Some(exec) = args.next() {
            exec
        } else {
            "executable".into()
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--sample-data" | "--sample" => self.sample = true,
                "--real-data" | "--real" | "--release" => self.sample = false,
                "--times" => self.times = true,
                "--md" => self.md = true,
                "--nocapture" => self.no_capture = true,
                "--help" | "-h" => Self::help(exec, None),
                _ if arg.starts_with("--") => {
                    Self::help(exec, Some(format!("Unknown option {arg:?}").as_str()))
                }
                _ => remaining.push(arg),
            }
        }

        if !remaining.is_empty() {
            match remaining[0].as_str() {
                "today" => {
                    if remaining.len() != 1 {
                        Self::help(exec, Some("today does not take any additional arguments"));
                    }
                    self.run = Run::Today;
                }
                "all" => {
                    if remaining.len() != 1 {
                        Self::help(exec, Some("all does not take any additional arguments"));
                    }
                    self.run = Run::All;
                }
                "year" => {
                    if remaining.len() != 2 {
                        Self::help(exec, Some("year takes 1 additional argument"));
                    }
                    if let Ok(year) = remaining[1].parse() {
                        self.run = Run::Year { year };
                    } else {
                        Self::help(exec, Some("Invalid year"));
                    }
                }
                "day" => {
                    if remaining.len() != 3 {
                        Self::help(exec, Some("day takes 2 additional arguments"));
                    }
                    match (remaining[1].parse(), remaining[2].parse()) {
                        (Ok(year), Ok(day)) => self.run = Run::Day { year, day },
                        (Err(_), Err(_)) => Self::help(exec, Some("Invalid year/day")),
                        (Err(_), _) => Self::help(exec, Some("Invalid year")),
                        (_, Err(_)) => Self::help(exec, Some("Invalid day")),
                    }
                }
                _ => Self::help(
                    exec,
                    Some(format!("Invalid command: {:?}", remaining[0]).as_str()),
                ),
            }
        }
    }
}
