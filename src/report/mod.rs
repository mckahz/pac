pub mod code;
pub mod document;
pub mod error;
pub mod pretty;

use code::Source;
use document::*;

#[derive(Debug)]
pub struct Report {
    pub title: String,
    pub path: String,
    pub message: Document,
}

impl Report {
    pub fn render(self, width: u32) -> String {
        stack(vec![
            color(
                Color::Cyan,
                append(vec![
                    text("-- "),
                    text(&self.title),
                    text(" "),
                    text(&"-".repeat(width as usize - 8 - self.title.len() - self.path.len())),
                    text(" "),
                    text(&self.path),
                    text(" --"),
                ]),
            ),
            self.message,
        ])
        .render(width)
    }
}
