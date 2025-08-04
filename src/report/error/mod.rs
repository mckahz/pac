pub mod syntax;

use crate::report::{code::Source, Report};

pub enum Error {
    Syntax(syntax::Error),
}

pub fn to_report(error: Error, source: Source) -> Report {
    match error {
        Error::Syntax(e) => syntax::to_report(e, source),
    }
}
