fn main() {
    let input = include_str!("input.txt");
    let mut tot = 0;

    for line in input.lines() {
        let mut l_sep = line.split('|');
        let w_nums = l_sep.next().unwrap().split_whitespace().collect::<Vec<_>>();
        tot += l_sep
            .next()
            .unwrap()
            .split_whitespace()
            .filter(|x| w_nums.contains(x))
            .fold(0, my_fold);
    }
    println!("{}", tot);
}

fn my_fold(acc: u32, _: &str) -> u32 {
    if acc == 0 {
        1
    } else {
        acc * 2
    }
}
