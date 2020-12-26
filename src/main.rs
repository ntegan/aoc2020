mod input;
mod myerror;

//  Day 10: Adapter Array
//  Day 10.1
//  ========
//  Each joltage adapter is rated for a specific output joltage (puzzle input).
//  Any given adapter can take an input 1,2,or3 jolts lower than its rating.
//
//  Also, device has a built-in adapter rated for 3 volts higher than the
//  highest rated adapter in your bag.
//
//  The charging outlet near your seat has an effective joltage rating of 0.
//
//  If use every adapter in your bag at once, what is the distribution of
//  joltage differences between the charging outlet, the adapters, and your
//  device?

//use regex::Regex;
//let re = Regex::new("^([[:alpha:]]*) ([+-][0-9]*)$").unwrap();
//let caps = re.captures(slice).unwrap();
//let op_code_str = caps.get(1).unwrap().as_str();
//let value = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
mod day_ten  {
    pub fn get_differences(numbers: Vec<i64>) -> Vec<i64> {
        let mut ret = Vec::new();

        for i in 0..numbers.len() - 1 {
            ret.push(
                numbers[i + 1] - numbers[i]
                )
        }


        ret
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input::read_until_eof()?;
    let input = input.trim();
    let mut numbers = input
        .split("\n")
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()
        .unwrap();

    numbers.push(0);
    numbers.push(numbers.iter().max().unwrap() + 3);
    numbers.sort();
    println!("{:?}", numbers);

    let mut differences = day_ten::get_differences(numbers);

    differences.sort();

    println!("{:?}", differences);

    let mut differences2 = differences.clone();
    differences2.dedup();

    for difference in differences2 {
        println!("diff {}, times {}", difference, 
            differences.iter().filter(|x| **x  == difference).count());
    }

    println!("{:?}", differences);



    Ok(())
}
