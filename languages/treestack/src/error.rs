use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut, Range};

#[derive(Debug)]
pub struct RangeError {
    pub message: String,
    pub range: Range<usize>,
}

impl RangeError {
    pub fn pretty_print(&self, program: &str, line_numbers: bool) {
        let Range { start, end } = self.range;
        // fix range to be printed
        eprintln!("\n\x1b[91m\x1b[1mError\x1b[0m: {}:{}: {}", start, end, self.message);

        let lines = program.lines();
        let mut line_start = 0;

        let mut line_no = 1;
        for line in lines {
            let line_end = line_start + line.len() + 1;
            let starter = if line_numbers { format!("{line_no} |") } else { String::new() };
            let starter_len = starter.len();
            eprintln!("{starter} {line}");
            if start <= line_end && end > line_start {
                eprintln!(
                    "\x1b[91m\x1b[1m{}{}\x1b[0m",
                    " ".repeat(start - line_start + starter_len),
                    "^".repeat(end + 1 - start)
                );
            }
            line_start = line_end;
            line_no += 1;
        }
    }
}

pub struct Positioned<T> {
    pub inner: T,
    pub range: Range<usize>,
}

pub fn position<T>(inner: T, range: Range<usize>) -> Positioned<T> {
    Positioned { inner, range }
}

impl<T: Clone> Clone for Positioned<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone(), range: self.range.clone() }
    }
}

impl<T: Debug> Debug for Positioned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.inner.fmt(f)
    }
}

impl<T> Deref for Positioned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for Positioned<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}
