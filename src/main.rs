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
//
//
// Day 5.1
// =======
// Binary Boarding
//
// board plane discover new problem, dropped boarding pass.
// Don't know what seat is mine and flight attendants busy
//
// quick program use phone's camera scan all nearby boarding passes
// (puzzle input).
//
// airline doesn't use zones or gorups it uses binary space partitioning
// to seat people.
//
// A seat might be specified like `FBFBBFFRLR` where f front,back,left,right.
//
// First 7 chars either F or B. specify exactly 1 of the 128 rows
// on the plane (0-127).
// Each letter tells which half of a region the given seat is in.
// start with whole list of rows. First letter indicates whetehr
// seat in front (0-63) or back (64-127).
// next letter indicates which half of that region the seat is in,...
// until left with exactly one row.
//
// e.g. consider just first seven letters of
// `FBFBBFFRLR`
// start consider whole range 0-127,
// F lower half, keep 0-63
// B upper 32-63,
// F 32-47
// B 40-47
// B 44-47
// F 44-46
// final F keeps lower ro44
//
// last three letters withe L or R specify one of the 8columns of seats
// on that plane (0-7)
// same process as above but this time with only 3 steps.
// l lower, right upper.
// 0-7
// 4-7
// 4-5
// R upper col 5
//
// => row 44 col 5
//
// each seat unique seat id (row * 8 + column)
//
// this e.g. 44 * 8 + 5 = 357
//
// here are some other boarding passes
//     BFFFBBFRRR: row 70, column 7, seat ID 567.
//     FFFBBBFRRR: row 14, column 7, seat ID 119.
//     BBFFBBFRLL: row 102, column 4, seat ID 820.
//     o
// 
// What is the highest seat ID on a boarding pass?
//
//
//
//
//
