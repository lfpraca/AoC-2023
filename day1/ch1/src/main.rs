fn main() {
    let input = include_str!("input.txt").lines();

    let mut output = 0;
    for line in input {
        output += line
            .chars()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap()
            * 10;
        output += line
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();
    }
    println!("{}", output);
}
