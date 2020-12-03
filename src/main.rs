mod input;
mod myerror;

// Day 1.1
// =======
// 5th year of AOC. Vacation at nice resort on tropical island.
// has its own currency and is entirely cash-only!
//
// Before leave, need to fix expense report (puzzle input).
// apparently it isn't quite adding up.
//
// Need to find two entries that sum up to 2020 and then multiply
// those numbers together
//
// Day 1.2
// =======
// find 3 numbers that meet the same criteria?
//
// Day 2.1
// =======
// Need t oget to coastal airport. quickest way is via toboggan.
// North Pole Toboggan Rental Shop owner has bad day, can't log into computers.
// password database corupted.
//
// debug problem given list of passwords (puzzle input) according to corrupted
// database and corporate policy when that password was set
//
// e.g.
// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
//
// policy indicates lowest and highest frequency a letter must appear for pass
// to be valid.
// 1-3 a means password must contain a [1,3] times.
// above example 2 passwords are valid.
//
//
// Day 2.2
// =======
// wasn't what the Official Toboggan Corporate Authentication System is expecting
// shopkeeper realizes accidentally explained password policy rules from
// old job at sled rental place down the street.
//
// Each policy actually decribes two positions in the password.
// 1 first char, 2 second char, so on.
// EXACLTY one of these positions must contain the given letter

mod authentication {
    use regex::Regex;
    use std::fmt;

    #[derive(Debug)]
    pub struct Policy {
        start: u64,
        end: u64,
        letter: char,
    }

    impl fmt::Display for Policy {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}-{} {}", self.start, self.end, self.letter)
        }
    }

    #[derive(Debug)]
    pub struct Entry {
        policy: Policy,
        password: String,
    }
    impl fmt::Display for Entry {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}: {}", self.policy, self.password)
        }
    }

    impl Entry {
        /// Ensure the frequency of self.policy.letter in self.password
        /// is in [self.policy.start, self.policy.ent]
        pub fn is_valid(&self) -> bool {
            if self.password.len() < self.policy.end as usize {
                return false;
            }
            let mut value = 0;
            if self
                .password
                .chars()
                .nth((self.policy.start - 1) as usize)
                .unwrap()
                == self.policy.letter
            {
                value = value + 1;
            }
            if self
                .password
                .chars()
                .nth((self.policy.end - 1) as usize)
                .unwrap()
                == self.policy.letter
            {
                value = value + 1;
            }
            value == 1
        }
    }

    pub fn parse_line_to_entry(line: &str) -> Result<Entry, Box<dyn std::error::Error>> {
        let re = Regex::new("^([0-9]*)-([0-9]*) ([a-zA-Z]): ([a-zA-Z]*)$")?;
        let caps = re.captures(line).ok_or("Couldn't get captures")?;

        let start = caps
            .get(1)
            .ok_or("Couldn't get capture group")?
            .as_str()
            .parse::<u64>()?;
        let end = caps
            .get(2)
            .ok_or("Couldn't get capture group")?
            .as_str()
            .parse::<u64>()?;
        let letter = caps
            .get(3)
            .ok_or("Couldn't get capture group")?
            .as_str()
            .parse::<char>()?;
        let password = caps.get(4).ok_or("Couldn't get capture group")?.as_str();

        Ok(Entry {
            policy: Policy { start, end, letter },
            password: password.into(),
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input::read_until_eof()?;

    let entries = input
        .lines()
        .map(authentication::parse_line_to_entry)
        .collect::<Result<Vec<authentication::Entry>, _>>()?;
    let valid_entries: Vec<&authentication::Entry> =
        entries.iter().filter(|entry| entry.is_valid()).collect();

    //println!("{}", entries);
    for entry in &entries {
        println!("{}", entry.to_string());
    }
    println!("Valid entries: {} / {}", valid_entries.len(), entries.len());

    Err(Box::new(myerror::MyError))
}
