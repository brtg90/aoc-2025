use std::fs;

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_input(filename)
        .lines()
        .filter_map(|x| {
            if x.is_empty() {
                None
            }
            else {
                Some(x.to_string())
            }
        })
        .collect()
}