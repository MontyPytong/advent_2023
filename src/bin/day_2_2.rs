// day 2 advent calendar
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let print_number = read_file_line_by_line("day_2.txt");
    println!("{:?}", print_number);
}

fn read_file_line_by_line(filepath: &str) -> i32 {
    println!("12 red , 13 green , 14 blue ");
    let mut power: i32 = 0;
    let mut sum: i32 = 0;

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let _ = for line in reader.lines() {
        for option in line {
            let mut red: i32 = 0;
            let mut blue: i32 = 0;
            let mut green: i32 = 0;
            let values: Vec<&str> = option.split(':').to_owned().collect();
            let sets: Vec<_> = values[1].split(';').collect();

            for set in sets {
                let s: Vec<_> = set.split(',').collect();
                let mut storage: HashMap<&str, &str> = HashMap::new();

                for ball in s {
                    let sa: Vec<_> = ball.split(',').collect();
                    for key_value in sa {
                        let v: Vec<&str> = key_value.split(" ").collect();
                        storage.insert(v[2], v[1]);
                    }
                }

                for (color, value) in storage {
                    match color {
                        "red" => {
                            let value = value.parse::<i32>().unwrap();
                            if value > red {
                                red = value;
                            }
                        }
                        "blue" => {
                            let value = value.parse::<i32>().unwrap();
                            if value > blue {
                                blue = value;
                            }
                        }
                        "green" => {
                            let value = value.parse::<i32>().unwrap();
                            if value > green {
                                green = value;
                            }
                        }
                        &_ => todo!(),
                    }
                }
            }

            power = red * blue * green;
            sum += power;

            red = 0;
            blue = 0;
            green = 0;
        }
    };

    sum
}

//fn sets_processing(String) ->
