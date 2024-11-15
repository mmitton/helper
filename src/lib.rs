mod algorithms;
mod bit_array;
mod bit_grid;
mod error;
mod file_scanner;
mod integer;
mod md5;
mod output;
mod parser;
mod permutations;
mod priority_vec;
mod run_output;
mod small_vec;
mod tile_set;

pub use algorithms::*;
pub use bit_array::BitArray;
pub use bit_grid::*;
pub use error::Error;
pub use file_scanner::{download_input, search_up, InputFileCache, SearchType};
pub use integer::Integer;
pub use md5::{MD5String, MD5};
pub use output::{Output, YearDayPart, OUTPUT};
pub use parser::{find_day_part_files, Lines, LinesIter, LinesOpt};
pub use permutations::Permutations;
pub use priority_vec::PriorityVec;
pub use run_output::RunOutput;
pub use small_vec::SmallVec;
pub use tile_set::{Point, Tile, TileSet};

pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;
pub type HashSet<K> = rustc_hash::FxHashSet<K>;

pub trait Runner {
    fn parse(&mut self, file: &[u8], part: u8) -> Result<(), Error>;
    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error>;
}

pub type NewRunner = fn() -> Box<dyn Runner>;

pub fn output<F, R>(f: F) -> R
where
    F: Fn(&mut Output) -> R,
{
    output::OUTPUT.with(|output| f(unsafe { &mut *output.get() }))
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        $crate::output(|output| {
            use std::fmt::Write;
            let _ = write!(output.mode, $($args)*);
        });
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::output(|output| {
            use std::fmt::Write;
            let _ = writeln!(output.mode);
        });
    };

    ($($args:tt)*) => {
        $crate::output(|output| {
            use std::fmt::Write;
            let _ = writeln!(output.mode, $($args)*);
        });
    };
}
