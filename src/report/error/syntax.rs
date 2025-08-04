use crate::{
    ast::{Located, Position, Span},
    report::{
        code::{self, Source},
        document::Document,
        Report,
    },
};

use nom::error::{ContextError, ParseError};

#[derive(Debug)]
pub enum ErrorTree {
    Base(Error, Position),
    Stack(Error, Position, Box<ErrorTree>),
    Alt(Vec<ErrorTree>),
}

#[derive(Debug)]
pub enum Error {
    MissingSemicolon,
    String(String),
}

impl Error {
    fn from_error_kind(kind: nom::error::ErrorKind) -> Self {
        Error::String(format!("{:?}", kind).to_owned())
    }
}

impl<'a> ParseError<Span<'a>> for ErrorTree {
    fn from_error_kind(input: Span, kind: nom::error::ErrorKind) -> Self {
        use nom::error::ErrorKind::*;

        let error = Error::from_error_kind(kind);

        ErrorTree::Base(
            error,
            Position {
                line: input.location_line() as usize,
                column: input.get_column(),
            },
        )
    }

    fn append(input: Span, kind: nom::error::ErrorKind, other: Self) -> Self {
        let error = Error::from_error_kind(kind);
        ErrorTree::Stack(
            error,
            Position {
                line: input.location_line() as usize,
                column: input.get_column(),
            },
            Box::new(other),
        )
    }
}

impl<'a> ContextError<Span<'a>> for ErrorTree {
    fn add_context(_input: Span<'a>, _ctx: &'static str, other: Self) -> Self {
        other
    }
}

pub fn to_report(error: Error, source: Source) -> Report {
    Report {
        title: "PARSE ERROR".to_owned(),
        region: todo!(),
        message: todo!(),
    }
}
