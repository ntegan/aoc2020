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
//
//
// Day 3.1
// =======
// Toboggan travel to the airport is easy but not safe.
// minimal steering and  area is covered in trees -- yikes.
// Calculate angles that will take me near the fewest trees.
//
// Due to local geology, trees only grow on integral coordinates on a grid.
// Made a map (puzzle input) of visible open squares (.) and trees (#).
// ..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#
//
// Due to something I read once about arboreal genetics and biome stability,
// same pattern repeats to the right many times.
//
// ..##.........##.........##.........##.........##.........##.......  --->
// #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
// .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
// ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
// .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
// ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
// .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
// .#........#.#........#.#........#.#........#.#........#.#........#
// #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
// #...##....##...##....##...##....##...##....##...##....##...##....#
// .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//
// start on open square in top left, and need to reach the bottom.
// (below the bottom-most row on the map).
//
// toboggan can only follow a few specific slopes, should've chosen the
// more expensive model that can work with more than just rational numbers.
//
// To start, count all trees would encounter for slope right 3, down 1.
// in this map, traversing would cause to encounter 7 trees.
//
// How many trees would encounter with the real map input?
//
//
// Day 3.2
// =======
// now to check the rest of the slopes.
// need to minimize the probability of a sudden arboreal stop.
//
// determine number of trees would encounter for each of following slopes
//
// right    down
// 1        1
// 3        1
// 5        1
// 7        1
// 1        2
//
// in above example, slopes would find 2,7,3,4,2 trees
// multiplied together produces 336
//
//
// Day 4.1
// =======
// Arrive at airport yay. Grabbed North Pole Credentials instead of passport!
// not valid for travel in most of the world.
//
// not the only one having problems. long line formed for automatic passport
// scanners. Delay could upset travel itinerary.
// Questionable network security, might be able to solve both problems at once.
//
// automatic passport scanners slow because having trouble detecting which
// passp[orts have all required fields.
// Expected fields:
// byr
// iyr
// eyr
// hgt
// hcl
// ecl
// pid
// cid
// birth yr, issue yr, expiration yr, height, hair color, eye color, passport id,
// country id
//
// pport data validated in batch files (puzzle input).
// each passporet represented as a sequence of key:fvalue pairs
// separated by spaces or newlines.
// passports separated by blank lines.
// example with 4 passports
// ```
// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm
//
// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
// hcl:#cfa07d byr:1929
//
// hcl:#ae17e1 iyr:2013
// eyr:2024
// ecl:brn pid:760753108 byr:1931
// hgt:179cm
//
// hcl:#cfa07d eyr:2025 pid:166559648
// iyr:2011 ecl:brn hgt:59in
// ```
//
// first pport valid, all 8 firleds present.
// second invalid missing height.
// third only missing field s cid, so looks like north pole cerendials, not pport.
//
// surely nobody would mind if you made system temporarily ignore missing cid fields.
// treat thisas fvalid.
//
// 2 valid passports in above example.
//
// COUNT the number of valid passports.
//

mod day_four {
    use crate::myerror;
    #[derive(Debug)]
    struct BirthYear(u64);
    #[derive(Debug)]
    struct IssueYear(u64);
    #[derive(Debug)]
    struct ExpirationYear(u64);
    #[derive(Debug)]
    enum Height {
        Inches(u64),
        Centimeters(u64),
    }
    #[derive(Debug)]
    struct HairColor(u8, u8, u8);
    #[derive(Debug)]
    struct EyeColor(String);
    #[derive(Debug)]
    struct PassportId(u64);
    #[derive(Debug)]
    struct CountryId(u64);

    #[derive(Debug)]
    struct KeyValuePair(String, String);
    impl KeyValuePair {
        pub fn from_colon_separated_string(
            string: &str,
        ) -> Result<KeyValuePair, Box<dyn std::error::Error>> {
            let parts = string.split(":").collect::<Vec<&str>>();
            match parts.len() {
                2 => Ok(KeyValuePair(String::from(parts[0]), String::from(parts[1]))),
                _ => Err(Box::new(myerror::MyError)),
            }
        }
    }

