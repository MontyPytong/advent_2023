use core::num;
// day 3 advent calendar
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::format,
    fs::File,
    io::{empty, BufRead, BufReader},
};

#[derive(Debug, Serialize, Deserialize)]
struct Lines {
    line: String,
    number: String,
    start: String,
    end: String,
}

fn main() {
    let print_number = read_file_line_by_line("day_3.txt");
    println!("{:?}", print_number);
}

fn read_file_line_by_line(filepath: &str) -> i32 {
    let mut sum: i32 = 0;
    let symbols = ['*', '@', '&', '/', '%', '$', '=', '-', '+', '#']; // if symbols.contains(&x)

    let mut collection: Vec<String> = Vec::new();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    for lines in reader.lines() {
        for line in lines {
            collection.push(line);
        }
    }
    let mut lines: Vec<String> = Vec::new();
    let position: Vec<(usize, &str, u8, u8)> = Vec::new();

    for line in collection {
        lines.push(line);
    }
    let mut position: Vec<(usize, String, u8, u8)> = Vec::new();

    for i in 0..lines.len() {
        let numeric_chars: String = lines[i]
            .chars()
            .filter(|c| c.is_numeric() || *c == '.' || c.is_ascii_punctuation())
            .collect();

        let numbers: Vec<&str> = numeric_chars
            .split('.')
            .filter(|&s| !s.is_empty())
            .collect();
        //println!("number :{:?}", numbers);
        for index in 0..numbers.len() {
            let value = numbers[index];
            let formatted_string = format!(
                r#"{{"line":"{}", "number":"{}", "start":"{}", "end":"{}"}}"#,
                i,
                numbers[index].to_owned(),
                lines[i].find(value).unwrap() as u8,
                lines[i].find(value).unwrap() as u8 + (value.len() - 1) as u8
            );

            let serialized: Lines = serde_json::from_str(&formatted_string).unwrap();
            println!("{:?}", serialized);
        }
    }

    sum
}
