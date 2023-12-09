fn main() {
    let input = include_str!("input.txt");

    let res: i32 = input.lines().map(extrapolate).sum();

    println!("{}", res);
}

fn extrapolate(line: &str) -> i32 {
    let mut iterations: Vec<Vec<i32>> = Vec::new();
    iterations.push(
        line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>(),
    );

    while !iterations.last().unwrap().iter().all(|&x| x == 0) {
        iterations.push(
            iterations
                .last()
                .unwrap()
                .windows(2)
                .map(|a| a[1] - a[0])
                .collect::<Vec<i32>>(),
        );
    }

    iterations.iter().map(|x| x.last().unwrap()).sum()
}
