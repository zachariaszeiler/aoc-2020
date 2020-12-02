use std::io;
use std::io::BufRead;

pub(crate) fn solve() {
   let stdin = io::stdin();

   let mut global_count = 0u32;

   for line in stdin.lock().lines() {
      let line = line.unwrap();
      let mut parts = line.split(&['-', ':', ' '][..]);
      let from = parts.next().unwrap().parse::<u32>().unwrap();
      let to = parts.next().unwrap().parse::<u32>().unwrap();
      // first (and only) letter from string
      let letter = parts.next().unwrap().chars().next().unwrap();
      parts.next(); // drop whitespace
      let password = parts.next().unwrap();

      // count occurrences of letter
      let count = password.chars().fold(0u32, |a, c|{ if c == letter {a + 1} else { a } });

      if from <= count && to >= count {
         global_count = global_count + 1;
      }

   }
   println!("Global count: {}", global_count);
}

pub(crate) fn solve_part2() {
   let stdin = io::stdin();

   let mut global_count = 0u32;

   for line in stdin.lock().lines() {
      let line = line.unwrap();
      let mut parts = line.split(&['-', ':', ' '][..]);
      let first = parts.next().unwrap().parse::<usize>().unwrap() - 1; //start from 1
      let second = parts.next().unwrap().parse::<usize>().unwrap() - 1;
      // first (and only) letter from string
      let letter = parts.next().unwrap().chars().next().unwrap();
      parts.next(); // drop whitespace
      let password = parts.next().unwrap();

      // count occurrences of letter
      let first = password.chars().nth(first).unwrap();
      let second = password.chars().nth(second).unwrap();

      if (first == letter) ^ (second == letter) {
         global_count = global_count + 1;
      }
   }
   println!("Global count: {}", global_count);
}