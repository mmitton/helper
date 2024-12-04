use super::{Error, InputFileCache};
use std::collections::BTreeMap;

mod args;
mod run;
mod times;

use times::TimesCacheEntry;

pub struct Config<RegisterFunc, MostRecentDayFunc>
where
    RegisterFunc: Fn(&mut BTreeMap<(usize, usize), (u8, super::NewRunner)>),
    MostRecentDayFunc: FnOnce(usize, usize, usize) -> (usize, usize),
{
    download_input: bool,
    allow_copy: bool,
    readme_header: &'static str,
    register_func: RegisterFunc,
    most_recent_day_func: MostRecentDayFunc,
}

impl<RegisterFunc, MostRecentDayFunc> Config<RegisterFunc, MostRecentDayFunc>
where
    RegisterFunc: Fn(&mut BTreeMap<(usize, usize), (u8, super::NewRunner)>),
    MostRecentDayFunc: FnOnce(usize, usize, usize) -> (usize, usize),
{
    pub fn new(register_func: RegisterFunc, most_recent_day_func: MostRecentDayFunc) -> Self {
        Self {
            download_input: true,
            allow_copy: true,
            readme_header: "",
            register_func,
            most_recent_day_func,
        }
    }

    pub fn download_input(&mut self, download_input: bool) {
        self.download_input = download_input;
    }

    pub fn readme_header(&mut self, readme_header: &'static str) {
        self.readme_header = readme_header;
    }

    pub fn allow_copy(&mut self, allow_copy: bool) {
        self.allow_copy = allow_copy;
    }
}

pub fn main<RegisterFunc, MostRecentDayFunc, const N: usize>(
    config: Config<RegisterFunc, MostRecentDayFunc>,
) -> Result<(), Error>
where
    RegisterFunc: Fn(&mut BTreeMap<(usize, usize), (u8, super::NewRunner)>),
    MostRecentDayFunc: FnOnce(usize, usize, usize) -> (usize, usize),
{
    let args = args::Args::new();

    let mut runners = BTreeMap::new();
    (config.register_func)(&mut runners);

    if args.times.is_some() {
        super::output(|output| output.no_output());
    } else if cfg!(debug_assertions) || args.no_capture {
        super::output(|output| output.stdout());
    } else {
        super::output(|output| output.capture());
    }

    let mut times_cache: BTreeMap<usize, Vec<TimesCacheEntry>> = BTreeMap::new();
    let run_count = args.times.unwrap_or(1);

    use chrono::Datelike;
    let today = chrono::Local::now();
    let most_recent_day = (config.most_recent_day_func)(
        today.year() as usize,
        today.month() as usize,
        today.day() as usize,
    );
    if config.download_input {
        match &args.run {
            args::Run::Day { year, day } => {
                if let Err(e) = super::download_input(*year, *day) {
                    println!("Cannot download input for {year}-{day:02}.  {e:?}");
                }
            }
            _ => {
                let year = most_recent_day.0;
                let day = most_recent_day.1;
                if let Err(e) = super::download_input(year, day) {
                    println!("Cannot download input for {year}-{day:02}.  {e:?}");
                }
            }
        }
    }

    let input_file_cache: InputFileCache<N> = super::InputFileCache::new(config.allow_copy)?;
    for ((year, day), (parts, new_runner)) in runners.iter() {
        if !args.run.matches(*year, *day, most_recent_day) {
            continue;
        }

        if !(*year < most_recent_day.0 || (*year == most_recent_day.0 && *day <= most_recent_day.1))
        {
            continue;
        }

        let mut times_cache_entry = TimesCacheEntry {
            day: *day,
            results: BTreeMap::new(),
        };
        for part in 1..=*parts {
            let result = run::run(
                args.sample,
                new_runner,
                args.times.is_none(),
                run_count,
                *year,
                *day,
                part,
                &input_file_cache,
            );
            times_cache_entry.results.insert(part, result);
        }
        times_cache
            .entry(*year)
            .or_default()
            .push(times_cache_entry);
    }

    if args.times.is_some() && !times_cache.is_empty() {
        let parts = *runners.values().map(|(parts, _)| parts).max().unwrap();
        times::print_times(
            args.md,
            config.readme_header,
            run_count,
            parts,
            &times_cache,
        );
    }

    Ok(())
}
