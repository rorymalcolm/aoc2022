use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut calories: HashMap<u128, u128> = HashMap::<u128, u128>::new();
    let mut counter = 0;
    if let Ok(lines) = read_lines("./input/one.txt") {
        for line in lines {
            if let Ok(line_processed) = line {
                if line_processed != "" && line_processed.parse::<u128>().is_ok() {
                    if calories.contains_key(&counter) {
                        calories.insert(
                            counter,
                            calories.get(&counter).unwrap()
                                + line_processed.parse::<u128>().unwrap(),
                        );
                    } else {
                        calories.insert(counter, line_processed.parse::<u128>().unwrap());
                    }
                } else if line_processed == "" {
                    counter += 1;
                }
            }
        }
    }
    let max_val = calories.values().max().unwrap();
    let top_three = calories.values().into_iter();
    let mut vec: Vec<&u128> = FromIterator::from_iter(top_three.map(|x| x));
    vec.sort();
    vec.reverse();
    if vec.len() > 3 {
        vec.truncate(3);
    }
    let mut sum = 0;
    for i in vec {
        sum += i;
    }
    println!("Top three: {:?}", sum);
    println!("Max value: {}", max_val);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
