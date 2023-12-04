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
    let mut sum: i32 = 0;

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let _ = for line in reader.lines() {
        for option in line {
            let mut red: i32 = 0;
            let mut blue: i32 = 0;
            let mut green: i32 = 0;
            let values: Vec<&str> = option.split(":").to_owned().collect();
            let game: Vec<_> = values[0].split(" ").collect();
            let sets: Vec<_> = values[1].split(";").collect();
            // println!(
            //     " ________________________________________ GAME [ {:#?} ]",
            //     game[1]
            // );
            // println!(" SET [ {:#?} ]", sets);

            let mut check: Vec<bool> = Vec::new();

            for set in sets {
                let s: Vec<_> = set.split(",").collect();
                let mut storage: HashMap<&str, &str> = HashMap::new();

                for ball in s {
                    let sa: Vec<_> = ball.split(",").collect();
                    for key_value in sa {
                        let v: Vec<&str> = key_value.split(" ").collect();
                        storage.insert(v[1], v[2]);
                    }
                }
                for (value, color) in storage {
                    match color {
                        "red" => red = value.parse::<i32>().unwrap(),
                        "blue" => blue = value.parse::<i32>().unwrap(),
                        "green" => green = value.parse::<i32>().unwrap(),
                        &_ => todo!(),
                    }
                }
                if red <= 12 && green <= 13 && blue <= 14 {
                    check.push(true);
                } else {
                    check.push(false);
                }

                red = 0;
                blue = 0;
                green = 0;
            }
            if !check.contains(&false) {
                let games: i32 = game[1].to_string().parse::<i32>().unwrap();
                //println!("sum was : {sum}");
                sum += games;
                //println!("sum is : {sum} added {games}");
            }
            //println!("check = {:?}", check);
            //println!("SUM = {:?}", sum);
        }
    };
    sum
}

//fn sets_processing(String) ->
