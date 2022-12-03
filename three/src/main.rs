use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    if let Ok(lines) = read_lines("./input/three.txt") {
        let mut sum_vec: Vec<u128> = vec![];
        let mut line_count = 0;
        let mut line_vec: Vec<String> = vec![];
        for line in lines {
            if let Ok(line_processed) = line {
                line_count += 1;
                line_vec.push(line_processed);
                if line_count > 2 {
                    let common_char = find_common_char_over_three_lines(
                        line_vec.get(0).unwrap().to_string(),
                        line_vec.get(1).unwrap().to_string(),
                        line_vec.get(2).unwrap().to_string(),
                    );
                    sum_vec.push(ascii_value(common_char.chars().nth(0).unwrap()));
                    line_count = 0;
                    line_vec.clear();
                }
            }
        }
        println!("First task: {}", sum_vec.iter().sum::<u128>());
    }
}

fn find_common_char(line_half_one: String, line_half_two: String) -> String {
    let mut common_chars = String::new();
    for (_i, c) in line_half_one.chars().enumerate() {
        if line_half_two.chars().find(|x| x.clone() == c).is_some() {
            common_chars.push(c);
        }
    }
    common_chars
}

fn find_common_char_over_three_lines(
    line_one: String,
    line_two: String,
    line_three: String,
) -> String {
    let mut common_chars = String::new();
    for (_i, c) in line_one.chars().enumerate() {
        if line_two.chars().find(|x| x.clone() == c).is_some()
            && line_three.chars().find(|x| x.clone() == c).is_some()
        {
            common_chars.push(c);
        }
    }
    common_chars
}

fn ascii_value(c: char) -> u128 {
    if c.is_uppercase() {
        c as u128 - 38
    } else {
        c as u128 - 96
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
