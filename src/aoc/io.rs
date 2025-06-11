use std::fs;

pub fn read_lines(full: bool) -> Vec<String> {
    let path = if full { "data.txt" } else { "test.txt" };
    fs::read_to_string(path)
    .unwrap()
    .lines()
    .map(String::from)
    .collect()
}

pub fn first_line(full: bool) -> String {
    read_lines(full)[0].to_string()
    // lines.first().unwrap().to_string()
    // lines.into_iter().nth(0).unwrap()
}