fn main() {
    let input = get_input();

    println!("Day 2 Part 2: {}", part1(&input));
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
            let splits = s.split(' ').collect::<Vec<&str>>();

            let control_type = match splits[0] {
                "forward" => ControlType::Forward,
                "down" => ControlType::Down,
                "up" => ControlType::Up,
                _ => panic!("Unknown input command found"),
            };

            let value = splits[1].parse::<i32>().unwrap();

            let control_message = ControlMessage {
                control_type,
                value,
            };

            control_message
        })
        .collect::<Vec<ControlMessage>>();

    values
}
