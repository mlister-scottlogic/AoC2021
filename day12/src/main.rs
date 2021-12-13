use std::collections::HashMap;

fn main() {
    let input = get_input();

    println!("{:?}", input);
    println!("Day 12 part 1 {:?}", part1(&input.clone()));
}

fn part1(caves: &HashMap<Cave, Vec<Cave>>) -> u32 {
    let start_cave = Cave::Small("start".to_string());

    visit_cave(start_cave, caves.clone(), vec![])
}

fn visit_cave(
    current_cave: Cave,
    caves: HashMap<Cave, Vec<Cave>>,
    visited_caves: Vec<Cave>,
) -> u32 {
    // We've managed to get to the end
    if current_cave == Cave::Small("end".to_string()) {
        println!("{:?}", visited_caves);
        return 1;
    }

    // println!("Visiting cave {:?}", current_cave);

    let mut caves = caves.clone();

    // If small, remove from other caves possible routes
    match current_cave {
        Cave::Small(_) => {
            for (_, c) in caves.iter_mut() {
                let remaining_values = c
                    .iter()
                    .filter(|ref x| ***x != current_cave)
                    .map(|y| y.clone())
                    .collect::<Vec<_>>();

                *c = remaining_values;
            }
        }
        _ => (),
    }

    // println!("{:?}", caves);
    let temp_cave = caves.clone();
    let connected_caves = temp_cave
        .get(&current_cave)
        .expect("Unexpected missing node");
    let mut newly_visited_caves = visited_caves.clone();
    newly_visited_caves.push(current_cave.clone());

    match current_cave {
        Cave::Small(_) => {
            caves.remove(&current_cave);
        }
        _ => (),
    }

    let mut total_routes = 0;
    // For each cave connected visit
    for connected_cave in connected_caves.iter() {
        total_routes += visit_cave(
            connected_cave.clone(),
            caves.clone(),
            newly_visited_caves.clone(),
        )
    }

    total_routes
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Cave {
    Large(String),
    Small(String),
}

fn get_input() -> HashMap<Cave, Vec<Cave>> {
    let input = include_str!("input.txt");

    let pairs = input
        .lines()
        .map(|l| {
            let pair = l.split("-").collect::<Vec<_>>();
            (
                parse_cave(pair[0].to_string()),
                parse_cave(pair[1].to_string()),
            )
        })
        .collect::<Vec<_>>();

    let mut hash_map: HashMap<Cave, Vec<Cave>> = HashMap::new();

    for p in pairs {
        hash_map.entry(p.0.clone()).or_default().push(p.1.clone());
        hash_map.entry(p.1.clone()).or_default().push(p.0.clone());
    }

    hash_map
}

fn parse_cave(input: String) -> Cave {
    if input == input.to_uppercase() {
        return Cave::Large(input);
    }

    Cave::Small(input)
}
