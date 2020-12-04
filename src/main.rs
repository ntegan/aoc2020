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

/// Coordinate system:
///     e.g. slope is right 3, down 1
///     top left is (0, 0)
///     next point is (3, 1)
mod day_three {
    use crate::myerror;

    #[derive(Debug)]
    pub struct Pair(pub u64, pub u64);
    #[derive(Debug)]
    struct Toboggan {
        slope: Pair,
    }
    #[derive(Debug)]
    enum GridSpace {
        Me(Toboggan),
        Open,
        Tree,
    }
    impl GridSpace {
        pub fn from_char(car: char) -> Result<GridSpace, Box<dyn std::error::Error>> {
            match car {
                '.' => Ok(GridSpace::Open),
                '#' => Ok(GridSpace::Tree),
                _ => Err(Box::new(myerror::MyError)),
            }
        }
    }
    #[derive(Debug)]
    struct TobogganMapLine {
        spaces: Vec<GridSpace>,
    }
    impl TobogganMapLine {
        pub fn get_grid_space_at(
            &self,
            pos: u64,
        ) -> Result<&GridSpace, Box<dyn std::error::Error>> {
            Ok(&self.spaces[pos as usize])
        }
        pub fn from_string(input: &str) -> Result<TobogganMapLine, Box<dyn std::error::Error>> {
            Ok(TobogganMapLine {
                spaces: input
                    .chars()
                    .map(GridSpace::from_char)
                    .collect::<Result<Vec<GridSpace>, _>>()?,
            })
        }
        pub fn get_width(&self) -> u64 {
            self.spaces.len() as u64
        }
        pub fn manually_place_toboggan_at(&mut self, toboggan: Toboggan, at: u64) {
            self.spaces[at as usize] = GridSpace::Me(toboggan);
        }
    }
    #[derive(Debug)]
    pub struct TobogganMap {
        /// This pair represents the number of lines in the input map file
        ///         (pair.1)
        /// and the number of columns (pair.0) which is calculated using the
        ///         Toboggan's slope. The original number of columns
        ///         (in the input file) is forgotten/ignored after this Map
        ///         is constructed.
        ///
        size: Pair,
        lines: Vec<TobogganMapLine>,
    }
    impl TobogganMap {
        pub fn from_input(input: String) -> Result<TobogganMap, Box<dyn std::error::Error>> {
            let toboggan_lines = input
                .lines()
                .map(TobogganMapLine::from_string)
                .collect::<Result<Vec<TobogganMapLine>, _>>()?;
            Ok(TobogganMap {
                size: Pair(
                    toboggan_lines.len() as u64,
                    toboggan_lines[0].spaces.len() as u64,
                ),
                lines: toboggan_lines,
            })
        }
        pub fn place_toboggan_with_slope_at(&mut self, slope: Pair, at: Pair) {
            self.lines[at.1 as usize].manually_place_toboggan_at(Toboggan { slope }, at.0);
        }
        fn get_grid_space_at_pair(
            &self,
            position: &Pair,
        ) -> Result<&GridSpace, Box<dyn std::error::Error>> {
            self.lines[position.1 as usize].get_grid_space_at(position.0)
        }
        pub fn count_trees_would_hit(&self) -> Result<u64, Box<dyn std::error::Error>> {
            let mut position = Pair(0, 0);
            let slope = Pair(3, 1);
            let mut trees = 0;

            loop {
                let space = self.get_grid_space_at_pair(&position)?;
                match space {
                    GridSpace::Me(_me) => {}
                    GridSpace::Tree => trees = trees + 1,
                    GridSpace::Open => {}
                }
                position.0 = position.0 + slope.0;
                position.1 = position.1 + slope.1;
                position.0 = position.0 % self.lines[0].get_width();
                if position.1 >= self.lines.len() as u64 {
                    break;
                }
            }

            Ok(trees)
        }
    }
}

use day_three::TobogganMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input::read_until_eof()?;

    let mut tmap = TobogganMap::from_input(input)?;
    tmap.place_toboggan_with_slope_at(day_three::Pair(3, 1), day_three::Pair(0, 0));

    let trees = tmap.count_trees_would_hit()?;
    println!("Got trees: {}", trees);

    // TODO: get proper map width by calculating given height + slope

    Ok(())
}
