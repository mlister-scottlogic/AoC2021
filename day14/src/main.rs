use std::collections::HashMap;

fn main() {
    println!("Day 14 part 1 {}", part1());
}

fn part1() -> i32 {
    let rules = get_input();
    let output = process_rules("VHCKBFOVCHHKOHBPNCKO".to_string(), rules, 10);

    let mut counts = HashMap::new();

    for c in output.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let max = counts.clone().into_iter().max_by_key(|&(_, count)| count);
    let min = counts.clone().into_iter().min_by_key(|&(_, count)| count);

    max.unwrap().1 - min.unwrap().1
}

fn process_rules(input: String, rules: HashMap<String, InsertionRule>, iterations: u32) -> String {
    let mut current_string = input;
    for _i in 0..iterations {
        let chars = current_string.chars().collect::<Vec<_>>();
        let mut new_chars = chars.clone();

        let windows = chars.windows(2);

        let mut new_chars_added = 0;

        for (i, window) in windows.enumerate() {
            let pair_string = window.iter().collect::<String>();

            let insert_rule = rules.get(&pair_string);

            match insert_rule {
                Some(x) => {
                    // println!("rule: {:?} window: {}", x, pair_string);

                    new_chars.splice(
                        i + new_chars_added..=i + new_chars_added + 1,
                        x.output.iter().cloned(),
                    );
                    new_chars_added += 1;
                }
                None => (),
            }
        }

        current_string = new_chars.iter().collect::<String>();

        println!("i {}", _i);
    }

    current_string
}

#[derive(Debug)]
struct InsertionRule {
    output: Vec<char>,
}

fn get_input() -> HashMap<String, InsertionRule> {
    let input = include_str!("input.txt");

    let values = input
        .lines()
        .map(|l| {
            let split = l.split(" -> ").collect::<Vec<_>>();

            let output = split[1].parse::<char>().unwrap();

            let input_chars = split[0].chars().collect::<Vec<_>>();
            let pair = (input_chars[0], input_chars[1]);

            (
                split[0].to_string(),
                InsertionRule {
                    output: vec![pair.0, output, pair.1],
                },
            )
        })
        .collect::<Vec<_>>();

    let mut hashmap = HashMap::new();

    for v in values {
        hashmap.insert(v.0, v.1);
    }

    hashmap
}
