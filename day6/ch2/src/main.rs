fn main() {
    let mut input = include_str!("input.txt").lines();
    let time = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let solution = (1..time)
        .skip_while(|x| x * (time - x) <= distance)
        .take_while(|x| x * (time - x) > distance)
        .count();

    println!("{}", solution);
}
