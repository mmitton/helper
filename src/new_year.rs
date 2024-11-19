use chrono::prelude::*;
use std::io::Write;
use std::path::PathBuf;

use crate::{search_up, Error, SearchType};

#[derive(Copy, Clone)]
pub struct Day {
    day: usize,
    parts: usize,
}

impl Day {
    pub fn new(day: usize, parts: usize) -> Self {
        Self { day, parts }
    }
}

pub struct Config {
    prefix: &'static str,
    days: Vec<Day>,
}

impl Config {
    pub fn new(prefix: &'static str, days: Vec<Day>) -> Self {
        Self { prefix, days }
    }
}

fn exec(cmd: &str, args: &[&str]) -> Result<(), Error> {
    use std::process::Command;

    let mut cmd = Command::new(cmd);
    cmd.args(args);
    let status = cmd.status()?;
    assert!(status.success());
    Ok(())
}

fn add_dependency(section: &str, dep: &str) -> Result<(), Error> {
    let mut lines: Vec<String> = std::fs::read_to_string("Cargo.toml")?
        .split_terminator('\n')
        .map(|s| s.into())
        .collect();

    let mut found = false;
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == section {
            found = true;
            lines.insert(i + 1, dep.into());
            break;
        }
    }

    if !found {
        // Could not find the section, add it!
        lines.push("".into());
        lines.push(section.into());
        lines.push(dep.into());
    }

    std::fs::write("Cargo.toml", lines.join("\n"))?;
    Ok(())
}

fn create_year(year: usize, config: Config) -> Result<(), Error> {
    // Find runner crate
    let runner_path = search_up("runner", SearchType::Dir)?;
    let root_path = runner_path.parent().unwrap();
    std::env::set_current_dir(root_path)?;

    // Check to see if crate exists
    let crate_path_str = format!("{prefix}_{year}", prefix = config.prefix);
    let mut crate_path: PathBuf = root_path.into();
    crate_path.push(&crate_path_str);
    if crate_path.exists() {
        return Err(Error::YearExists(year));
    }

    // Create crate library and add it as a dependency to runner
    exec("cargo", &["new", "--lib", &crate_path_str])?;

    add_dependency(
        "[workspace.dependencies]",
        format!(
            "{prefix}_{year} = {{ path = \"{prefix}_{year}\" }}",
            prefix = config.prefix
        )
        .as_str(),
    )?;
    std::env::set_current_dir(runner_path)?;
    add_dependency(
        "[dependencies]",
        format!("{prefix}_{year}.workspace = true", prefix = config.prefix).as_str(),
    )?;

    // Change in to crate folder and build files
    std::env::set_current_dir(crate_path)?;
    add_dependency("[dependencies]", "helper.workspace = true")?;

    let mut mod_path = PathBuf::from("src");
    mod_path.push("lib.rs");
    let mut m = std::fs::File::create(mod_path)?;
    writeln!(m, "use helper::NewRunner;")?;
    writeln!(m, "use std::collections::BTreeMap;")?;
    writeln!(m)?;
    for Day { day, parts } in config.days.iter().copied() {
        writeln!(m, "mod day_{day:02};")?;

        let mut day_path = PathBuf::from("src");
        day_path.push(format!("day_{day:02}.rs"));
        let mut d = std::fs::File::create(day_path)?;
        writeln!(d, "#[allow(unused_imports)]")?;
        writeln!(
            d,
            "use helper::{{print, println, Error, HashMap, HashSet, Lines, LinesOpt}};"
        )?;
        writeln!(d)?;
        writeln!(d, "#[derive(Default)]")?;
        writeln!(d, "pub struct Day{day:02} {{}}")?;
        writeln!(d)?;
        writeln!(d, "impl Day{day:02} {{")?;
        writeln!(d, "    pub fn new() -> Self {{")?;
        writeln!(d, "        Self::default()")?;
        writeln!(d, "    }}")?;
        for part in 1..=parts {
            writeln!(d)?;
            writeln!(
                d,
                "    fn part{part}(&mut self) -> Result<helper::RunOutput, Error> {{"
            )?;
            writeln!(d, "        Err(Error::Unsolved)")?;
            writeln!(d, "    }}")?;
        }
        writeln!(d, "}}")?;
        writeln!(d)?;
        writeln!(d, "impl helper::Runner for Day{day:02} {{")?;
        writeln!(
            d,
            "    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {{"
        )?;
        writeln!(
            d,
            "        let _lines = Lines::from_bufread(file, LinesOpt::RAW)?;"
        )?;
        writeln!(d, "        Ok(())")?;
        writeln!(d, "    }}")?;
        writeln!(d)?;
        writeln!(
            d,
            "    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {{"
        )?;

        writeln!(d, "        match part {{")?;
        for part in 1..=parts {
            writeln!(d, "            {part} => self.part{part}(),")?;
        }
        writeln!(d, "            _ => Err(Error::Skipped),")?;
        writeln!(d, "        }}")?;
        writeln!(d, "    }}")?;
        writeln!(d, "}}")?;
    }

    writeln!(m)?;
    writeln!(
        m,
        "pub fn register(runners: &mut BTreeMap<(usize, usize), (u8, NewRunner)>) {{"
    )?;

    for Day { day, parts } in config.days.iter() {
        writeln!(
            m,
            "    runners.insert(({year}, {day}), ({parts}, || Box::new(day_{day:02}::Day{day:02}::new())));"
        )?;
    }
    writeln!(m, "}}")?;
    Ok(())
}

pub fn main(config: Config) -> Result<(), Error> {
    let env: Vec<String> = std::env::args().collect();
    if env.len() != 2 {
        println!("Usage: {} year", env[0]);
    }

    let now = Local::now();
    let cur_year = now.year() as usize;
    let year: usize = env[1].parse().expect("Unable to parse year");
    if year < 2015 || year > cur_year {
        panic!("Year {year} out of range.  2015..={cur_year}");
    }

    println!("Making new year for {year}");

    create_year(year, config)
}
