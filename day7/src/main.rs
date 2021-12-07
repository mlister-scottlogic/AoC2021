extern crate stopwatch;
use stopwatch::Stopwatch;

fn main() {
    let input = get_input();

    println!("Day 7 Part 1: {}", part1(&input));
    let sw = Stopwatch::start_new();
    println!("Day 7 Part 2: {}", part2(&input));
    println!("Part 2 took {}ms", sw.elapsed_ms());
}

fn part1(input_positions: &Vec<i32>) -> i32 {
    find_min_distance(input_positions, |target: i32, distance: i32| {
        (target - distance).abs()
    })
}

fn part2(input_positions: &Vec<i32>) -> i32 {
    find_min_distance(input_positions, |target: i32, distance: i32| {
        add_cumulative((target - distance).abs())
    })
}

fn find_min_distance(
    input_positions: &Vec<i32>,
    measure_distance: fn(target: i32, distance: i32) -> i32,
) -> i32 {
    let biggest_position = *input_positions.iter().max().unwrap();
    let smallest_position = *input_positions.iter().min().unwrap();

    (smallest_position..=biggest_position)
        .into_iter()
        .map(|target| {
            input_positions
                .iter()
                .map(|&distance| measure_distance(target, distance))
                .sum()
        })
        .min()
        .unwrap()
}

fn add_cumulative(current_value: i32) -> i32 {
    (current_value + 1) * (current_value) / 2
}

fn get_input() -> Vec<i32> {
    let raw_input = include_str!("input.txt");
    raw_input
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>()
}
