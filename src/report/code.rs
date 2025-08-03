pub struct Source(Vec<(u32, String)>);

impl Source {
    pub fn len(&self) -> usize {
        self.0.iter().map(|(_, line)| line.len()).sum()
    }
}

#[derive(Debug)]
pub struct Region {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug)]
pub struct Position {
    pub row: u16,
    pub col: u16,
}

fn from_string(string: &str) -> Source {
    Source(
        string
            .split("\n")
            .enumerate()
            .map(|(n, line)| (n as u32, line.to_owned()))
            .collect(),
    )
}
