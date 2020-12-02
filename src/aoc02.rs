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

      // count occurences of letter
      let count = password.chars().fold(0u32, |a, c|{ if c == letter {a + 1} else { a } });

      if from <= count && to >= count {
         global_count = global_count + 1;
      }

   }
   println!("Global count: {}", global_count);
}