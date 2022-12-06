use std::{io::{self, BufRead}, collections::VecDeque, path::Path, fs::File};

fn main() {
    if let Ok(lines) = read_lines("./input/six.txt") {
        for line in lines {
            let mut stack: VecDeque<char> = VecDeque::<char>::new();
            let mut counter = 0;
            if let Ok(line_processed) = line {
                for c in line_processed.chars() {
                    if stack.len() > 13 {
                        println!("index: {}", counter);
                        break;
                    }
                    if !stack.contains(&c) {
                        stack.push_back(c);
                        println!("{:?}, pushed: {}, index: {}", stack, c, counter);
                    } else {
                        let popped = pop_until_char(&mut stack, c);
                        stack.push_back(c);
                        println!("{:?}, popped: {:?}, index: {}", stack, popped, counter);
                    }
                    counter += 1;
                }
            }
        }
    }
}

fn pop_until_char(stack: &mut VecDeque<char>, c: char) {
    while stack.len() > 0 {
        if stack.pop_front() == Some(c) {
            break;
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
