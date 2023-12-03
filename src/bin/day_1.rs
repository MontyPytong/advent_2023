use core::num;
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
    let number = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum: i32 = 0;
    let mut count: i32 = 0;
    let mut add_count: i32 = 0;

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let _ = for line in reader.lines() {
        println!("[ {} ]+++++++++++++++++++++++++++++++++", count);
        count += 1;
        for char in line {
            let mut i: u32 = 0;
            let mut value: String = String::from(char.clone());
            println!("original : {:?}", char);

            for n in number {
                let g_index: Option<usize> = value.find("oneight");

                if value.as_str().contains("oneight") {
                    value = value.replace("oneight", 18.to_string().as_str());
                }
                if value.as_str().contains("twone") {
                    value = value.replace("twone", 21.to_string().as_str());
                }

                if value.as_str().contains("eightwo") {
                    value = value.replace("eightwo", 82.to_string().as_str());
                }

                if value.as_str().contains(n) {
                    value = value.replace(n, i.to_string().as_str());
                };
                i += 1;
            }
            println!("transformed : {:?}", value);

            let v: Vec<&str> = value.matches(char::is_numeric).collect();

            if v.len() < 2 {
                let first = v.first().unwrap().parse::<i32>().unwrap();
                let last = v.first().unwrap().parse::<i32>().unwrap();
                let combine = format!("{}{}", first, last).parse::<i32>().unwrap();
                println!("mai avem caz dasta {:?}", combine);
                sum += combine;
                add_count += 1;
            } else {
                let first = v.first().unwrap().parse::<i32>().unwrap();
                let last = v.last().unwrap().parse::<i32>().unwrap();
                let combine = format!("{}{}", first, last).parse::<i32>().unwrap();
                println!("{:?}", combine);
                sum += combine;
                add_count += 1;
            }
            println!("total : {:?}", sum);
        }
    };
    return sum;
}
