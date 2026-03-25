use std::ops::Range;

pub trait Rope {
    const NAME: &'static str;
    const EDITS_USE_BYTE_OFFSETS: bool = false;

    fn from_str(s: &str) -> Self;

    fn insert(&mut self, at_offset: usize, text: &str);

    fn remove(&mut self, between_offsets: Range<usize>);

    #[inline(always)]
    fn replace(&mut self, between_offsets: Range<usize>, text: &str) {
        let Range { start, end } = between_offsets;

        if end > start {
            self.remove(start..end);
        }

        if !text.is_empty() {
            self.insert(start, text);
        }
    }

    /// The returned length is interpreted as either number of codepoints or
    /// the number of bytes depending on the value of
    /// [`EDITS_USE_BYTE_OFFSETS`](Self::EDITS_USE_BYTE_OFFSETS).
    fn len(&self) -> usize;
}

impl Rope for String {
    const NAME: &'static str = "String";
    const EDITS_USE_BYTE_OFFSETS: bool = true;

    #[inline(always)]
    fn from_str(s: &str) -> Self {
        s.into()
    }

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.insert_str(at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.replace_range(range, "");
    }

    #[inline(always)]
    fn replace(&mut self, range: Range<usize>, s: &str) {
        self.replace_range(range, s);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl Rope for crop::Rope {
    const NAME: &'static str = "crop";
    const EDITS_USE_BYTE_OFFSETS: bool = true;

    #[inline(always)]
    fn from_str(s: &str) -> Self {
        s.into()
    }

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.insert(at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.delete(range);
    }

    #[inline(always)]
    fn replace(&mut self, range: Range<usize>, s: &str) {
        self.replace(range, s);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.byte_len()
    }
}

impl Rope for jumprope::JumpRope {
    const NAME: &'static str = "JumpRope";

    #[inline(always)]
    fn from_str(s: &str) -> Self {
        s.into()
    }

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.insert(at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.remove(range);
    }

    #[inline(always)]
    fn replace(&mut self, range: Range<usize>, s: &str) {
        self.replace(range, s);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len_chars()
    }
}

impl Rope for jumprope::JumpRopeBuf {
    // We put `Buf` before `Rope` to be able to only run the `JumpRope`
    // benchmarks by passing `JumpR***` to the CLI.
    const NAME: &'static str = "JumpBufRope";

    #[inline(always)]
    fn from_str(s: &str) -> Self {
        s.into()
    }

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.insert(at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.remove(range);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len_chars()
    }
}

impl Rope for ropey::Rope {
    const NAME: &'static str = "Ropey";

    #[inline(always)]
    fn from_str(s: &str) -> Self {
        s.into()
    }

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.insert(at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.remove(range);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len_chars()
    }
}

impl Rope for zed_rope::Rope {
    const NAME: &'static str = "zed_rope";
    const EDITS_USE_BYTE_OFFSETS: bool = true;

    #[inline(always)]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.replace(at..at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.replace(range, "");
    }

    #[inline(always)]
    fn replace(&mut self, range: Range<usize>, s: &str) {
        self.replace(range, s);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl Rope for xi_rope::Rope {
    const NAME: &'static str = "xi_rope";
    const EDITS_USE_BYTE_OFFSETS: bool = true;

    #[inline(always)]
    fn from_str(s: &str) -> Self {
        s.into()
    }

    #[inline(always)]
    fn insert(&mut self, at: usize, s: &str) {
        self.edit(at..at, s);
    }

    #[inline(always)]
    fn remove(&mut self, range: Range<usize>) {
        self.edit(range, "");
    }

    #[inline(always)]
    fn replace(&mut self, range: Range<usize>, s: &str) {
        self.edit(range, s);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}
