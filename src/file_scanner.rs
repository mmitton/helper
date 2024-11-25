use crate::Error;
use std::collections::BTreeMap;
use std::fs::{canonicalize, read_dir};
use std::path::{Path, PathBuf};

#[derive(PartialEq, Eq)]
pub enum SearchType {
    File,
    Dir,
}

pub struct InputFileCache<const N: usize>(BTreeMap<(usize, usize), [[Vec<InputFileSet>; N]; 2]>);

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
    part: usize,
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
            "expect3" => Some(3),
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

        let part_strs = ["1", "2", "3"];

        // Is this part 2?
        let part = match &expect {
            Some(part) => {
                if part_strs.contains(&next) {
                    next = parts.pop()?;
                }
                *part
            }
            None => {
                if let Some(idx) = part_strs.iter().position(|s| *s == next) {
                    next = parts.pop()?;
                    idx + 1
                } else {
                    1
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
                part,
                idx,
            },
        })
    }
}

impl<const N: usize> InputFileCache<N> {
    const REAL: usize = 0;
    const SAMPLE: usize = 1;

    pub fn new(allow_copy: bool) -> Result<Self, Error> {
        let input_files = search_up("input_files", SearchType::Dir)?;
        let mut cache: BTreeMap<(usize, usize), [[Vec<InputFileSet>; N]; 2]> = BTreeMap::new();
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
                    .or_insert(std::array::from_fn(|_| std::array::from_fn(|_| Vec::new())));
                let parts = if input_file.info.sample {
                    &mut day[Self::SAMPLE]
                } else {
                    &mut day[Self::REAL]
                };

                parts[input_file.info.part - 1].push(InputFileSet {
                    input_file: input_file.clone(),
                    expect_file: None,
                })
            }
        }

        // Copy from part n to part n+1 if part n+1 is empty
        // and set expect files
        for input_files in cache.values_mut() {
            for (real_sample_idx, real_sample) in input_files.iter_mut().enumerate() {
                if allow_copy || real_sample_idx != Self::REAL {
                    for i in 1..real_sample.len() {
                        if real_sample[i].is_empty() {
                            let copied = real_sample[i - 1]
                                .iter()
                                .map(|f| {
                                    let mut input_file = f.input_file.clone();
                                    input_file.info.part += 1;
                                    InputFileSet {
                                        input_file,
                                        expect_file: None,
                                    }
                                })
                                .collect();
                            real_sample[i] = copied;
                        }
                    }
                }

                for (part, files) in real_sample.iter_mut().enumerate() {
                    for file in files.iter_mut() {
                        let mut expect_info = file.input_file.info;
                        expect_info.expect = Some(part + 1);
                        if let Some(expect_file) = all_files.get(&expect_info) {
                            file.expect_file = Some(expect_file.clone());
                        }
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

        let parts = if sample {
            &cache[Self::SAMPLE]
        } else {
            &cache[Self::REAL]
        };
        let files = parts.get(part - 1).ok_or(Error::MissingInput)?;
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

#[test]
fn test_input_file_cache() {
    let input_files_cache: InputFileCache<3> =
        InputFileCache::new(false).expect("Could not load input files");
    println!("{:?}", input_files_cache.0);
    println!();
    println!("{:?}", input_files_cache.files(2024, 1, 1, false));
    println!();
    println!("{:?}", input_files_cache.files(2024, 1, 1, true));
}
