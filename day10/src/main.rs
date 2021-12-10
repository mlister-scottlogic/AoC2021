fn main() {
    let input = get_input();

    println!("Day 10 Part 1 {}", part1(&input));
    println!("Day 10 Part 2 {}", part2(&input));
}

fn part2(input: &Vec<Vec<char>>) -> u64 {
    let mut scores = parse_input(input)
        .iter()
        .filter(|o| match *o {
            LineType::Incomplete(_) => true,
            _ => false,
        })
        .map(|row| match row {
            LineType::Incomplete(x) => find_incomplete_score(&mut x.clone()),
            _ => panic!("Unexpected value"),
        })
        .collect::<Vec<_>>();

    scores.sort();

    let mid = scores.len() / 2;

    scores[mid]
}

fn find_incomplete_score(opening_chars: &mut Vec<char>) -> u64 {
    opening_chars.reverse();

    opening_chars.iter().fold(0, |total, c| {
        let score = match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Unexpected character"),
        };
        total * 5 + score
    })
}

fn part1(input: &Vec<Vec<char>>) -> u32 {
    parse_input(input)
        .iter()
        .filter(|o| match *o {
            LineType::Corrupt(_) => true,
            _ => false,
        })
        .map(|row| match row {
            LineType::Corrupt(x) => x,
            _ => panic!("Unexpected value"),
        })
        .fold(0, |total, c| {
            let score = match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Unexpected character"),
            };
            total + score
        })
}

fn parse_input(input: &Vec<Vec<char>>) -> Vec<LineType> {
    input
        .iter()
        .map(|row| {
            let mut opening_chars = vec![];

            for &c in row {
                match c {
                    '(' => opening_chars.push(c),
                    '[' => opening_chars.push(c),
                    '{' => opening_chars.push(c),
                    '<' => opening_chars.push(c),

                    ')' => match opening_chars.pop() {
                        Some(x) => {
                            if x != '(' {
                                return LineType::Corrupt(c);
                            }
                        }
                        None => {
                            return LineType::Corrupt(c);
                        }
                    },
                    ']' => match opening_chars.pop() {
                        Some(x) => {
                            if x != '[' {
                                return LineType::Corrupt(c);
                            }
                        }
                        None => {
                            return LineType::Corrupt(c);
                        }
                    },
                    '}' => match opening_chars.pop() {
                        Some(x) => {
                            if x != '{' {
                                return LineType::Corrupt(c);
                            }
                        }
                        None => {
                            return LineType::Corrupt(c);
                        }
                    },
                    '>' => match opening_chars.pop() {
                        Some(x) => {
                            if x != '<' {
                                return LineType::Corrupt(c);
                            }
                        }
                        None => {
                            return LineType::Corrupt(c);
                        }
                    },

                    _ => panic!("Unexpected character {}", c),
                }
            }

            LineType::Incomplete(opening_chars)
        })
        .collect::<Vec<_>>()
}

enum LineType {
    Incomplete(Vec<char>),
    Corrupt(char),
}

fn get_input() -> Vec<Vec<char>> {
    let input = include_str!("input.txt");

    input
        .lines()
        .map(|i| i.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
