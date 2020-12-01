mod input;
mod myerror;

// Day 1
// =====
// 5th year of AOC. Vacation at nice resort on tropical island.
// has its own currency and is entirely cash-only!
//
// Before leave, need to fix expense report (puzzle input).
// apparently it isn't quite adding up.
//
// Need to find two entries that sum up to 2020 and then multiply
// those numbers together
//

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input::read_until_eof()?;

    let expense_report_entries = input
        .lines()
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;

    for i in 0..expense_report_entries.len() {
        for j in 0..expense_report_entries.len() {
            let a = expense_report_entries[i];
            let b = expense_report_entries[j];
            if a + b == 2020 {
                println!("{} + {} == 2020", a, b);
                println!("{} * {} == {}", a, b, a * b);
                return Ok(());
            }
        }
    }

    Err(Box::new(myerror::MyError))
}
