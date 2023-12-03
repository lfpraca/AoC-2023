use regex::Regex;

const SYMBOLS: &[char] = &['*', '@', '#', '-', '+', '=', '/', '$', '&', '%'];

fn main() {
    let input = include_str!("input.txt");
    let i_lines = input.lines().collect::<Vec<_>>();
    let re = Regex::new(r"\d+").unwrap();
    let mut tot = 0;

    for (i, &i_line) in i_lines.iter().enumerate() {
        let matches = re.find_iter(i_line);
        for m in matches {
            let range = m.start().saturating_sub(1)..=(m.end()).min(i_line.len() - 1);
            if i_line[range.clone()].chars().any(|c| SYMBOLS.contains(&c))
                || (i > 0
                    && i_lines[i - 1][range.clone()]
                        .chars()
                        .any(|c| SYMBOLS.contains(&c)))
                || (i < (i_lines.len() - 1)
                    && i_lines[i + 1][range].chars().any(|c| SYMBOLS.contains(&c)))
            {
                tot += m.as_str().parse::<u32>().unwrap();
            }
        }
    }

    println!("{}", tot);
}
