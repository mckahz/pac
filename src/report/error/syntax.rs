use crate::{
    ast::Span,
    report::{
        code::{self, Position, Region, Source},
        document::Document,
        Report,
    },
};

use nom;

pub type Error<'a> = nom::error::Error<Span<'a>>;

pub fn to_report(error: Error, source: Source) -> Report {
    Report {
        title: "PARSE ERROR".to_owned(),
        region: todo!(),
        message: todo!(),
    }
}
