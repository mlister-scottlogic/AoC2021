fn main() {
    let input = get_input();

    println!("Day 3 Part 1: {}", part1(&input));
    println!("Day 3 Part 2: {}", part2(&input));
}

fn part1(readings: &Vec<Vec<bool>>) -> i32 {
    let gamma = convert_to_int(&most_common(readings, true));
    let epsilon = convert_to_int(&most_common(readings, false));

    gamma * epsilon
}

fn part2(readings: &Vec<Vec<bool>>) -> i32 {
    let oxygen_most_common = |remaining_input: &Vec<Vec<_>>| most_common(remaining_input, true);
    let co2_most_common = |remaining_input: &Vec<Vec<_>>| most_common(remaining_input, false);

    let oxygen_generator = hidden_reading_extractor(readings, 0, &oxygen_most_common);

    let co2_scrubber = hidden_reading_extractor(readings, 0, &co2_most_common);

    oxygen_generator * co2_scrubber
}

fn convert_to_int(bit_list: &Vec<bool>) -> i32 {
    let string_binary = bit_list
        .iter()
        .map(|v| (*v as u8).to_string())
        .collect::<String>();

    i32::from_str_radix(&string_binary, 2).unwrap()
}

fn hidden_reading_extractor(
    input: &Vec<Vec<bool>>,
    index: usize,
    find_most_common: &dyn Fn(&Vec<Vec<bool>>) -> Vec<bool>,
) -> i32 {
    if input.len() == 1 {
        return convert_to_int(&input[0]);
    }

    // This is doing more work than it has to, would be better to pass only the items after the index
    let most_common_remaining = find_most_common(input);

    let still_valid_inputs = input
        .iter()
        .filter(|c| c[index] == most_common_remaining[index])
        .cloned()
        .collect();

    let new_index = index + 1;

    hidden_reading_extractor(&still_valid_inputs, new_index, find_most_common)
}

fn most_common(input: &Vec<Vec<bool>>, target: bool) -> Vec<bool> {
    let input_length = input[0].len();
    let mut position_counter = vec![0; input_length];

    for input_line in input {
        for (i, t) in input_line.into_iter().enumerate() {
            if *t == target {
                position_counter[i] += 1
            } else {
                position_counter[i] -= 1
            }
        }
    }

    position_counter
        .iter()
        .copied()
        .map(|v| {
            if v == 0 {
                return target;
            }

            v > 0
        })
        .collect()
}

fn get_input() -> Vec<Vec<bool>> {
    let day_2_input = include_str!("input.txt");
    let input = day_2_input.lines().map(|s| s.trim());

    let values = input
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => panic!("Unknown binary number"),
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    values
}
