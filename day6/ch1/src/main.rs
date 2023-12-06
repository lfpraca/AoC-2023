fn main() {
    let mut input = include_str!("input.txt").lines();
    let time = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap());
    let distance = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap());

    let solution: usize = time
        .zip(distance)
        .map(|(t, d)| {
            (1..t)
                .skip_while(|x| x * (t - x) <= d)
                .take_while(|x| x * (t - x) > d)
                .count()
        })
        .product();

    println!("{}", solution);
}
