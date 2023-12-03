use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let i_lines = input.lines().collect::<Vec<_>>();
    let mut tot = 0;

    for (li, &i_line) in i_lines.iter().enumerate() {
        for (ci, c) in i_line.chars().enumerate() {
            if c != '*' {
                continue;
            }

            let mut nums = Vec::new();
            {
                let my = &i_lines[li][ci - 1..=ci + 1];
                nums.extend(get_nums(my, ci - 1, i_line));
            }
            if nums.len() <= 2 && li > 0 {
                // Not first line
                let line = &i_lines[li - 1];
                let my = &line[ci - 1..=ci + 1];
                nums.extend(get_nums(my, ci - 1, line));
            }
            if nums.len() <= 2 && li < i_lines.len() {
                // Not last line
                let line = &i_lines[li + 1];
                let my = &line[ci - 1..=ci + 1];
                nums.extend(get_nums(my, ci - 1, line));
            }

            if nums.len() == 2 {
                tot += nums.iter().product::<u32>();
            }
        }
    }

    println!("{}", tot);
}

fn get_nums(section: &str, ci: usize, line: &str) -> Vec<u32> {
    let re = Regex::new(r"\d+").unwrap();

    re.find_iter(section)
        .map(|m| get_num(ci + m.start(), line))
        .collect::<Vec<_>>()
}

fn get_num(ci: usize, line: &str) -> u32 {
    let mut start = ci + 1;
    let mut l_iter = line.chars().rev().skip(line.len() - 1 - ci);
    while l_iter.next().is_some_and(|c| c.is_ascii_digit()) {
        start -= 1;
    }

    line.chars()
        .skip(start)
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}
