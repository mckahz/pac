use nom_locate::LocatedSpan;
use std::fmt;

pub mod canonical;
pub mod optimized;
pub mod source;

pub type Span<'a> = LocatedSpan<&'a str>;

pub type Name = String;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ModuleName(pub Vec<String>);

impl fmt::Display for ModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join("."))
    }
}

#[derive(Debug, Clone)] // TODO: Add top level for further optimizations?
pub enum Qualified<T> {
    Foreign { module: ModuleName, member: T },
    Local(T),
    Kernel(T),
}

impl<T> Qualified<T> {
    pub fn get(&self) -> &T {
        match self {
            Qualified::Foreign { module, member } => member,
            Qualified::Local(member) => member,
            Qualified::Kernel(member) => member,
        }
    }
}

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
    pub const ZERO: Self = Self {
        start: Position::START,
        end: Position::START,
    };

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

impl Position {
    pub const START: Self = Self { line: 1, column: 1 };

    pub fn from_span(span: Span) -> Position {
        Position {
            line: span.location_line() as usize,
            column: span.get_column(),
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }
}

impl Eq for Position {}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.line.partial_cmp(&other.line) {
            Some(std::cmp::Ordering::Equal) => self.column.partial_cmp(&other.column),
            ord => ord,
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.column.cmp(&other.line),
            ord => ord,
        }
    }
}
