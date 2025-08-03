use crate::report::{
    code::{self, Position, Region, Source},
    document::Document,
    Report,
};

use nom::error::*;
use nom_supreme::error::*;

pub type Error<'a> = ErrorTree<&'a str>;

pub fn to_report(error: Error, source: Source) -> Report {
    Report {
        title: "PARSE ERROR".to_owned(),
        region: todo!(),
        message: todo!(),
    }
}
