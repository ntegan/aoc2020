mod input;
mod myerror;

//  Day 9: Encoding Error
//  Day 9.1
//  =======
//  Port on seat in front of you outputs a series of numbers (puzzle input).
//  Data encrypted with eXchange-Masking Addition System (XMAS)
//  which is an old cypher with important weakness.
//
//  XMAS starts by transmit preamble of 25 numbers.
//  After that, each number receive should be sum of any two of the
//  25 immediately preceding numbers.
//  The two numbers will have different values and may be more than one such
//  pair.
//
//  Find the first number in the list (after preamble) which is not
//  the sum of two of the 25 numbers before it?
//  What is the first number that does not have this property?

//use regex::Regex;
//let re = Regex::new("^([[:alpha:]]*) ([+-][0-9]*)$").unwrap();
//let caps = re.captures(slice).unwrap();
//let op_code_str = caps.get(1).unwrap().as_str();
//let value = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
mod day_nine {

    pub struct XmasCipher {
        preamble_size: usize,
        numbers: Vec<i64>,
    }
    impl XmasCipher {
        fn fits_cipher(&self, index: usize) -> bool {
            let preceding = &self.numbers[index - self.preamble_size..index];
            for i in 0..preceding.len() {
                for j in i+1..preceding.len() {
                    if preceding[i] + preceding[j] == self.numbers[index] {
                        return true;
                    }
                }
            }

            false
        }
        pub fn get_first_number_not_fitting(&self) -> i64 {
            for i in self.preamble_size..self.numbers.len() {
                println!("{}", i);
                if !self.fits_cipher(i) {
                    return self.numbers[i];
                }
            }
            panic!("DIdn't find a number");
        }
        pub fn new(numbers: Vec<i64>, preamble_size: usize) -> XmasCipher {
            XmasCipher {
                preamble_size,
                numbers,
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input::read_until_eof()?;
    let input = input.trim();
    let numbers = input
        .split("\n")
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()
        .unwrap();

    let cipher = day_nine::XmasCipher::new(numbers, 25);

    let answer = cipher.get_first_number_not_fitting();
    println!("Answer: {}", answer);

    Ok(())
}
