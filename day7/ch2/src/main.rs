use std::cmp::Ordering;

use itertools::Itertools;

enum HandOrder {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeKind = 3,
    FullHouse = 4,
    FourKind = 5,
    FiveKind = 6,
}

const CARD_ORDER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn main() {
    let input = include_str!("input.txt");
    let result = input
        .lines()
        .map(|x| {
            let mut parts = x.split_whitespace();
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .sorted_unstable_by(my_sort)
        .enumerate()
        .fold(0, |acc, (i, (_, v))| acc + v * (i as u32 + 1));

    println!("{:#?}", result);
}

fn my_sort(x: &(&str, u32), y: &(&str, u32)) -> Ordering {
    match find_value(x.0).cmp(&find_value(y.0)) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => compare_order(x.0, y.0),
    }
}

fn compare_order(x: &str, y: &str) -> Ordering {
    for (i, c) in x.chars().enumerate() {
        let x_val = CARD_ORDER.into_iter().take_while(|&my| my != c).count();
        let y_val = CARD_ORDER
            .into_iter()
            .take_while(|&my| my != y.chars().nth(i).unwrap())
            .count();
        match x_val.cmp(&y_val) {
            Ordering::Equal => continue,
            r => return r,
        }
    }
    unreachable!();
}

fn find_value(mut s: &str) -> u8 {
    if s == "JJJJJ" {
        s = "22222";
    }
    let s_nums = s
        .chars()
        .sorted_unstable()
        .dedup_with_count()
        .collect::<Vec<_>>();
    let j_count = s_nums.iter().find(|x| x.1 == 'J').map(|x| x.0).unwrap_or(0);
    let max_char = s_nums
        .iter()
        .filter(|x| x.1 != 'J')
        .max_by_key(|x| x.0)
        .unwrap()
        .1;
    let s_nums = s_nums
        .iter()
        .filter(|x| x.1 != 'J')
        .map(|x| if x.1 != max_char { x.0 } else { x.0 + j_count })
        .sorted_unstable()
        .collect::<Vec<_>>();

    (match s_nums.as_slice() {
        [5] => HandOrder::FiveKind,
        [1, 4] => HandOrder::FourKind,
        [2, 3] => HandOrder::FullHouse,
        [1, 1, 3] => HandOrder::ThreeKind,
        [1, 2, 2] => HandOrder::TwoPair,
        [1, 1, 1, 2] => HandOrder::OnePair,
        [1, 1, 1, 1, 1] => HandOrder::HighCard,
        _ => unreachable!(),
    }) as u8
}
