use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn get_blossum(path: &str) -> Vec<Vec<i32>> {
    let f = BufReader::new(File::open(path).unwrap());

    f.lines()
        .map(|l| {
            l.unwrap()
                .split(char::is_whitespace)
                .filter_map(|number| number.parse::<i32>().ok())
                .collect()
        })
        .collect()
}
