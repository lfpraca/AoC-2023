use regex::Regex;

const DIGITS: &str = "1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine";

fn to_my_digits(input: &str) -> u32 {
    match input {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => unreachable!(),
    }
}

fn main() {
    let input = include_str!("input.txt").lines();

    let mut output = 0;
    let re_first = Regex::new(&format!("({})", DIGITS)).unwrap();
    let re_last = Regex::new(&format!(".*({})", DIGITS)).unwrap();
    for line in input {
        output += to_my_digits(re_first.captures(line).unwrap().get(1).unwrap().as_str()) * 10;
        output += to_my_digits(re_last.captures(line).unwrap().get(1).unwrap().as_str());
    }
    println!("{}", output);
}
