use std::{io::{self, BufRead}, path::Path, fs::File};

fn main() {
    if let Ok(lines) = read_lines("./input/two.txt") {
        let mut sum_vec : Vec<u128> = vec![];
        for line in lines {
            if let Ok(line_processed) = line {
               let split = line_processed.split_ascii_whitespace();
                let vec: Vec<&str> = FromIterator::from_iter(split);
                let opp_move = vec.get(0).unwrap().chars().nth(0).unwrap();
                let strat_move = vec.get(1).unwrap().chars().nth(0).unwrap();
                let score = round_score_two(opp_move, strat_move);
                sum_vec.push(score);
            }
        }
        println!("Sum: {}", sum_vec.iter().sum::<u128>());
    }
}

fn round_score(opponent_move: char, strategy_move: char) -> u128 {
    let move_score = match strategy_move {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };
    let round_outcome_score = match opponent_move {
        'A' => match strategy_move {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => 0,
        },
        'B' => match strategy_move {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => 0,

        },
        'C' => match strategy_move {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => 0,
        },
        _ => 0,
    };
    move_score + round_outcome_score
}

fn round_score_two(opponent_move: char, strategy_move: char) -> u128 {
    let move_score = match strategy_move {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    };
    let round_outcome_score = match opponent_move {
        'A' => match strategy_move {
            'X' => 3,
            'Y' => 1,
            'Z' => 2,
            _ => 0,
        },
        'B' => match strategy_move {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,

        },
        'C' => match strategy_move {
            'X' => 2,
            'Y' => 3,
            'Z' => 1,
            _ => 0,
        },
        _ => 0,
    };
    println!("{} {} {}", move_score, round_outcome_score, move_score + round_outcome_score);
    move_score + round_outcome_score
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

