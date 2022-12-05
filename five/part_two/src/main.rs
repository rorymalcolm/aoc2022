use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    if let Ok(lines) = read_lines("./input/five.txt") {
        let mut newline_reached = false;
        let mut stack_vec: HashMap<usize, Vec<String>> = HashMap::new();
        let re = Regex::new(r".*\[").unwrap();
        let regex_extract = Regex::new(r"\[(.*)\]").unwrap();
        let move_step = Regex::new(r"move.*").unwrap();
        let extract_move_components = Regex::new(r"move (.*) from (.*) to (.*)").unwrap();
        for line in lines {
            if let Ok(line_processed) = line {
                if !newline_reached {
                    let split_iter = split_every_n(line_processed.as_str(), 4).into_iter();
                    let trimmed = split_iter.map(|x| x.trim());
                    let captured = trimmed.map(|x| regex_extract.captures(x));
                    let vals = captured
                        .map(|x| return_value_from_capture_or_none(x))
                        .enumerate();
                    for (i, v) in vals {
                        if let Some(v) = v {
                            if stack_vec.contains_key(&i) {
                                stack_vec.get_mut(&i).unwrap().push(v);
                            } else {
                                stack_vec.insert(i, vec![v]);
                            }
                        }
                    }
                }
                if !newline_reached && !re.is_match(&line_processed) {
                    stack_vec = flip_stack_order(stack_vec);
                    newline_reached = true;
                }
                if newline_reached && move_step.is_match(&line_processed) {
                    format_stack_vec(stack_vec.clone());
                    let caps = extract_move_components
                        .captures(line_processed.as_str())
                        .unwrap();
                    caps.iter().for_each(|x| match x {
                        Some(x) => println!("x: {}", x.as_str()),
                        None => {}
                    });
                    let source = parse_to_u128_nth_of_capture(&caps, 2);
                    let target = parse_to_u128_nth_of_capture(&caps, 3);
                    let mut count = parse_to_u128_nth_of_capture(&caps, 1);
                    let mut slab = vec![];
                    while count > 0 {
                        let val = stack_vec
                            .get_mut(&(source as usize - 1))
                            .unwrap()
                            .pop()
                            .unwrap();
                        slab.push(val);
                        count -= 1;
                    }
                    for val in slab.into_iter().rev() {
                        stack_vec.get_mut(&(target as usize - 1)).unwrap().push(val);
                    }
                }
            }
        }
        println!("Final stack: ");
        let cloned = stack_vec.clone();
        format_stack_vec(stack_vec);
        print!("\n\n\n\n\n");
        print_top_of_each_in_order(cloned);
    }
}

fn return_value_from_capture_or_none(capture: Option<regex::Captures>) -> Option<String> {
    match capture {
        Some(c) => Some(c.get(1).unwrap().as_str().to_string()),
        None => None,
    }
}

fn flip_stack_order(stack_vec: HashMap<usize, Vec<String>>) -> HashMap<usize, Vec<String>> {
    let mut new_stack_vec: HashMap<usize, Vec<String>> = HashMap::new();
    for (i, v) in stack_vec {
        let mut new_vec = v.clone();
        new_vec.reverse();
        new_stack_vec.insert(i, new_vec);
    }
    new_stack_vec
}

fn print_top_of_each_in_order(stack_vec: HashMap<usize, Vec<String>>) {
    let mut stack_vec = stack_vec;
    let mut stack_vec_formatted = String::new();
    for i in 0..stack_vec.len() {
        let stack = stack_vec.get_mut(&i).unwrap();
        stack_vec_formatted.push_str(&format!("{:?}", stack.last().unwrap().as_str()));
    }
    println!("{}", stack_vec_formatted);
}

fn parse_to_u128_nth_of_capture(capture: &regex::Captures, n: usize) -> u128 {
    capture
        .iter()
        .nth(n)
        .unwrap()
        .unwrap()
        .as_str()
        .trim()
        .parse::<u128>()
        .unwrap()
}

fn split_every_n(s: &str, n: usize) -> Vec<&str> {
    (0..s.len())
        .step_by(n)
        .map(|i| &s[i..(i + n).min(s.len())])
        .collect()
}

fn format_stack_vec(stack_vec: HashMap<usize, Vec<String>>) {
    let mut stack_vec = stack_vec;
    let mut stack_vec_formatted = String::new();
    for i in 0..stack_vec.len() {
        let stack = stack_vec.get_mut(&i).unwrap();
        stack_vec_formatted.push_str(&format!("{}: ", i + 1));
        for val in stack {
            stack_vec_formatted.push_str(&format!("{} ", val));
        }
        stack_vec_formatted.push_str(&format!(" "));
    }
    println!("{}", stack_vec_formatted);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
