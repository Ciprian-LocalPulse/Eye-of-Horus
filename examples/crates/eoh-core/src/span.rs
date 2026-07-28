//! Source-location spans used by diagnostics throughout the toolchain.

use serde::{Deserialize, Serialize};

/// A half-open byte range `[start, end)` within a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Span {
    /// Byte offset of the first character.
    pub start: usize,
    /// Byte offset one past the last character.
    pub end: usize,
    /// Index into the source-file table (0 = anonymous/REPL).
    pub file_id: u32,
}

impl Span {
    /// Construct a new span.
    pub fn new(start: usize, end: usize, file_id: u32) -> Self {
        debug_assert!(start <= end, "span start must not exceed end");
        Self { start, end, file_id }
    }

    /// Merge two spans, producing the smallest span covering both.
    pub fn merge(self, other: Self) -> Self {
        debug_assert_eq!(self.file_id, other.file_id, "cannot merge spans from different files");
        Self {
            start:   self.start.min(other.start),
            end:     self.end.max(other.end),
            file_id: self.file_id,
        }
    }

    /// Byte length of this span.
    pub fn len(&self) -> usize { self.end - self.start }

    /// `true` if this span covers zero bytes.
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
