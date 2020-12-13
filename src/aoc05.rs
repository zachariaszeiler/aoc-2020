use std::io::{stdin, BufRead, StdinLock};

// O(n)
pub fn solve() {
    let max = parse_ids(stdin().lock())
        .max().unwrap();
    println!("Max id is {}.", max);
}

// O(n)
fn parse_ids(stdin: StdinLock) -> impl Iterator<Item=u32> + '_ {
    stdin.lines().map(|line| {
        let line = line.unwrap();

        let binary = line.chars().map(|c| {
            if c == 'B' || c == 'R' { '1' } else { '0' }
        }).collect::<String>();

        let row = u32::from_str_radix(&binary[0..7], 2).unwrap();
        let seat = u32::from_str_radix(&binary[7..10], 2).unwrap();
        let id = row * 8 + seat;

        id
    })
}

// O(n log n)
pub fn solve_part2() {
    let mut ids = parse_ids(stdin().lock()).collect::<Vec<u32>>();

    ids.sort();

    let mut prev_seat = u32::MAX - 2;

    for cur_seat in ids {
        if prev_seat + 2 == cur_seat {
             println!("My seat is {}.", prev_seat + 1)
        }
        prev_seat = cur_seat;
    }
}
