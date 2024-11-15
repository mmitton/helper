use crate::Error;
use std::collections::BTreeMap;
use std::fs::{canonicalize, read_dir};
use std::path::{Path, PathBuf};

#[derive(PartialEq, Eq)]
pub enum SearchType {
    File,
    Dir,
}

pub struct InputFileCache(BTreeMap<(usize, usize), [Vec<InputFileSet>; 4]>);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputFile {
    path: PathBuf,
    info: InputFileInfo,
}

#[derive(Clone, Debug)]
pub struct InputFileSet {
    input_file: InputFile,
    expect_file: Option<InputFile>,
}

impl InputFileSet {
    pub fn files(&self) -> (String, Option<String>) {
        let input_file: String = self.input_file.path.to_str().unwrap().into();
        let expect_file: Option<String> = self
            .expect_file
            .clone()
            .map(|expect_file| expect_file.path.to_str().unwrap().into());
        (input_file, expect_file)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputFileInfo {
    year: usize,
    day: usize,
    expect: Option<usize>,
    sample: bool,
    part2: bool,
    idx: u8,
}

impl InputFile {
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    fn parse(path: PathBuf) -> Option<Self> {
        let file_name = path.file_name()?.to_str()?;
        let mut parts: Vec<&str> = file_name.split(['-', '.']).collect();

        // Input file or expected output?
        let expect = match parts.pop()? {
            "txt" => None,
            "expect1" => Some(1),
            "expect2" => Some(2),
            _ => return None,
        };

        // Is this a sub test?
        let mut next = parts.pop()?;
        let idx = if next.len() == 1 {
            let c = next.chars().next()?;
            match c {
                'a'..='z' => {
                    next = parts.pop()?;
                    c as u8 - b'a' + 1
                }
                _ => 0,
            }
        } else {
            0
        };

        // Is this part 2?
        let part2 = match &expect {
            Some(part) => {
                if next == "1" || next == "2" {
                    next = parts.pop()?;
                }
                *part == 2
            }
            None => {
                if next == "2" {
                    next = parts.pop()?;
                    true
                } else {
                    false
                }
            }
        };

        // Is this a sample file?
        let sample = if next == "sample" {
            next = parts.pop()?;
            true
        } else {
            false
        };

        // `next` should now be a day
        let day: usize = next.parse().ok()?;
        let year: usize = parts.pop()?.parse().ok()?;

        // `parts` should now be len() == 1 and == [ "input" ]
        if parts != ["input"] {
            return None;
        }

        Some(Self {
            path,
            info: InputFileInfo {
                year,
                day,
                expect,
                sample,
                part2,
                idx,
            },
        })
    }
}

impl InputFileCache {
    const SAMPLE: usize = 2;
    const REAL: usize = 0;

    pub fn new() -> Result<Self, Error> {
        let input_files = search_up("input_files", SearchType::Dir)?;
        let mut cache: BTreeMap<(usize, usize), [Vec<InputFileSet>; 4]> = BTreeMap::new();
        let mut all_files = BTreeMap::new();

        for path in read_dir(input_files)? {
            let path = path?;
            if let Some(input_file) = InputFile::parse(path.path()) {
                if let Some(input_file) = all_files.insert(input_file.info, input_file) {
                    return Err(Error::DuplicateInputFile(input_file));
                }
            }
        }

        for input_file in all_files.values() {
            if input_file.info.expect.is_none() {
                let day = cache
                    .entry((input_file.info.year, input_file.info.day))
                    .or_default();
                let idx = if input_file.info.sample {
                    Self::SAMPLE
                } else {
                    Self::REAL
                } + if input_file.info.part2 { 1 } else { 0 };

                day[idx].push(InputFileSet {
                    input_file: input_file.clone(),
                    expect_file: None,
                })
            }
        }

        // Copy from part 1 to part 2 if part 2 is empty
        for input_files in cache.values_mut() {
            for idx in [Self::SAMPLE, Self::REAL] {
                if input_files[idx + 1].is_empty() {
                    let mut files = input_files[idx].clone();
                    for mut f in files.drain(..) {
                        f.input_file.info.part2 = true;
                        input_files[idx + 1].push(f);
                    }
                }
            }

            for (idx, sets) in input_files.iter_mut().enumerate() {
                for set in sets.iter_mut() {
                    let mut expect_info = set.input_file.info;
                    expect_info.expect = Some((idx % 2) + 1);
                    if let Some(expect_file) = all_files.get(&expect_info) {
                        set.expect_file = Some(expect_file.clone());
                    }
                }
            }
        }

        Ok(Self(cache))
    }

    pub fn files(
        &self,
        year: usize,
        day: usize,
        part: usize,
        sample: bool,
    ) -> Result<&[InputFileSet], Error> {
        let cache = self.0.get(&(year, day)).ok_or(Error::MissingInput)?;
        let idx = (part - 1) + if sample { Self::SAMPLE } else { Self::REAL };

        let files = cache.get(idx).ok_or(Error::MissingInput)?;
        if files.is_empty() {
            Err(Error::MissingInput)
        } else {
            Ok(files)
        }
    }
}

pub fn download_input(year: usize, day: usize) -> Result<(), Error> {
    let mut local = search_up("input_files", SearchType::Dir)?;
    local.push(format!("input-{year}-{day:02}.txt"));
    if local.is_file() {
        return Ok(());
    }
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let cookies_path = search_up("cookies.txt", SearchType::File)?;
    let cookies = std::fs::read_to_string(cookies_path)?;
    let response = minreq::get(url).with_header("Cookie", cookies).send()?;

    std::fs::write(local, response.as_str()?)?;

    Ok(())
}

pub fn search_up(file: &str, file_type: SearchType) -> Result<PathBuf, Error> {
    let mut root = canonicalize(".")?;
    let mut path;
    loop {
        path = root.clone();
        path.push(file);
        match file_type {
            SearchType::Dir => {
                if path.is_dir() {
                    return Ok(path);
                }
            }
            SearchType::File => {
                if path.is_file() {
                    return Ok(path);
                }
            }
        }

        root = if let Some(root) = root.parent() {
            root.into()
        } else {
            return Err(Error::SearchUpFailed(file.into()));
        };
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn get_files(
    year: usize,
    day: usize,
) -> Result<(Vec<String>, Vec<String>, Vec<String>, Vec<String>), Error> {
    let input_files = search_up("input_files", SearchType::Dir)?;

    let mut sample_1 = Vec::new();
    let mut sample_2 = Vec::new();
    let mut real_1 = Vec::new();
    let mut real_2 = Vec::new();
    for path in read_dir(&input_files)? {
        let path = path?;
        let file_name = path.file_name();
        let file_name: String = if let Some(file_name) = file_name.to_str() {
            file_name.into()
        } else {
            return Err(Error::InvalidInputFile(file_name));
        };
        let mut path = input_files.clone();
        path.push(&file_name);
        let path: String = if let Some(path) = path.to_str() {
            path.into()
        } else {
            return Err(Error::InvalidInputFile(path.into_os_string()));
        };

        if let Some(base) = file_name.as_str().strip_suffix(".txt") {
            let parts: Vec<&str> = base.split('-').collect();
            if parts.len() < 3 {
                continue;
            }
            if parts[0] != "input" {
                continue;
            }
            if let Ok(file_year) = parts[1].parse() {
                if year != file_year {
                    continue;
                }
            }
            if let Ok(file_day) = parts[2].parse() {
                if day != file_day {
                    continue;
                }
            }

            let mut rest = &parts[3..];
            if rest.is_empty() {
                real_1.push(path);
                continue;
            }
            let sample = if rest[0] == "sample" {
                rest = &rest[1..];
                true
            } else {
                false
            };

            let part = if rest.is_empty() {
                1
            } else {
                rest[0].parse().unwrap_or(1)
            };

            match (sample, part) {
                (false, 1) => real_1.push(path),
                (false, 2) => real_2.push(path),
                (true, 1) => sample_1.push(path),
                (true, 2) => sample_2.push(path),
                _ => {}
            }
        }
    }
    Ok((sample_1, sample_2, real_1, real_2))
}

#[test]
fn test_input_file_cache() {
    use crate::find_day_part_files;
    let input_files_cache = InputFileCache::new().expect("Could not load input files");
    let year_days: Vec<(usize, usize)> = input_files_cache.0.keys().copied().collect();
    for (year, day) in year_days.iter().copied() {
        for part in 1..=2 {
            for sample in [true, false] {
                let old_list = find_day_part_files(year, day, part, sample);
                let new_list = input_files_cache.files(year, day, part, sample);

                match (old_list, new_list) {
                    (Err(Error::MissingInput), Err(Error::MissingInput)) => {} // Ok
                    (Ok(mut old_list), Ok(new_list)) => {
                        old_list.sort_by(|a, b| {
                            if a.0.len() != b.0.len() {
                                a.0.len().cmp(&b.0.len())
                            } else {
                                a.0.cmp(&b.0)
                            }
                        });

                        assert_eq!(
                            old_list.len(),
                            new_list.len(),
                            "{year}-{day:02} Part {part} {} data, List lengths do not match",
                            if sample { "sample" } else { "real" }
                        );

                        for (old, new) in old_list.iter().zip(new_list.iter()) {
                            let old_name = &old.0;
                            let new_name = new.input_file.path.to_str().unwrap();
                            assert_eq!(
                                old_name,
                                new_name,
                                "{year}-{day:02} Part {part} {} data, input files do not match",
                                if sample { "sample" } else { "real" }
                            );
                            let old_expect = old.1.as_deref();
                            // if let Some(old) = &old.1 {
                            //     Some(old.as_str())
                            // } else {
                            //     None
                            // };
                            let new_expect = if let Some(new) = &new.expect_file {
                                new.path.to_str()
                            } else {
                                None
                            };
                            assert_eq!(
                                old_expect,
                                new_expect,
                                "{year}-{day:02} Part {part} {} data, expect files do not match",
                                if sample { "sample" } else { "real" }
                            );
                        }
                    }
                    (old_list, new_list) => {
                        panic!("old_list: {old_list:?}\nnew_list: {new_list:?}");
                    }
                }
            }
        }
    }
}
