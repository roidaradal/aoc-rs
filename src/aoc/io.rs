use std::fs;

pub fn read_lines(full: bool) -> Vec<String> {
    let path = if full { "data.txt" } else { "test.txt" };
    fs::read_to_string(path)
    .unwrap()
    .lines()
    .map(String::from)
    .collect()
}