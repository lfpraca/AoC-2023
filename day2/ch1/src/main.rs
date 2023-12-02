use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    // let game_re = Regex::new(r"^Game (\d*):").unwrap();

    let mut tot = 0;
    'line: for (i, line) in input.lines().enumerate() {
        for section in line.split(';') {
            let red = red(section);
            let green = green(section);
            let blue = blue(section);
            if red > 12 || green > 13 || blue > 14 {
                continue 'line;
            }
        }
        tot += i + 1;
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
