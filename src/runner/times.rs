use crate::Error;
use std::collections::BTreeMap;
use std::time::Duration;

pub(super) struct TimesCacheEntry {
    pub(super) day: usize,
    pub(super) results: BTreeMap<u8, Result<Duration, Error>>,
}

pub(super) fn print_times(
    md: bool,
    readme_header: &str,
    run_count: usize,
    parts: u8,
    times_cache: &BTreeMap<usize, Vec<TimesCacheEntry>>,
) {
    fn print_dashed(parts: u8, header: &str) {
        fn dashed(len: usize) {
            for _ in 0..len {
                print!("-");
            }
        }
        print!("+");
        dashed(header.len() + 2);
        print!("+");
        for _ in 1..=parts {
            dashed(12);
            print!("+");
        }
        if header == "Year" {
            dashed(13);
            println!("+");
        } else {
            for _ in 1..=parts {
                dashed(10);
                print!("+");
            }
            println!();
        }
    }
    fn print_header(md: bool, parts: u8, header: &str) {
        if md {
            print!("| {header} |");
            for part in 1..=parts {
                print!(" Part {part} |");
            }
            if header == "Year" {
                println!(" Total |");
            } else {
                for part in 1..=parts {
                    print!(" Part {part} % |");
                }
                println!();
            }
            print!("| ---: |");
            for _ in 1..=parts {
                print!(" --: |");
            }
            if header == "Year" {
                println!(" ---: |");
            } else {
                for _ in 1..=parts {
                    print!(" --: |");
                }
                println!();
            }
        } else {
            print_dashed(parts, header);
            print!("| {header} |");
            for part in 1..=parts {
                print!(" {part:>10} |", part = format!("Part {part}"));
            }
            if header == "Year" {
                println!(" {total:>11} |", total = "Total");
            } else {
                for part in 1..=parts {
                    print!(" {part:>8} |", part = format!("Part {part} %"));
                }
                println!();
            }
            print_dashed(parts, header);
        }
    }

    if md {
        use sysinfo::{CpuRefreshKind, RefreshKind, System};

        let s =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

        println!("{readme_header}");
        println!();
        println!("Run on {}, single threaded.", s.cpus()[0].brand());
        println!();
    }

    if times_cache.len() > 1 {
        print_header(md, parts, "Year");
        for (year, times_cache) in times_cache.iter().rev() {
            let mut total = Duration::new(0, 0);
            let mut part_totals: BTreeMap<u8, Duration> = BTreeMap::new();
            for entry in times_cache.iter() {
                for (part, result) in entry.results.iter() {
                    if let Ok(dur) = result {
                        *part_totals.entry(*part).or_default() += *dur;
                        total += *dur;
                    }
                }
            }
            if md {
                print!("| {year} |");
                for part in 1..=parts {
                    if let Some(dur) = part_totals.get(&part) {
                        print!(" {dur:0.5} s |", dur = dur.as_secs_f64());
                    } else {
                        print!(" |");
                    }
                }
                println!(" {dur:0.5} s |", dur = total.as_secs_f64());
            } else {
                print!("| {year} |");
                for part in 1..=parts {
                    if let Some(dur) = part_totals.get(&part) {
                        print!(" {dur:>10} |", dur = format!("{:0.5} s", dur.as_secs_f64()));
                    } else {
                        print!(" |");
                    }
                }
                println!(
                    " {dur:>11} |",
                    dur = format!("{:0.5} s", total.as_secs_f64())
                );
            }
        }
        if !md {
            print_dashed(parts, "Year")
        }
        println!();
    }

    for (year, times_cache) in times_cache.iter().rev() {
        let mut total = Duration::new(0, 0);
        let mut part_totals: BTreeMap<u8, Duration> = BTreeMap::new();
        for entry in times_cache.iter() {
            for (part, result) in entry.results.iter() {
                if let Ok(dur) = result {
                    *part_totals.entry(*part).or_default() += *dur;
                    total += *dur;
                }
            }
        }

        if run_count > 1 {
            println!("Year: {year}  Averaged over {run_count} runs.");
        } else {
            println!("Year: {year}");
        }
        print_header(md, parts, "Day");
        for TimesCacheEntry { day, results } in times_cache.iter().rev() {
            if !results.values().any(|result| result.is_ok()) {
                continue;
            }
            if md {
                print!("| {day} |");
            } else {
                print!("| {day:>3} |");
            }
            for part in 1..=parts {
                let time = if let Some(Ok(dur)) = results.get(&part) {
                    format!("{:0.5} s", dur.as_secs_f64())
                } else {
                    String::new()
                };
                if md {
                    print!(" {time} |");
                } else {
                    print!(" {time:>10} |");
                }
            }
            for part in 1..=parts {
                let percent = if let Some(Ok(dur)) = results.get(&part) {
                    format!("{:0.2}%", dur.as_secs_f64() / total.as_secs_f64() * 100.)
                } else {
                    String::new()
                };
                if md {
                    print!(" {percent} |");
                } else {
                    print!(" {percent:>8} |");
                }
            }
            println!();
        }

        if !md {
            print_dashed(parts, "Day");
        }
        print!("| All |");
        for part in 1..=parts {
            let time = if let Some(dur) = part_totals.get(&part) {
                format!("{:0.5} s", dur.as_secs_f64())
            } else {
                String::new()
            };
            if md {
                print!(" {time} |");
            } else {
                print!(" {time:>10} |");
            }
        }
        if md {
            for _ in 1..=parts - 2 {
                print!(" |");
            }
            print!(" Total | {total:0.5} s |", total = total.as_secs_f64());
        } else {
            for _ in 1..=parts - 2 {
                print!(" {:9} ", " ");
            }
            print!(
                " {total:>19} |",
                total = format!("Total {total:0.5} s", total = total.as_secs_f64())
            );
        }
        println!();

        if !md {
            print_dashed(parts, "Day");
        }
        println!();
    }
}
