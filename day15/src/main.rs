use std::collections::BinaryHeap;
use std::collections::HashMap;

use std::cmp::Ordering;

use stopwatch::Stopwatch;

fn main() {
    println!("Day 15 part 1 {}", part1());
    let sw = Stopwatch::start_new();
    // do something that takes some time
    println!("Day 15 part 2 {}", part2());
    println!("Thing took {}ms", sw.elapsed_ms());

    // Djikstra with 50 copies : 60,976ms
    // A* with Manhattan distance: 59,281
    // Virtually no difference
}

fn part1() -> i32 {
    let input = get_input();

    let grid_size = 99;

    let cheapest_path = a_star(
        (0, 0),
        (grid_size, grid_size),
        input.clone(),
        manhattan_distance,
    )
    .expect("No path found");

    let mut score = 0;

    for i in 1..cheapest_path.len() {
        score += input[&cheapest_path[i]];
    }

    score
}

fn part2() -> i32 {
    let mut input = get_input();

    let copies = 5;
    let initial_grid_size = 100;
    let big_grid_size = initial_grid_size * copies - 1;

    for y in 0..=big_grid_size {
        for x in 0..=big_grid_size {
            let seed_y = y % initial_grid_size;
            let seed_x = x % initial_grid_size;

            let y_offset = y / initial_grid_size;
            let x_offset = x / initial_grid_size;

            let initial_value = input[&(seed_x, seed_y)];

            let mut new_value = initial_value + y_offset + x_offset;

            if new_value > 9 {
                new_value = new_value % 10 + 1;
            }

            input.insert((x, y), new_value);
        }
    }

    let cheapest_path = a_star(
        (0, 0),
        (big_grid_size, big_grid_size),
        input.clone(),
        manhattan_distance,
    )
    .expect("No path found");

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
    heuristic_distance: fn((i32, i32), (i32, i32)) -> i32,
) -> Option<Vec<(i32, i32)>> {
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    let mut open_set = BinaryHeap::new();
    open_set.push(FScore {
        node: start,
        fscore: heuristic_distance(start, goal),
    });

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
    // to n currently known.
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score = HashMap::new();
    for n in all_nodes.iter() {
        g_score.insert(*n.0, i32::MAX);
    }
    g_score.insert(start, 0);

    while !open_set.is_empty() {
        // This operation can occur in O(1) time if openSet is a min-heap or a priority queue
        // current := the node in openSet having the lowest fScore[] value
        let best_node = open_set.pop().unwrap();
        let current = best_node.node;

        if current == goal {
            return Some(reconstruct_path(came_from, current));
        }

        let neighbours = get_neighbours(current, &all_nodes);

        for neighbour in neighbours {
            let tentative_g_score = g_score[&current] + all_nodes[&neighbour];

            if tentative_g_score < g_score[&neighbour] {
                *came_from.entry(neighbour).or_insert(current) = current;

                g_score.insert(neighbour, tentative_g_score);

                let new_f_score = tentative_g_score + heuristic_distance(neighbour, goal);

                open_set.push(FScore {
                    node: neighbour,
                    fscore: new_f_score,
                });
            }
        }
    }

    // Open set is empty but goal was never reached
    // return failure
    None
}

#[derive(Debug)]
struct FScore {
    node: (i32, i32),
    fscore: i32,
}

impl Ord for FScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fscore.cmp(&other.fscore).reverse()
    }
}

impl PartialOrd for FScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for FScore {}

impl PartialEq for FScore {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
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
