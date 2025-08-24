use crate::util;

// an abstract representation of the text reported to the terminal (or some other backend).
/*
* data Doc ann =
    | Union (Doc ann) (Doc ann)
    | Annotated ann (Doc ann)
    // these 3 constructors are quite antithetical to rusts type system.
    | Nesting (Int -> Doc ann)
    | WithPageWidth(fn(u32) -> Document),
    | Column(fn(u32) -> Document),
*/

#[derive(Debug, Clone)]
pub enum Document {
    Fail,
    Empty,
    Char(char),
    Text(String),
    NewLine,
    Over {
        top: Box<Document>,
        bottom: Box<Document>,
    },
    Sequence(Box<Document>, Box<Document>),
    Indent {
        amount: u32,
        document: Box<Document>,
    },
    Style(Style, Box<Document>),
}

#[derive(Debug, Clone)]
pub enum Style {
    Color(Color),
}

#[derive(Debug, Clone)]
pub enum Color {
    Cyan,   // headings
    Red,    // errors
    Yellow, // warnings
    Green,  // idk??
    Blue,   // suggestions
}

// Primatives

pub fn text(message: &str) -> Document {
    Document::Text(message.to_owned())
}

pub fn note(message: &str) -> Document {
    append(vec![color(Color::Blue, text("Note: ")), text(message)])
}

pub fn hint(message: &str) -> Document {
    append(vec![color(Color::Blue, text("Hint: ")), text(message)])
}

pub fn line_break() -> Document {
    Document::NewLine
}

// Combinators

pub fn stack(docs: Vec<Document>) -> Document {
    vertical_append(util::intersperse(docs, text("")))
}

pub fn vertical_append(mut docs: Vec<Document>) -> Document {
    append(util::intersperse(docs, line_break()))
}

pub fn append(docs: Vec<Document>) -> Document {
    docs.into_iter().fold(Document::Empty, |full, doc| {
        Document::Sequence(Box::new(full), Box::new(doc))
    })
}

// style

pub fn color(color: Color, doc: Document) -> Document {
    Document::Style(Style::Color(color), Box::new(doc))
}

impl Document {
    pub fn render(self, width: u32) -> String {
        match self {
            Document::Fail => "".to_owned(),
            Document::Empty => "".to_owned(),
            Document::Char(c) => c.to_string(),
            Document::Text(s) => s.to_string(),
            Document::NewLine => "\n".to_owned(),
            Document::Over { top, bottom } => top.render(width) + "\n" + &bottom.render(width),
            Document::Sequence(first, second) => first.render(width) + &second.render(width),
            Document::Indent { amount, document } => document.render(width - amount),
            Document::Style(style, document) => match style {
                Style::Color(color) => {
                    let color_code = match color {
                        Color::Red => 31,
                        Color::Green => 32,
                        Color::Yellow => 33,
                        Color::Blue => 34,
                        Color::Cyan => 36,
                    };
                    "\x1B[0;".to_owned()
                        + &color_code.to_string()
                        + "m"
                        + &document.render(width)
                        + "\x1B[0m"
                }
            },
        }
    }
}
