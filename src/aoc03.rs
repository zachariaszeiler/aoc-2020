use std::io;
use std::io::BufRead;

pub fn solve() {
    let mut slopes = vec![Slope::new(3, 1)];
    solve_multiple(&mut slopes);
    println!("Number of trees: {}", slopes[0].count);
}

pub fn solve_part2() {
    let mut slopes = vec![
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ];
    solve_multiple(&mut slopes);

    let solution: u32 = slopes.iter().map(|s| { s.count }).product();

    println!("Product: {}", solution);
}

struct Slope {
    x: usize,
    y: usize,
    count: u32,
}

impl Slope {
    fn new(x: usize, y: usize) -> Self {
        Slope { x, y, count: 0 }
    }
}

// O(n)
fn solve_multiple(slopes: &mut Vec<Slope>) {
    io::stdin().lock().lines().enumerate().skip(1).for_each(|(i, line)| {
        let line = line.unwrap();
        let length = line.len(); // this counts bytes instead of chars, but works for ascii input and isn't O(n)

        for slope in slopes.iter_mut() {
            if i % slope.y == 0 {
                let position = (i * slope.x) % length;
                if line.chars().nth(position).unwrap() == '#' {
                    slope.count = slope.count + 1;
                }
            }
        }
    });
}