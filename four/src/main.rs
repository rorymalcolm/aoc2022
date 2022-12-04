use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    if let Ok(lines) = read_lines("./input/four.txt") {
        let mut entirely_contains_count = 0;
        for line in lines {
            if let Ok(line_processed) = line {
                let split = line_processed.split(",");
                let mut vec: Vec<HashSet<u128>> = vec![];
                for s in split {
                    vec.push(calculate_jobs(s.to_string()));
                }
                let intersect = vec
                    .get(0)
                    .unwrap()
                    .intersection(vec.get(1).unwrap())
                    .collect::<HashSet<_>>();
                if intersect.len() > 0 {
                    entirely_contains_count += 1;
                }
            }
        }
        println!("{}", entirely_contains_count);
    }
}

fn calculate_jobs(job_text: String) -> HashSet<u128> {
    let begin_and_end = job_text.split("-").collect::<Vec<&str>>();
    let begin = begin_and_end[0].parse::<u128>().unwrap();
    let end = begin_and_end[1].parse::<u128>().unwrap();
    let mut jobs = HashSet::new();
    for i in begin..end + 1 {
        jobs.insert(i);
    }
    jobs.clone()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
