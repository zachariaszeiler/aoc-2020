use std::io::{stdin, BufRead};
use std::error::Error;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve() {
    let mut answers = HashSet::new();
    let mut sum = 0u32;
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            sum += answers.len() as u32;
            answers.clear();
        } else {
            answers.extend(line.chars());
        }
    }

    println!("{}", sum);
}

pub fn solve_part2() {
    let mut group_answers = "abcdefghijklmnopqrstuvwxyz".chars().collect::<HashSet<char>>();
    let mut sum = 0u32;
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            sum += group_answers.len() as u32;
            group_answers.extend("abcdefghijklmnopqrstuvwxyz".chars());
        } else {
            let user_answers = line.chars().collect::<HashSet<char>>();
            group_answers = group_answers.intersection(&user_answers).copied().collect::<HashSet<char>>();
        }
    }

    println!("{}", sum);
}