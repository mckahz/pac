use crate::ast::Region;

use super::document::*;

pub struct Source<'a> {
    code: &'a str,
}

fn left_pad(string: &str, width: u32, c: char) -> String {
    c.to_string().repeat(width as usize - string.len()) + string
}

impl<'a> Source<'a> {
    pub fn new(file: &'a str) -> Self {
        Self { code: file }
    }

    pub fn snippet(&self, region: Region) -> Document {
        vertical_append(
            self.code
                .to_owned()
                .split("\n")
                .enumerate()
                .skip(region.start.line - 1)
                .take(1 + region.end.line - region.start.line)
                .map(|(i, line)| {
                    append(vec![
                        text(&(left_pad(&(i + 1).to_string(), 4, ' ') + " |")),
                        color(Color::Red, text("> ")),
                        text(line),
                    ])
                })
                .collect::<Vec<Document>>(),
        )
    }
}
