fn main() {
    let input = get_input();

    println!("Day 10 Part 1 {}", part1(&input));
    println!("Day 10 Part 2 {}", part2(&input));
}

fn part2(input: &Vec<Vec<char>>) -> u64 {
    let mut scores = input
        .iter()
        .map(|row| find_incomplete_score(row))
        .filter(|score| score.is_some())
        .map(|score| score.unwrap())
        .collect::<Vec<_>>();

    scores.sort();

    let mid = scores.len() / 2;

    scores[mid]
}

fn find_incomplete_score(row: &Vec<char>) -> Option<u64> {
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
                        return None;
                    }

                    opening_chars.pop();
                }
                None => {
                    return None;
                }
            },
            ']' => match opening_chars.last() {
                Some(x) => {
                    if *x != '[' {
                        return None;
                    }

                    opening_chars.pop();
                }
                None => {
                    return None;
                }
            },
            '}' => match opening_chars.last() {
                Some(x) => {
                    if *x != '{' {
                        return None;
                    }

                    opening_chars.pop();
                }
                None => {
                    return None;
                }
            },
            '>' => match opening_chars.last() {
                Some(x) => {
                    if *x != '<' {
                        return None;
                    }

                    opening_chars.pop();
                }
                None => {
                    return None;
                }
            },

            _ => panic!("Unexpected character {}", c),
        }
    }

    opening_chars.reverse();

    Some(opening_chars.iter().fold(0, |total, c| {
        let score = match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Unexpected character"),
        };
        total * 5 + score
    }))
}

fn part1(input: &Vec<Vec<char>>) -> u32 {
    input
        .iter()
        .fold(0, |total, row| total + find_illegal_char_score(row))
}

fn find_illegal_char_score(row: &Vec<char>) -> u32 {
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
                        return 3;
                    }

                    opening_chars.pop();
                }
                None => {
                    return 3;
                }
            },
            ']' => match opening_chars.last() {
                Some(x) => {
                    if *x != '[' {
                        return 57;
                    }

                    opening_chars.pop();
                }
                None => {
                    return 57;
                }
            },
            '}' => match opening_chars.last() {
                Some(x) => {
                    if *x != '{' {
                        return 1197;
                    }

                    opening_chars.pop();
                }
                None => {
                    return 1197;
                }
            },
            '>' => match opening_chars.last() {
                Some(x) => {
                    if *x != '<' {
                        return 25137;
                    }

                    opening_chars.pop();
                }
                None => {
                    return 25137;
                }
            },

            _ => panic!("Unexpected character {}", c),
        }
    }

    0
}

fn get_input() -> Vec<Vec<char>> {
    let input = include_str!("input.txt");

    input
        .lines()
        .map(|i| i.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
