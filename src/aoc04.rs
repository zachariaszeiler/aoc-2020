use std::io::{BufRead, StdinLock};

const NECESSARY_FIELDS: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid"
];

/// streaming solution of part1 that parses input to passport objects
pub fn solve() {
    let stdin = std::io::stdin();

    let correct = passport_iter(stdin.lock())
        .map(|p| p.validate_part1())
        .filter(|t| *t)
        .count();

    println!("{} correct passports.", correct);
}

/// simple solution of part1
pub fn solve_p1() {
    let stdin = std::io::stdin();

    let mut valid_passports = 0u32;

    // current passport state (multiline input)
    let mut entries = Vec::default();

    for line in stdin.lock().lines() {
        let values = line.unwrap().split_whitespace().map(|s| &s[0..3]).map(str::to_owned).collect::<Vec<String>>();

        // empty line => end of passport
        if values.is_empty() {
            let valid = NECESSARY_FIELDS.iter().all(|f| entries.iter().any(|s| s == f));

            if valid {
                valid_passports = valid_passports + 1;
            }

            entries.clear();
        } else {
            entries.extend(values);
        }
    }

    println!("{}", valid_passports);
}

/// return an iterator of passport structs
fn passport_iter(stdin: StdinLock) -> impl Iterator<Item=Passport> + '_ {
    stdin.lines()
        .map(Result::unwrap)
        .flat_map(|s: String| {
            s.split(" ")
                .map(|s| s.split(":").collect_tuple())
                .collect::<Vec<Option<(String, String)>>>()
        })
        .scan(Passport::default(), parse_ports)
        .filter_map(|p| p)
}

/// Itertools::collect_tuple couldn't infer tuple type in map chain
trait CollectTuple {
    fn collect_tuple(&mut self) -> Option<(String, String)>;
}

impl<'a, T> CollectTuple for T
    where T: Iterator<Item=&'a str> {
    fn collect_tuple(&mut self) -> Option<(String, String)> {
        let first = self.next();
        let second = self.next();
        if first.is_some() && second.is_some() {
            Some((first?.to_owned(), second?.to_owned()))
        } else {
            None
        }
    }
}

/// Combine tuples to passport objects
fn parse_ports(passport: &mut Passport, entry: Option<(String, String)>) -> Option<Option<Passport>> {
    if let Some((key, value)) = entry {
        passport.enter(&key, value);
        Some(None)
    } else {
        let copy = passport.clone();
        passport.clear();
        Some(Some(copy))
    }
}

#[derive(Debug, Clone, Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    /// set all fields to None (scan reuses passport object)
    fn clear(&mut self) {
        self.byr = None;
        self.iyr = None;
        self.eyr = None;
        self.hgt = None;
        self.hcl = None;
        self.ecl = None;
        self.pid = None;
        self.cid = None;
    }

    /// set field to value
    fn enter(&mut self, field: &str, value: String) {
        match field {
            "byr" => self.byr = Some(value),
            "iyr" => self.iyr = Some(value),
            "eyr" => self.eyr = Some(value),
            "hgt" => self.hgt = Some(value),
            "hcl" => self.hcl = Some(value),
            "ecl" => self.ecl = Some(value),
            "pid" => self.pid = Some(value),
            "cid" => self.cid = Some(value),
            _ => panic!()
        }
    }

    fn validate_part1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    /// that's super annoying, even with regex
    fn validate_part2(&self) -> bool {
        let byr = self.byr.as_ref().map(|byr| byr.parse::<u32>().map_or(false, |byr| {
            byr >= 1920 && byr <= 2002
        })).unwrap_or(false);

        byr
        // too much effort
    }
}