    #[derive(Debug)]
    pub struct Passport {
        birth_year: BirthYear,
        issue_year: IssueYear,
        expiration_year: ExpirationYear,
        height: Height,
        hair_color: HairColor,
        eye_color: EyeColor,
        passport_id: PassportId,
        country_id: Option<CountryId>,
    }
    impl Passport {
        fn passport_from_string(string: String) -> Result<Passport, Box<dyn std::error::Error>> {
            let mut pairs = Vec::new();
            for passport_item in string.split(" ") {
                let pair = KeyValuePair::from_colon_separated_string(passport_item)?;
                pairs.push(pair);
            }
            let birth_year = BirthYear(
                pairs
                    .iter()
                    .find(|&pair| pair.0 == "byr")
                    .ok_or("Couldn't find pair")?
                    .1
                    .parse::<u64>()?,
            );
            let issue_year = IssueYear(
                pairs
                    .iter()
                    .find(|&pair| pair.0 == "iyr")
                    .ok_or("Couldn't find pair")?
                    .1
                    .parse::<u64>()?,
            );
            let expiration_year = ExpirationYear(
                pairs
                    .iter()
                    .find(|&pair| pair.0 == "eyr")
                    .ok_or("Couldn't find pair")?
                    .1
                    .parse::<u64>()?,
            );
            let height;
            let heightt = &pairs
                .iter()
                .find(|&pair| pair.0 == "hgt")
                .ok_or("Couldn't find pair")?
                .1;
            if heightt.contains("cm") {
                height = Height::Centimeters(heightt[..heightt.len() - 2].parse::<u64>()?);
            } else if heightt.contains("inches") {
                height = Height::Inches(heightt[..heightt.len() - 6].parse::<u64>()?);
            } else {
                return Err(Box::new(myerror::MyError));
            }

            let hair_color = &pairs
                .iter()
                .find(|&pair| pair.0 == "hcl")
                .ok_or("Couldn't find pair")?
                .1;
            let x = u8::from_str_radix(&hair_color[1..3], 16)?;
            let y = u8::from_str_radix(&hair_color[3..5], 16)?;
            let z = u8::from_str_radix(&hair_color[5..7], 16)?;
            let hair_color = HairColor(x, y, z);

            let eye_color = EyeColor(String::from(
                &pairs
                    .iter()
                    .find(|&pair| pair.0 == "ecl")
                    .ok_or("Couldn't find pair")?
                    .1,
            ));

            let passport_id = PassportId(
                pairs
                    .iter()
                    .find(|&pair| pair.0 == "pid")
                    .ok_or("Couldn't find pair")?
                    .1
                    .parse::<u64>()?,
            );

            let country_id = match pairs.iter().find(|&pair| pair.0 == "cid") {
                Some(pair) => Some(CountryId(pair.1.parse::<u64>()?)),
                None => None,
            };

            let passport = Passport {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
                country_id,
            };
            Ok(passport)
        }
        pub fn passports_from_string(
            input: &String,
        ) -> Result<Vec<Passport>, Box<dyn std::error::Error>> {
            let passports = input
                .split("\n\n")
                .map(|passport_line| {
                    passport_line
                        .chars()
                        .map(|c| match c {
                            '\n' => ' ',
                            _ => c,
                        })
                        .filter(|c| *c != '\n')
                        .collect::<String>()
                })
                .map(Passport::passport_from_string)
                .filter(|f| f.is_ok())
                .collect::<Result<Vec<Passport>, _>>()?;
            Ok(passports)
        }
    }
}

use day_four::Passport;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input::read_until_eof()?;
    let passports = Passport::passports_from_string(&input)?;
    println!("Got passports {}", passports.len());
    Ok(())
}
