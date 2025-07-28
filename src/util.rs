pub fn indent(input: &str) -> String {
    input
        .lines()
        .map(|line| "    ".to_owned() + line)
        .collect::<Vec<String>>()
        .join("\n")
}
