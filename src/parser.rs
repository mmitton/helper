use crate::Error;
use bitflags::bitflags;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Lines(Vec<String>);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LinesOpt: u8 {
        const RAW = 0;
        const TRIM = 1 << 0;
        const REMOVE_COMMENTS = 1 << 1;
        const REMOVE_EMPTY = 2 << 1;
        const ALL = !0;
    }
}

impl Lines {
    pub fn from_bufread(r: impl BufRead, options: LinesOpt) -> Result<Self, Error> {
        let mut lines = Vec::new();
        for line in r.lines() {
            let line = line?;
            let mut l = line.as_str();
            if options.contains(LinesOpt::TRIM) {
                l = l.trim();
            }
            if options.contains(LinesOpt::REMOVE_COMMENTS) && l.starts_with('#') {
                continue;
            }
            if options.contains(LinesOpt::REMOVE_EMPTY) && l.is_empty() {
                continue;
            }
            lines.push(String::from(l));
        }

        Ok(Self(lines))
    }

    pub fn from_reader(r: impl Read, options: LinesOpt) -> Result<Self, Error> {
        Self::from_bufread(BufReader::new(r), options)
    }

    pub fn from_path(path: impl AsRef<Path>, options: LinesOpt) -> Result<Self, Error> {
        Self::from_reader(File::open(path)?, options)
    }

    pub fn iter(&self) -> LinesIter {
        LinesIter(self.0.iter())
    }
}

impl Deref for Lines {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Lines {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct LinesIter<'a>(std::slice::Iter<'a, String>);

impl<'a> Iterator for LinesIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(s) => Some(s.as_str()),
            None => None,
        }
    }
}

pub fn find_day_part_files(
    year: usize,
    day: usize,
    part: usize,
    sample_data: bool,
) -> Result<Vec<(String, Option<String>)>, Error> {
    super::file_scanner::download_input(year, day)?;

    let (sample_1, sample_2, real_1, real_2) = super::file_scanner::get_files(year, day)?;

    let (part1, mut part2) = if sample_data {
        (sample_1, sample_2)
    } else {
        (real_1, real_2)
    };

    if part == 2 && part2.is_empty() {
        part2.extend(part1.iter().cloned());
    }

    let files = match part {
        1 => part1,
        2 => part2,
        _ => unreachable!(),
    };

    if files.is_empty() {
        Err(Error::MissingInput)
    } else {
        let mut ret = Vec::new();
        for f in files {
            let output = if let Some(output) = f.strip_suffix(".txt") {
                let output = format!("{output}.expect{part}");
                if PathBuf::from(&output).is_file() {
                    Some(output)
                } else {
                    None
                }
            } else {
                None
            };

            ret.push((f, output));
        }
        ret.sort();
        Ok(ret)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    const INPUT: &str = " # Hello

World
  
# Todo 
Yup  ";

    fn reader() -> Cursor<Vec<u8>> {
        Cursor::new(INPUT.into())
    }

    #[test]
    fn test_raw() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::RAW)?
                .iter()
                .collect::<Vec<&str>>(),
            [" # Hello", "", "World", "  ", "# Todo ", "Yup  "]
        );

        Ok(())
    }

    #[test]
    fn test_trim() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::TRIM)?
                .iter()
                .collect::<Vec<&str>>(),
            ["# Hello", "", "World", "", "# Todo", "Yup"]
        );

        Ok(())
    }

    #[test]
    fn test_remove_comments() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::REMOVE_COMMENTS)?
                .iter()
                .collect::<Vec<&str>>(),
            [" # Hello", "", "World", "  ", "Yup  "]
        );

        Ok(())
    }

    #[test]
    fn test_remove_empty() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::REMOVE_EMPTY)?
                .iter()
                .collect::<Vec<&str>>(),
            [" # Hello", "World", "  ", "# Todo ", "Yup  "]
        );

        Ok(())
    }

    #[test]
    fn test_trim_remove_comments() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::TRIM | LinesOpt::REMOVE_COMMENTS)?
                .iter()
                .collect::<Vec<&str>>(),
            ["", "World", "", "Yup"]
        );

        Ok(())
    }

    #[test]
    fn test_trim_remove_empty() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::TRIM | LinesOpt::REMOVE_EMPTY)?
                .iter()
                .collect::<Vec<&str>>(),
            ["# Hello", "World", "# Todo", "Yup"]
        );

        Ok(())
    }

    #[test]
    fn test_remove_empty_and_comments() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::REMOVE_COMMENTS | LinesOpt::REMOVE_EMPTY)?
                .iter()
                .collect::<Vec<&str>>(),
            [" # Hello", "World", "  ", "Yup  "]
        );

        Ok(())
    }

    #[test]
    fn test_all() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::ALL)?
                .iter()
                .collect::<Vec<&str>>(),
            ["World", "Yup"]
        );

        Ok(())
    }
}
