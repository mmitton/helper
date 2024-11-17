use clap::{arg, Arg, Command};

pub(crate) fn get() -> (
    bool,
    bool,
    Option<usize>,
    bool,
    Option<usize>,
    Option<usize>,
) {
    let matches = Command::new("runner")
        .about("AoC Runner")
        .arg(
            Arg::new("sample-data")
                .long("sample-data")
                .visible_alias("sample")
                .num_args(0)
                .required(false)
                .help("Run Sample Data"),
        )
        .arg(
            Arg::new("real-data")
                .long("real-data")
                .visible_alias("real")
                .num_args(0)
                .required(false)
                .help("Run Real Data"),
        )
        .arg(
            Arg::new("times")
                .long("times")
                .num_args(1)
                .default_missing_value("5")
                .required(false)
                .help("Generate Times Table"),
        )
        .arg(
            Arg::new("md")
                .long("md")
                .num_args(0)
                .required(false)
                .help("Format Times Table as Markdown"),
        )
        .arg(
            Arg::new("no-capture")
                .long("no-capture")
                .num_args(0)
                .required(false)
                .help("Do not capture output"),
        )
        .subcommand(
            Command::new("today").about("Run latest day available.  Will be today during AoC"),
        )
        .subcommand(Command::new("all").about("Run all days"))
        .subcommand(
            Command::new("day")
                .about("Run a given day")
                .arg_required_else_help(true)
                .arg(arg!(<YEAR> "Year").value_parser(clap::value_parser!(usize)))
                .arg(arg!(<DAY> "Day").value_parser(clap::value_parser!(usize))),
        )
        .subcommand(
            Command::new("year")
                .about("Run all days in a given year")
                .arg_required_else_help(true)
                .arg(arg!(<YEAR> "Year").value_parser(clap::value_parser!(usize))),
        )
        .get_matches();

    let sample_data = matches
        .get_one::<bool>("sample-data")
        .copied()
        .unwrap_or_default();
    let real_data = matches
        .get_one::<bool>("real-data")
        .copied()
        .unwrap_or_default();
    let times = if let Some(times) = matches.get_one::<String>("times") {
        match times.parse() {
            Ok(times) => Some(times),
            Err(_) => panic!("Number of times is not a number: {times:?}"),
        }
    } else {
        None
    };

    let md = matches.get_one::<bool>("md").copied().unwrap_or_default();
    let no_capture = matches
        .get_one::<bool>("no-capture")
        .copied()
        .unwrap_or_default();

    let sample_data = match (sample_data, real_data) {
        (true, true) => panic!("Cannot use both sample-data and real-data"),
        (true, false) => true,
        (false, true) => false,
        (false, false) => cfg!(debug_assertions),
    };

    let (year, day) = match matches.subcommand() {
        None | Some(("today", _)) => {
            use chrono::prelude::*;
            let today = Local::now();
            match today.month() {
                11 | 12 => match today.day() {
                    1..=20 => (Some(today.year() as usize), Some(today.day() as usize)),
                    _ => (Some(today.year() as usize), Some(20)),
                },
                _ => (Some(today.year() as usize - 1), Some(20)),
            }
        }
        Some(("all", _)) => (None, None),
        Some(("day", submatches)) => {
            let year = submatches.get_one::<usize>("YEAR").copied();
            let day = submatches.get_one::<usize>("DAY").copied();
            (year, day)
        }
        Some(("year", submatches)) => {
            let year = submatches.get_one::<usize>("YEAR").copied();
            (year, None)
        }
        subcommand => unreachable!("{subcommand:?}"),
    };

    (sample_data, no_capture, times, md, year, day)
}
