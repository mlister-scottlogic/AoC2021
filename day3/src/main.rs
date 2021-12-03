fn main() {
    let input = get_input();

    let most_common = most_common(&input);

    println!("Day 3 Part 1: {}", part1(&most_common));
}

fn part1(inputs: &(String, String)) -> i32 {
    let gamma = i32::from_str_radix(&inputs.0, 2).unwrap();
    let epsilon = i32::from_str_radix(&inputs.1, 2).unwrap();

    gamma * epsilon
}

fn most_common(input: &Vec<Vec<char>>) -> (String, String) {
    let input_length = input[0].len();
    let mut position_counter = vec![0; input_length];

    for binary_string in input {
        for (i, c) in binary_string.iter().enumerate() {
            match c {
                '0' => position_counter[i] -= 1,
                '1' => position_counter[i] += 1,
                _ => panic!("Unknown binary value"),
            }
        }
    }

    let gamma_string = position_counter
        .iter()
        .copied()
        .map(|v| if v >= 0 { "1" } else { "0" })
        .collect::<String>();

    let epsilon_string = position_counter
        .iter()
        .copied()
        .map(|v| if v >= 0 { "0" } else { "1" })
        .collect::<String>();

    (gamma_string, epsilon_string)
}

fn get_input() -> Vec<Vec<char>> {
    let day_2_input = include_str!("input.txt");
    let input = day_2_input.split("\n").map(|s| s.trim());

    let values = input
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    values
}
