use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("{}", part1());
}

fn part1() -> i32 {
    let input = get_input();

    let grid_size = 99;

    println!("Starting path");

    let cheapest_path = a_star(
        (0, 0),
        (grid_size, grid_size),
        input.clone(),
        manhattan_distance,
    )
    .expect("No path found");
    println!("Finished path");

    println!("{:?}", cheapest_path);

    let mut score = 0;

    for i in 1..cheapest_path.len() {
        score += input[&cheapest_path[i]];
    }

    score
}

fn manhattan_distance(current: (i32, i32), goal: (i32, i32)) -> i32 {
    let x_distance = goal.0 - current.0;
    let y_distance = goal.1 - current.1;

    x_distance + y_distance
}

fn a_star(
    start: (i32, i32),
    goal: (i32, i32),
    all_nodes: HashMap<(i32, i32), i32>,
    h: fn((i32, i32), (i32, i32)) -> i32,
) -> Option<Vec<(i32, i32)>> {
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    let mut open_set = HashSet::new();
    open_set.insert(start);

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
    // to n currently known.
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score = HashMap::new();
    for n in all_nodes.iter() {
        g_score.insert(*n.0, i32::MAX);
    }
    g_score.insert(start, 0);

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how short a path from start to finish can be if it goes through n.
    let mut f_score = HashMap::new();
    for n in all_nodes.iter() {
        f_score.insert(*n.0, i32::MAX);
    }
    f_score.insert(start, h(start, goal));

    while !open_set.is_empty() {
        // This operation can occur in O(1) time if openSet is a min-heap or a priority queue
        // current := the node in openSet having the lowest fScore[] value
        let open_set_clone = open_set.clone();
        let best_node = open_set_clone
            .iter()
            .map(|n| (n, f_score[n]))
            .min_by_key(|&(_, score)| score)
            .unwrap();
        let current = best_node.0;

        if *current == goal {
            return Some(reconstruct_path(came_from, *current));
        }

        open_set.remove(current);

        let neighbours = get_neighbours(*current, &all_nodes);

        for neighbour in neighbours {
            let tentative_g_score = g_score[current] + all_nodes[&neighbour];

            if tentative_g_score < g_score[&neighbour] {
                *came_from.entry(neighbour).or_insert(*current) = *current;

                g_score.insert(neighbour, tentative_g_score);

                let new_f_score = tentative_g_score + h(neighbour, goal);
                f_score.insert(neighbour, new_f_score);

                if !open_set.contains(&neighbour) {
                    open_set.insert(neighbour);
                }
            }
        }
    }

    // Open set is empty but goal was never reached
    // return failure
    None
}

fn reconstruct_path(
    came_from: HashMap<(i32, i32), (i32, i32)>,
    current: (i32, i32),
) -> Vec<(i32, i32)> {
    let mut current = current;
    let mut total_path = vec![current];

    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.push(current);
    }

    total_path.reverse();

    total_path
}

fn get_neighbours(node: (i32, i32), all_nodes: &HashMap<(i32, i32), i32>) -> Vec<(i32, i32)> {
    let possible_nodes = vec![
        (node.0 + 1, node.1),
        (node.0 - 1, node.1),
        (node.0, node.1 + 1),
        (node.0, node.1 - 1),
    ];

    possible_nodes
        .iter()
        .filter(|&n| all_nodes.contains_key(n))
        .map(|&n| n)
        .collect::<Vec<_>>()
}

fn get_input() -> HashMap<(i32, i32), i32> {
    let input = include_str!("input.txt");

    let mut hashmap = HashMap::new();

    let input = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            hashmap.insert((x as i32, y as i32), input[y][x]);
        }
    }

    hashmap
}
