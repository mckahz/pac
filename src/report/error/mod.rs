pub mod syntax;
pub mod tipe;

use crate::report::{code::Source, Report};

pub enum Error {
    Syntax(syntax::Error),
}

impl Error {
    pub fn to_report(&self, source: Source, file_name: &str) -> Report {
        match self {
            Error::Syntax(e) => e.to_report(source, file_name),
        }
    }
}
