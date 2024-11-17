use super::{Error, InputFileCache};
use std::{cmp::Ordering, collections::BTreeMap};

mod args;
mod run;
mod times;

use times::TimesCacheEntry;

pub fn main<F>(download_input: bool, register: F) -> Result<(), Error>
where
    F: Fn(&mut BTreeMap<(usize, usize), (u8, super::NewRunner)>),
{
    let (sample_data, no_capture, times, md, target_year, target_day) = args::get();

    let mut runners = BTreeMap::new();
    register(&mut runners);

    if times.is_some() {
        super::output(|output| output.no_output());
    } else if cfg!(debug_assertions) || no_capture {
        super::output(|output| output.stdout());
    } else {
        super::output(|output| output.capture());
    }

    use chrono::prelude::*;
    let today = Local::now();

    let mut times_cache: BTreeMap<usize, Vec<TimesCacheEntry>> = BTreeMap::new();
    let run_count = times.unwrap_or(1);

    if download_input {
        if let (Some(year), Some(day)) = (target_year, target_day) {
            super::download_input(year, day)?;
        }
    }

    let input_file_cache: InputFileCache<3> = super::InputFileCache::new()?;
    for ((year, day), (parts, new_runner)) in runners.iter().rev() {
        if let Some(target_year) = target_year {
            if target_year != *year {
                continue;
            }
        }
        if let Some(target_day) = target_day {
            if target_day != *day {
                continue;
            }
        }

        match (
            (today.year() as usize).cmp(year),
            (today.month() as usize).cmp(&11),
            (today.day() as usize).cmp(day),
        ) {
            (Ordering::Less, _, _) => continue,
            (Ordering::Equal, Ordering::Less, _) => continue,
            (Ordering::Equal, Ordering::Equal, Ordering::Less) => continue,
            _ => {}
        }

        let mut times_cache_entry = TimesCacheEntry {
            day: *day,
            results: BTreeMap::new(),
        };
        for part in 1..=*parts {
            let result = run::run(
                sample_data,
                new_runner,
                times.is_none(),
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

    if times.is_some() && !times_cache.is_empty() {
        let parts = *runners.values().map(|(parts, _)| parts).max().unwrap();
        times::print_times(md, run_count, parts, &times_cache);
    }

    Ok(())
}
