fn main() {
    let input = get_input();

    println!("Day 10 Part 1 {}", part1(&input));
    println!("Day 10 Part 2 {}", part2(&input));
}

fn part2(input: &Vec<Vec<char>>) -> u64 {
    let mut scores = parse_input(input)
        .iter()
        .filter(|o| o.1.is_none())
        .map(|row| find_incomplete_score(&mut row.0.clone()))
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
        .map(|i| i.1)
        .filter(|o| o.is_some())
        .map(|n| n.unwrap())
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

fn parse_input(input: &Vec<Vec<char>>) -> Vec<(Vec<char>, Option<char>)> {
    input
        .iter()
        .map(|row| {
            let mut opening_chars = vec![];

            for c in row {
                match c {
                    '(' => opening_chars.push('('),
                    '[' => opening_chars.push('['),
                    '{' => opening_chars.push('{'),
                    '<' => opening_chars.push('<'),

                    ')' => match opening_chars.last() {
                        Some(x) => {
                            if *x != '(' {
                                return (opening_chars, Some(*c));
                            }

                            opening_chars.pop();
                        }
                        None => {
                            return (opening_chars, Some(*c));
                        }
                    },
                    ']' => match opening_chars.last() {
                        Some(x) => {
                            if *x != '[' {
                                return (opening_chars, Some(*c));
                            }

                            opening_chars.pop();
                        }
                        None => {
                            return (opening_chars, Some(*c));
                        }
                    },
                    '}' => match opening_chars.last() {
                        Some(x) => {
                            if *x != '{' {
                                return (opening_chars, Some(*c));
                            }

                            opening_chars.pop();
                        }
                        None => {
                            return (opening_chars, Some(*c));
                        }
                    },
                    '>' => match opening_chars.last() {
                        Some(x) => {
                            if *x != '<' {
                                return (opening_chars, Some(*c));
                            }

                            opening_chars.pop();
                        }
                        None => {
                            return (opening_chars, Some(*c));
                        }
                    },

                    _ => panic!("Unexpected character {}", c),
                }
            }

            (opening_chars, None)
        })
        .collect::<Vec<_>>()
}

fn get_input() -> Vec<Vec<char>> {
    let input = include_str!("input.txt");

    input
        .lines()
        .map(|i| i.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
