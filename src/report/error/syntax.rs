use crate::{
    ast::{Located, Position, Region, Span},
    report::{code::Source, document::Document, Report},
};

use nom::error::{ContextError, ParseError};

pub type Error = Located<ErrorKind>;

#[derive(Debug)]
pub enum ErrorKind {
    MissingSemicolon,
    // try remove these
    ExpectedTag,
    Expected(String),
}

impl<'a> ParseError<Span<'a>> for Error {
    fn from_error_kind(input: Span, kind: nom::error::ErrorKind) -> Self {
        use nom::error::ErrorKind::*;

        let pos = Position::from_span(input);

        Error {
            region: Region {
                start: pos.clone(),
                end: pos,
            },
            inner: ErrorKind::ExpectedTag,
        }
    }

    fn append(input: Span, kind: nom::error::ErrorKind, other: Self) -> Self {
        let error = Error::from_error_kind(input, kind);
        Error {
            region: error.region.merge(&other.region),
            inner: other.inner,
        }
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        let pos = Position::from_span(input);
        Error {
            region: Region {
                start: pos.clone(),
                end: pos,
            },
            inner: ErrorKind::Expected(c.to_string()),
        }
    }

    fn or(self, other: Self) -> Self {
        let pos1 = &self.region.end;
        let pos2 = &other.region.end;
        if pos1 > pos2 {
            self
        } else {
            other
        }
    }
}

impl<'a> ContextError<Span<'a>> for Error {
    fn add_context(_input: Span<'a>, _ctx: &'static str, other: Self) -> Self {
        let error = match other.inner {
            ErrorKind::ExpectedTag if _ctx == ";" => ErrorKind::MissingSemicolon,
            _ => other.inner,
        };
        Located {
            region: other.region,
            inner: error,
        }
    }
}

impl Error {
    pub fn to_report(&self, source: Source, file_name: &str) -> Report {
        use crate::report::{code, document::*};
        match self.inner {
            ErrorKind::MissingSemicolon => Report {
                title: "MISSING SEMICOLON".to_owned(),
                path: file_name.to_owned(),
                message: stack(vec![
                    text("I'm not sure when the end of this statement is!"),
                    source.snippet(self.region.clone()),
                    hint("Add a semicolon (;) at the end."),
                ]),
            },
            ErrorKind::ExpectedTag => todo!(),
            ErrorKind::Expected(_) => todo!(),
        }
    }
}
