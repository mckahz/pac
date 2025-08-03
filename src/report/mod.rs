pub mod code;
pub mod document;
pub mod error;
pub mod pretty;

use document::Document;

#[derive(Debug)]
pub struct Report {
    pub title: String,
    pub region: code::Region,
    pub message: Document,
}
