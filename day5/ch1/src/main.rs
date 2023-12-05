use std::str::Lines;

struct MyMap {
    destination: u64,
    source: u64,
    length: u64,
}

fn main() {
    let mut input = include_str!("input.txt").lines();
    let seeds: Vec<u64> = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    // Skip two lines
    input.next().unwrap();
    input.next().unwrap();

    let mut my_maps = Vec::new();

    for _ in 1..=7 {
        let mut my_map = Vec::new();
        fill_vec(&mut my_map, &mut input);
        my_maps.push(my_map);
    }

    let mut min_end = u64::MAX;
    for seed in seeds {
        let mut end = seed;
        for my_map in &my_maps {
            end = match my_map
                .iter()
                .find(|x| x.source <= end && x.length > (end - x.source))
            {
                Some(x) => x.destination + (end - x.source),
                None => end,
            };
        }
        min_end = min_end.min(end);
    }
    println!("{}", min_end);
}

fn fill_vec(my_vec: &mut Vec<MyMap>, input: &mut Lines<'_>) {
    for l in input.by_ref() {
        if l.is_empty() {
            break;
        }

        let mut values = l.split_whitespace().map(|x| x.parse::<u64>().unwrap());
        my_vec.push(MyMap {
            destination: values.next().unwrap(),
            source: values.next().unwrap(),
            length: values.next().unwrap(),
        });
    }

    let _ = input.next();
}
