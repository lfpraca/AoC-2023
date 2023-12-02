use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    // let game_re = Regex::new(r"^Game (\d*):").unwrap();

    let mut tot = 0;
    for line in input.lines() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for section in line.split(';') {
            max_red = red(section).max(max_red);
            max_green = green(section).max(max_green);
            max_blue = blue(section).max(max_blue);
        }
        tot += max_red * max_green * max_blue;
    }
    println!("{}", tot);
}

fn red(x: &str) -> u32 {
    let red_re = Regex::new(r" (\d*) red").unwrap();

    red_re
        .captures(x)
        .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
        .unwrap_or(0)
}

fn green(x: &str) -> u32 {
    let green_re = Regex::new(r" (\d*) green").unwrap();

    green_re
        .captures(x)
        .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
        .unwrap_or(0)
}

fn blue(x: &str) -> u32 {
    let blue_re = Regex::new(r" (\d*) blue").unwrap();

    blue_re
        .captures(x)
        .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
        .unwrap_or(0)
}
