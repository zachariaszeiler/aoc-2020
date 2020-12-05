use std::io::BufRead;
use std::iter;

// Idea:
// 1. sort numbers O(n log n)
// 2. search from beginning and end O(n)
// => O(n log n) instead of naive O(n^2)
pub fn solve() {
    let io = std::io::stdin();

    let mut numbers =  io.lock().lines().map(|line| {
        line.unwrap().parse::<u32>().unwrap()
    }).collect::<Vec<u32>>();
    numbers.sort();

    let mut iter = numbers.iter();
    let mut rev_iter = numbers.iter().rev();

    let mut first = iter.next().unwrap();
    let mut second = rev_iter.next().unwrap();

    while first < second {
        if first + second > 2020 {
            second = rev_iter.next().unwrap();
        } else if first + second < 2020 {
            first = iter.next().unwrap();
        } else if first + second == 2020 {
            println!("{}", first * second);
            break;
        }
    }
}

// O(n^3)
pub fn solve_part2() {
    let io = std::io::stdin();

    // collect input
    let numbers =  io.lock().lines().map(|line| {
        line.unwrap().parse::<u32>().unwrap()
    }).collect::<Vec<u32>>();

    // build iterator of all (a, b, c) where a,b,c \in numbers
    let it = numbers.iter();
    let it = it.clone().flat_map(|a| {
        it.clone().zip(iter::repeat(a)).flat_map(|(a, b)| {
            it.clone().zip(iter::repeat((a, b))).map(|(a, p)| {
                (a, p.0, p.1)
            })
        })
    });

    for (a, b, c) in it {
        if a + b + c == 2020 {
            println!("{}", a * b * c);
            break;
        }
    }
}

