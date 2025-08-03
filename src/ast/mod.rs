use nom_locate::LocatedSpan;

pub mod core;
pub mod source;

pub type Span<'a> = LocatedSpan<&'a str>;

pub type Name = String;

#[derive(Debug, Clone)]
pub struct Located<T> {
    pub region: Region,
    pub inner: T,
}

impl<T> Located<T> {
    pub fn map<U, F>(&self, function: F) -> Located<U>
    where
        F: FnOnce(T) -> U,
        T: Clone,
    {
        Located {
            region: self.region.clone(),
            inner: function(self.inner.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    pub start: Position,
    pub end: Position,
}

impl Region {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            start: self.start.clone(),
            end: other.end.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}
