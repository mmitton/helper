use std::cell::UnsafeCell;
use std::fmt::{Display, Write};

thread_local! {
    pub static OUTPUT: UnsafeCell<Output> = UnsafeCell::new(Output::default());
}

#[derive(Copy, Clone, Default)]
pub struct YearDayPart {
    year: usize,
    day: usize,
    part: usize,
}

impl YearDayPart {
    pub fn new(year: usize, day: usize, part: usize) -> Self {
        Self { year, day, part }
    }
}

impl Display for YearDayPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{year}-{day:02} Part {part}",
            year = self.year,
            day = self.day,
            part = self.part
        )
    }
}

#[derive(Default)]
pub struct Output {
    pub mode: Mode,
}

#[derive(Default)]
pub enum Mode {
    #[default]
    NoOutput,
    Stdout {
        ydp: YearDayPart,
        new_line: bool,
    },
    Capture {
        ydp: YearDayPart,
        new_line: bool,
        capture: String,
    },
}

impl Output {
    pub fn start_run(&mut self, ydp: YearDayPart) {
        self.mode.reset(ydp);
    }

    pub fn ensure_nl(&mut self) {
        self.mode.ensure_nl();
    }

    pub fn get_capture(&mut self) -> Option<String> {
        self.mode.get_capture()
    }

    pub fn stdout(&mut self) {
        self.mode = Mode::Stdout {
            ydp: Default::default(),
            new_line: true,
        };
    }

    pub fn capture(&mut self) {
        self.mode = Mode::Capture {
            ydp: Default::default(),
            new_line: true,
            capture: String::new(),
        };
    }

    pub fn no_output(&mut self) {
        self.mode = Mode::NoOutput;
    }
}

impl Mode {
    pub fn get_capture(&self) -> Option<String> {
        match self {
            Self::NoOutput | Self::Stdout { .. } => None,
            Self::Capture { capture, .. } => {
                if capture.is_empty() {
                    None
                } else {
                    Some(capture.clone())
                }
            }
        }
    }

    pub fn reset(&mut self, ydp: YearDayPart) {
        match self {
            Self::NoOutput => {}
            Self::Stdout {
                ydp: _ydp,
                new_line,
            } => {
                *_ydp = ydp;
                *new_line = true;
            }
            Self::Capture {
                ydp: _ydp,
                new_line,
                capture,
            } => {
                *_ydp = ydp;
                *new_line = true;
                capture.clear();
            }
        }
    }

    fn ensure_nl(&mut self) {
        match self {
            Self::NoOutput => {}
            Self::Stdout { new_line, .. } | Self::Capture { new_line, .. } => {
                if !*new_line {
                    let _ = writeln!(self);
                }
            }
        }
    }
}

impl Write for Mode {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        use std::io::Write;
        macro_rules! output {
            ($ydp:ident, $new_line:ident, $w:expr) => {{
                #[allow(unused_mut)]
                let mut w = $w;
                let (s, ends_with_nl) = if let Some(s) = s.strip_suffix('\n') {
                    (s, true)
                } else {
                    (s, false)
                };
                for (idx, s) in s.split('\n').enumerate() {
                    if idx != 0 {
                        let _ = write!(w, "\n");
                        *$new_line = true;
                    }
                    if *$new_line {
                        let _ = write!(w, "{ydp}: ", ydp = $ydp);
                        *$new_line = false;
                    }
                    let _ = write!(w, "{s}");
                }
                if ends_with_nl {
                    let _ = write!(w, "\n");
                }
                *$new_line = ends_with_nl;
            }};
        }

        match self {
            Self::NoOutput => {}
            Self::Stdout { ydp, new_line } => {
                output!(ydp, new_line, std::io::stdout().lock())
            }
            Self::Capture {
                ydp,
                new_line,
                capture,
            } => output!(ydp, new_line, capture),
        }

        Ok(())
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        match self {
            Self::NoOutput => {}
            Self::Stdout { .. } | Self::Capture { .. } => self.write_str(&args.to_string())?,
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        match self {
            Self::NoOutput => {}
            Self::Stdout { .. } | Self::Capture { .. } => self.write_str(&format!("{c}"))?,
        }

        Ok(())
    }
}
