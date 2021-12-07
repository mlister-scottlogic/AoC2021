fn main() {
    let input = get_input();

    println!("Day 7 Part 1: {}", part1(&input));
    println!("Day 7 Part 2: {}", part2(&input));
}

fn part1(input_positions: &Vec<i32>) -> i32 {
    let biggest_position = *input_positions.iter().max().unwrap();

    let mut distances: Vec<i32> = vec![];

    for i in 0..=biggest_position {
        distances.push(input_positions.iter().map(|p| (i - p).abs()).sum());
    }

    *distances.iter().min().unwrap()
}

fn part2(input_positions: &Vec<i32>) -> i32 {
    let biggest_position = *input_positions.iter().max().unwrap();

    let mut distances: Vec<i32> = vec![];

    for i in 0..=biggest_position {
        distances.push(
            input_positions
                .iter()
                .map(|p| add_cumulative((i - p).abs(), 0))
                .sum(),
        );
    }

    *distances.iter().min().unwrap()
}

fn add_cumulative(current_value: i32, total: i32) -> i32 {
    if current_value == 0 {
        return total;
    }

    add_cumulative(current_value - 1, total + current_value)
}

fn get_input() -> Vec<i32> {
    let raw_input = include_str!("input.txt");
    raw_input
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>()
}
