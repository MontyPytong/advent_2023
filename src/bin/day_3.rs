// day 3 advent calendar
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
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
    let mut all_lines: Vec<[String; 4]> = Vec::new();

    let mut collection: Vec<String> = Vec::new();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    for lines in reader.lines() {
        for line in lines {
            collection.push(line);
        }
    }
    let mut lines: Vec<String> = Vec::new();

    for line in collection {
        lines.push(line);
    }

    for i in 0..lines.len() {
        // let numeric_chars: String = lines[i]
        //     .chars()
        //     .filter(|c| c.is_digit(10) || *c == '.' || c.is_ascii_punctuation())
        //     .collect();

        // let numbers: Vec<&str> = numeric_chars
        //     .split('.')
        //     .filter(|&s| !s.is_empty())
        //     .collect();
        let numeric_chars: String = lines[i]
            .chars()
            .filter(|c| c.is_digit(10) || *c == '.' || *c == '*')
            .collect();
        let punctuation: String = lines[i]
            .chars()
            .filter(|c| c.is_ascii_punctuation())
            .collect();

        let combined_numbers: Vec<&str> = numeric_chars
            .split(|c| c == '.' || c == '*')
            .filter(|&s| !s.is_empty())
            .collect();
        let combined_punctuation: Vec<&str> =
            punctuation.split('.').filter(|&s| !s.is_empty()).collect();

        let numbers: Vec<&str> = combined_numbers
            .into_iter()
            .chain(combined_punctuation)
            .collect();

        println!(" numbers: {:?}", numbers);

        // for index in 0..numbers.len() {
        //     let value = numbers[index];

        //     all_lines.push([
        //         i.to_string(),
        //         numbers[index].to_owned(),
        //         lines[i].find(value).unwrap().to_string(),
        //         (lines[i].find(value).unwrap() as u8 + (value.len() - 1) as u8).to_string(),
        //     ]);
        // }
        for index in 0..numbers.len() {
            let value = &numbers[index];
            let mut start_index = 0;

            while let Some(found_index) = lines[i][start_index..].find(value) {
                // Calculate the start and end indices relative to the entire string
                let absolute_start_index = start_index + found_index;
                let absolute_end_index = absolute_start_index + value.len();

                // Check if the found substring is a standalone number
                let is_standalone_number = absolute_start_index == 0
                    || !lines[i][absolute_start_index - 1..absolute_start_index]
                        .chars()
                        .all(char::is_numeric)
                        && (absolute_end_index == lines[i].len()
                            || !lines[i][absolute_end_index..absolute_end_index + 1]
                                .chars()
                                .all(char::is_numeric));

                if is_standalone_number {
                    let new_line = [
                        i.to_string(),
                        numbers[index].to_owned(),
                        absolute_start_index.to_string(),
                        if numbers[index].to_owned().len() < 2 {
                            absolute_start_index.to_string()
                        } else {
                            (absolute_end_index - 1).to_string()
                        },
                    ];
                    if !all_lines.contains(&new_line) {
                        all_lines.push(new_line);
                    }
                }

                // Update the start_index to search for the next occurrence
                start_index = absolute_end_index;
            }
        }
    }
    println!("ALL LINES : {:?}", all_lines);

    for (k, v) in all_lines.iter().enumerate() {
        if v[0].eq("0") {
            println!("index {:?}, line {:?}", k, v);

            if v[1].contains(char::is_numeric) {
                let check: bool = contains_punctuation(&v[1]);

                if check {
                    let cleaned_string = remove_non_numeric(&v[1]).parse::<i32>().unwrap();
                    sum += cleaned_string;
                    println!("1.---------> punct added to sum {cleaned_string}");
                } else {
                    let range_start = if v[2].parse::<i32>().unwrap() == 0 {
                        v[2].parse::<i32>().unwrap()
                    } else {
                        v[2].parse::<i32>().unwrap() - 1
                    };

                    let range_end = if v[3].parse::<i32>().unwrap() != 140 {
                        v[3].parse::<i32>().unwrap() + 1
                    } else {
                        v[3].parse::<i32>().unwrap()
                    };
                    let range = range_start..=range_end;

                    let matching_lines: Vec<_> = all_lines
                        .iter()
                        .filter(|&line| {
                            line[0].parse::<usize>().ok()
                                == Some(v[0].parse::<usize>().unwrap() + 1)
                        })
                        .collect();

                    for next_line in matching_lines {
                        if symbols.contains(&next_line[1].chars().nth(0).unwrap()) {
                            let next_start = &next_line[2].parse::<i32>().unwrap();
                            let next_end = &next_line[3].parse::<i32>().unwrap();
                            // println!(
                            //     "next_line : {:?}, range {:?}{:?}",
                            //     next_line, next_start, next_end
                            // );
                            //println!("{:?}", range.contains(&next_start));
                            // println!("{:?}", range.contains(&next_end));
                            if range.contains(&next_start) || range.contains(&next_end) {
                                let add_to_sum = v[1].parse::<i32>().unwrap();
                                sum += add_to_sum;
                                println!(
                                    "\x1b[93mSUM\x1b[0m : 1.---------> added to sum {add_to_sum}"
                                );
                            }
                        }
                    }
                }
            }
        } else {
            println!("index {:?}, line {:?}", k, v);
            if v[1].contains(char::is_numeric) {
                let check: bool = contains_punctuation(&v[1]);

                if check {
                    let cleaned_string = remove_non_numeric(&v[1]).parse::<i32>().unwrap();
                    sum += cleaned_string;
                    println!(
                        "\x1b[93mSUM\x1b[0m :2.---------> punct added to sum {cleaned_string}"
                    );
                } else {
                    let range_start = if v[2].parse::<i32>().unwrap() == 0 {
                        v[2].parse::<i32>().unwrap()
                    } else {
                        v[2].parse::<i32>().unwrap() - 1
                    };

                    let range_end = if v[3].parse::<i32>().unwrap() != 139 {
                        v[3].parse::<i32>().unwrap() + 1
                    } else {
                        v[3].parse::<i32>().unwrap()
                    };
                    let range = range_start..=range_end;
                    println!("range{:?}", range);

                    ////need to check if LEFT AND RIGHT on current line to number there is a sign and add it to number ---------------------------------------/
                    let range_start_usize = range_start as usize;
                    let range_end_usize = range_end as usize;
                    let line_to_scan = &v[0].parse::<usize>();
                    let left_right: &String = &lines[line_to_scan.clone().unwrap()];
                    let hello: &str = &left_right[range_start_usize..=range_end_usize];

                    if contains_punctuation_2(hello) {
                        let add_to_sum = v[1].parse::<i32>().unwrap();
                        sum += add_to_sum;
                        println!("\x1b[93mSUM\x1b[0m :left and right .---------> punct added to sum {add_to_sum}");
                    }
                    //----------------------------------------------------------------------------------------------------/

                    ////need to check if LEFT AND RIGHT on PRELINE line to number there is a sign and add it to number ---------------------------------------/

                    let pre_line_to_scan = &v[0].parse::<usize>().unwrap() - 1;
                    let pre_left_right: &String = &lines[pre_line_to_scan.clone()];
                    let pre_hello: &str = &pre_left_right[range_start_usize..=range_end_usize];

                    if contains_punctuation_2(pre_hello) {
                        let add_to_sum = v[1].parse::<i32>().unwrap();
                        sum += add_to_sum;
                        println!("\x1b[93mSUM\x1b[0m :PRELINE left and right .---------> punct added to sum {add_to_sum}");
                    }
                    //----------------------------------------------------------------------------------------------------/

                    ////need to check if LEFT AND RIGHT on nextline line to number there is a sign and add it to number ---------------------------------------/

                    let next_line_to_scan = if v[0].parse::<usize>().unwrap() == 139 {
                        v[0].parse::<usize>().unwrap()
                    } else {
                        v[0].parse::<usize>().unwrap() + 1
                    };
                    let next_left_right: &String = &lines[next_line_to_scan];
                    let next_hello: &str = &next_left_right[range_start_usize..=range_end_usize];

                    if contains_punctuation_2(next_hello) {
                        let add_to_sum = v[1].parse::<i32>().unwrap();
                        sum += add_to_sum;
                        println!("\x1b[93mSUM\x1b[0m :NEXTLINE left and right .---------> punct added to sum {add_to_sum}");
                    }
                    //----------------------------------------------------------------------------------------------------/
                }
            } else if symbols.contains(&v[1].chars().nth(0).unwrap()) {
                //println!("we found a char");
                continue;
            }
        }
    }
    //println!("{sum}");
    sum
}

fn remove_non_numeric(input: &str) -> String {
    input.chars().filter(|c| c.is_digit(10)).collect()
}

fn contains_punctuation(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_punctuation())
}

fn contains_punctuation_2(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_punctuation() && c != '.')
}
