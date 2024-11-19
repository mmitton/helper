use crate::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::{BitOr, Deref, DerefMut};
use std::path::Path;

#[derive(Debug)]
pub struct Lines(Vec<String>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinesOpt(u8);

impl LinesOpt {
    pub const RAW: Self = Self(0);
    pub const TRIM: Self = Self(1 << 0);
    pub const REMOVE_COMMENTS: Self = Self(1 << 1);
    pub const REMOVE_EMPTY: Self = Self(2 << 1);
    pub const ALL: Self = Self(!0);

    fn contains(&self, rhs: Self) -> bool {
        self.0 & rhs.0 == rhs.0
    }
}

impl BitOr for LinesOpt {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
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

    pub fn single_line(mut self) -> Result<String, Error> {
        if self.len() == 1 {
            Ok(self.remove(0))
        } else {
            Err(Error::InvalidInput(format!(
                "Expected only 1 line, got {}",
                self.len()
            )))
        }
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
