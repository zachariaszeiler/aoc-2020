use std::io::{stdin, BufRead};

pub fn solve() {
    let max = stdin().lock().lines().map(|line| {
        let line = line.unwrap();

        let binary = line.chars().map(|c| {
            if c == 'B' || c == 'R' { '1' } else { '0' }
        }).collect::<String>();

        let row = u32::from_str_radix(&binary[0..7], 2).unwrap();
        let seat = u32::from_str_radix(&binary[7..10], 2).unwrap();
        let id = row * 8 + seat;

        id

    }).max().unwrap();

    println!("Max id is {}.", max);
}
