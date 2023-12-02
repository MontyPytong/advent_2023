use std::{
    fmt::format,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    let print_number = read_file_line_by_line("advent_1.txt");
    println!("{:?}", print_number);
}

fn read_file_line_by_line(filepath: &str) -> i32 {
    let mut add: i32 = 0;
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let _ = for line in reader.lines() {
        for char in line {
            let v: Vec<&str> = char.matches(char::is_numeric).collect();
            //println!("v : {:?}, v.len : {:?}", v, v.len());

            if v.len() < 2 {
                let first = v.first().unwrap().parse::<i32>().unwrap();
                let last = v.first().unwrap().parse::<i32>().unwrap();
                let sum = format!("{}{}", first, last).parse::<i32>().unwrap();
                add += sum;
            } else {
                let first = v.first().unwrap().parse::<i32>().unwrap();
                let last = v.last().unwrap().parse::<i32>().unwrap();
                let sum = format!("{}{}", first, last).parse::<i32>().unwrap();
                add += sum;
            }
        }
    };
    return add;
}
