// Extended prelude traits for the CURSED language

// Raw pointer extension trait
pub trait RawPtrExt {
    fn as_usize(&self) -> usize;
}

impl<T> RawPtrExt for *const T {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl<T> RawPtrExt for *mut T {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

// Vector string join extension
pub trait VecStrJoinExt {
    fn join(&self, separator: &str) -> String;
}

impl VecStrJoinExt for Vec<String> {
    fn join(&self, separator: &str) -> String {
        self.iter().fold(String::new(), |acc, s| {
            if acc.is_empty() {
                s.clone()
            } else {
                acc + separator + s
            }
        })
    }
}

// String chars extension
pub trait StrCharsExt {
    fn chars(&self) -> std::str::Chars<'_>;
}

impl StrCharsExt for str {
    fn chars(&self) -> std::str::Chars<'_> {
        <str>::chars(self)
    }
}

// Slice extension
pub trait SliceExt<T> {
    fn into_vec(self) -> Vec<T> where T: Clone;
}

impl<T> SliceExt<T> for &[T] {
    fn into_vec(self) -> Vec<T> where T: Clone {
        self.to_vec()
    }
}
