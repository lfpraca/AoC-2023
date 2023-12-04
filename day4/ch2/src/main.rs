use std::io::{self, Write};

#[derive(Clone)]
struct Card {
    id: u32,
    w_count: u32,
}

fn main() {
    let input = include_str!("input.txt");
    let mut cards = Vec::new();

    for line in input.lines() {
        let id = line[5..=7]
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let mut l_sep = line.split('|');
        let w_nums = l_sep.next().unwrap().split_whitespace().collect::<Vec<_>>();
        let w_count = l_sep
            .next()
            .unwrap()
            .split_whitespace()
            .filter(|x| w_nums.contains(x))
            .count() as u32;
        cards.push(Card { id, w_count });
    }

    let max_id = cards.iter().map(|x| x.id).max().unwrap();
    for i in 1..=max_id {
        print!("\rProcessing {}", i);
        io::stdout().flush().unwrap();
        let mut new_cards = Vec::new();
        for card in cards.iter().filter(|x| x.id == i) {
            for n in 1..=card.w_count {
                let insert_card = cards.iter().find(|x| x.id == i + n).cloned().unwrap();
                new_cards.push(insert_card);
            }
        }
        cards.extend(new_cards);
    }

    println!("\n{}", cards.len());
}
