use std::time::{Duration, Instant};

use crate::{output, Error, InputFileCache, NewRunner, YearDayPart};
use colored::Colorize;

fn run_part(
    new_runner: &NewRunner,
    part: u8,
    input: impl AsRef<[u8]>,
    expect: Option<impl AsRef<[u8]>>,
) -> Result<String, Error> {
    let mut runner = new_runner();
    runner.parse(input.as_ref(), part)?;
    let output = runner.run_part(part)?;

    let output = output.to_string();
    let output = output.trim_end_matches('\n');
    if let Some(expect) = expect {
        let expect = std::str::from_utf8(expect.as_ref())?;
        let expect = expect.trim_end_matches('\n');
        if expect == output {
            Ok(output.to_string())
        } else {
            Err(Error::WrongAnswer(output.to_string(), expect.to_string()))
        }
    } else {
        Err(Error::MissingExpect(output.to_string()))
    }
}

#[allow(clippy::too_many_arguments)]
pub fn run<const N: usize>(
    sample_data: bool,
    new_runner: &NewRunner,
    times: bool,
    year: usize,
    day: usize,
    part: u8,
    input_file_cache: &InputFileCache<N>,
) -> Result<Duration, Error> {
    let ydp = YearDayPart::new(year, day, part as usize);

    let f = input_file_cache.files(year, day, part as usize, sample_data)?;
    let files: Vec<(String, Option<String>)> = f.iter().map(|f| f.files()).collect();

    let mut elapsed = Vec::new();
    loop {
        for (input_path, expect_path) in files.iter() {
            if !times {
                println!("{ydp}: Using {input_path}");
                output(|output| output.start_run(ydp));
            }

            let start = Instant::now();
            let input = std::fs::read(input_path)?;
            let expect = if let Some(p) = &expect_path {
                Some(std::fs::read(p)?)
            } else {
                None
            };
            let result = run_part(new_runner, part, &input, expect.as_ref());

            elapsed.push(start.elapsed());

            if !times {
                output(|output| output.ensure_nl());
                if result.is_err() {
                    if let Some(capture) = output(|output| output.get_capture()) {
                        print!("{capture}");
                    }
                }
                match result {
                    Ok(output) => {
                        if !output.contains('\n') {
                            println!("{ydp}:   Answer: {output}", output = output.bright_green());
                        } else {
                            for line in output.split('\n') {
                                println!("{ydp}:   Answer: {output}", output = line.bright_green());
                            }
                        }
                    }
                    Err(Error::WrongAnswer(output, expect)) => {
                        if !output.contains('\n') {
                            println!("{ydp}:   Answer: {output}", output = output.bright_red());
                        } else {
                            for line in output.split('\n') {
                                println!("{ydp}:   Answer: {output}", output = line.bright_red());
                            }
                        }
                        println!("{ydp}: ERROR: Output did not match expected output.");
                        if !expect.contains('\n') {
                            println!("{ydp}: Expected: {expect}", expect = expect.bright_yellow());
                        } else {
                            for line in expect.split('\n') {
                                println!(
                                    "{ydp}: Expected: {output}",
                                    output = line.bright_yellow()
                                );
                            }
                        }
                    }
                    Err(Error::MissingExpect(output)) => {
                        if !output.contains('\n') {
                            println!("{ydp}:   Answer: {output}", output = output.bright_yellow());
                        } else {
                            for line in output.split('\n') {
                                println!(
                                    "{ydp}:   Answer: {output}",
                                    output = line.bright_yellow()
                                );
                            }
                        }
                        println!("{ydp}: No expected output to compare");
                    }
                    Err(Error::Skipped) => {
                        println!("{ydp}: {}", "skipped".bright_yellow());
                    }
                    Err(e) => {
                        println!("{ydp}: Error: {}", format!("{e:?}").bright_red());
                    }
                }
                println!("{ydp}: {elapsed:?}");
                println!();
            } else if let Err(e) = result {
                if !matches!(e, Error::Skipped) {
                    return Err(e);
                }
            }
        }
        if !times {
            break;
        }
        if elapsed.iter().sum::<Duration>() > Duration::from_secs_f64(0.2) || elapsed.len() >= 10 {
            break;
        }
    }
    if elapsed.len() > 5 {
        // Remove lowest and highest runs
        elapsed.sort();
        elapsed.remove(0);
        elapsed.pop();
    }
    Ok(elapsed.iter().sum::<Duration>() / elapsed.len() as u32)
}
