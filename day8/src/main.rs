fn main() {
    let input = get_input();

    println!("Day 8 Part 1 {}", part1(&input))
}

fn part1(input: &Vec<SignalLine>) -> u32 {
    let mut one_count = 0;
    let mut four_count = 0;
    let mut seven_count = 0;
    let mut eight_count = 0;

    for l in input {
        for s in l.output.iter() {
            match s.chars().count() {
                2 => one_count += 1,
                4 => four_count += 1,
                3 => seven_count += 1,
                7 => eight_count += 1,
                _ => (),
            }
        }
    }

    one_count + four_count + seven_count + eight_count
}

struct SignalLine {
    input: Vec<String>,
    output: Vec<String>,
}

fn get_input() -> Vec<SignalLine> {
    let raw_input = include_str!("input.txt");
    let input_lines = raw_input.lines();

    input_lines
        .map(|s| {
            let splits = s.split('|').collect::<Vec<_>>();

            let input = splits[0]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            let output = splits[1]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            SignalLine { input, output }
        })
        .collect::<Vec<_>>()
}
