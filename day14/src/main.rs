use std::collections::HashMap;

fn main() {
    println!("Day 14 part 1 {}", part1());
    println!("Day 14 part 2 {}", part2());
}

fn part1() -> i64 {
    let rules = get_input();
    let output = process_rules("VHCKBFOVCHHKOHBPNCKO".to_string(), rules, 10);

    let mut counts = HashMap::new();

    for t in output {
        let chars = t.0.chars().collect::<Vec<_>>();

        *counts.entry(chars[0]).or_insert(0) += t.1;
        *counts.entry(chars[1]).or_insert(0) += t.1;
    }

    let max = counts.clone().into_iter().max_by_key(|&(_, count)| count);
    let min = counts.clone().into_iter().min_by_key(|&(_, count)| count);

    (max.unwrap().1 + 1) / 2 - (min.unwrap().1 + 1) / 2
}

fn part2() -> i64 {
    let rules = get_input();
    let output = process_rules("VHCKBFOVCHHKOHBPNCKO".to_string(), rules, 40);

    let mut counts = HashMap::new();

    for t in output {
        let chars = t.0.chars().collect::<Vec<_>>();

        *counts.entry(chars[0]).or_insert(0) += t.1;
        *counts.entry(chars[1]).or_insert(0) += t.1;
    }

    let max = counts.clone().into_iter().max_by_key(|&(_, count)| count);
    let min = counts.clone().into_iter().min_by_key(|&(_, count)| count);

    (max.unwrap().1 + 1) / 2 - (min.unwrap().1 + 1) / 2
}

fn process_rules(
    input: String,
    rules: HashMap<String, InsertionRule>,
    iterations: u32,
) -> HashMap<String, i64> {
    let mut hashmap = HashMap::new();

    let chars = input.chars().collect::<Vec<_>>();

    let windows = chars.windows(2);

    for (_i, window) in windows.enumerate() {
        let pair_string = window.iter().collect::<String>();

        *hashmap.entry(pair_string).or_insert(0) += 1
    }

    for _i in 0..iterations {
        let mut new_hashmap = HashMap::new();
        for e in hashmap.iter() {
            let insert_rule = rules.get(e.0);

            match insert_rule {
                Some(x) => {
                    let output1 = &x.output.0;
                    let output2 = &x.output.1;

                    *new_hashmap.entry(output1.clone()).or_insert(0) += e.1;
                    *new_hashmap.entry(output2.clone()).or_insert(0) += e.1;
                }
                None => (),
            }
        }

        hashmap = new_hashmap;
    }

    // println!("{:?}", hashmap);
    hashmap
}

#[derive(Debug)]
struct InsertionRule {
    output: (String, String),
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
                    output: (
                        vec![pair.0, output].iter().collect::<String>(),
                        vec![output, pair.1].iter().collect::<String>(),
                    ),
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
