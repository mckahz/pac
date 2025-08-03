// an abstract representation of the text reported to the terminal (or some other backend).
/*
* data Doc ann =
    | Union (Doc ann) (Doc ann)
    | Nesting (Int -> Doc ann)
    | Annotated ann (Doc ann)
*/
#[derive(Debug)]
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
    WithPageWidth(fn(u32) -> Document),
    Column(fn(u32) -> Document),
}
