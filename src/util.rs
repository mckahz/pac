pub fn indent(input: &str) -> String {
    input
        .lines()
        .map(|line| "    ".to_owned() + line)
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn to_camel_case(ident: &str) -> String {
    ident
        .chars()
        .fold(("".to_owned(), false), |(camel_case, next_word), c| {
            let upper_c = c.to_uppercase().next().unwrap().to_string();
            let lower_c = c.to_string();
            let string_char: &str = if next_word { &upper_c } else { &lower_c };
            match c {
                '_' => (camel_case, true),
                '?' => (camel_case + "__hmmm", true),
                _ => (camel_case + string_char, false),
            }
        })
        .0
}

pub fn intersperse<T>(mut vec: Vec<T>, sep: T) -> Vec<T>
where
    T: Clone,
{
    vec.reverse();
    let Some(first) = vec.pop() else {
        return vec![];
    };
    vec.into_iter().rev().fold(vec![first], |mut full, x| {
        full.append(&mut vec![sep.clone(), x]);
        full
    })
}
