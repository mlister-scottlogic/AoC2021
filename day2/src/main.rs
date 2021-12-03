fn main() {
    let input = get_input();

    println!("Day 2 Part 1: {}", part1(&input));
    println!("Day 2 Part 2: {}", part2(&input));
}

fn part1(input: &Vec<ControlMessage>) -> i32 {
    let coordinates = input.iter().fold((0, 0), |(x, y), control_message| {
        match control_message.control_type {
            ControlType::Forward => (x + control_message.value, y),
            ControlType::Down => (x, y + control_message.value),
            ControlType::Up => (x, y - control_message.value),
        }
    });

    coordinates.0 * coordinates.1
}

fn part2(input: &Vec<ControlMessage>) -> i32 {
    let coordinates =
        input.iter().fold(
            (0, 0, 0),
            |(x, y, aim), control_message| match control_message.control_type {
                ControlType::Forward => (
                    x + control_message.value,
                    y + control_message.value * aim,
                    aim,
                ),
                ControlType::Down => (x, y, aim + control_message.value),
                ControlType::Up => (x, y, aim - control_message.value),
            },
        );

    coordinates.0 * coordinates.1
}

enum ControlType {
    Forward,
    Down,
    Up,
}

struct ControlMessage {
    control_type: ControlType,
    value: i32,
}

fn get_input() -> Vec<ControlMessage> {
    let day_2_input = include_str!("input.txt");
    let input = day_2_input.split("\n").map(|s| s.trim());

    let values = input
        .map(|s| {
            let splits = s.split(' ').collect::<Vec<_>>();

            let control_type = match splits[0] {
                "forward" => ControlType::Forward,
                "down" => ControlType::Down,
                "up" => ControlType::Up,
                _ => panic!("Unknown input command found"),
            };

            let value = splits[1].parse().unwrap();

            ControlMessage {
                control_type,
                value,
            }
        })
        .collect::<Vec<_>>();

    values
}
