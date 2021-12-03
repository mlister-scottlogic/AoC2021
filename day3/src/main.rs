fn main() {
    let input = get_input();

    println!("Day 3 Part 1: {}", part1(&input));
    // println!("Day 3 Part 2: {}", part2(&input));
}

fn part1(readings: &Vec<Vec<char>>) -> i32 {
    let gamma_string = most_common(readings, '1').iter().collect::<String>();
    let epsilon_string = most_common(readings, '0').iter().collect::<String>();

    let gamma = i32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon_string, 2).unwrap();

    gamma * epsilon
}

// fn part2(readings: &Vec<Vec<char>>, most_common: &(String, String)) -> i32 {
//     let oxygen_generator = hidden_reading_extractor(readings, &most_common.0.chars().collect(), 0);
//     let co2_scrubber = hidden_reading_extractor(readings, &most_common.1.chars().collect(), 0);

//     println!("{}", oxygen_generator);
//     println!("{}", co2_scrubber);

//     oxygen_generator * co2_scrubber
// }

// fn hidden_reading_extractor(
//     input: &Vec<Vec<char>>,
//     index: usize,
//     find_most_common: &dyn Fn(&Vec<Vec<char>>) -> Vec<char>,
// ) -> i32 {
//     if input.len() == 1 {
//         let hidden_reading_string = input[0].iter().collect::<String>();

//         return i32::from_str_radix(&hidden_reading_string, 2).unwrap();
//     }

//     let still_valid_inputs = input
//         .iter()
//         .filter(|c| c[index] == bit_matcher[index])
//         .cloned()
//         .collect();

//     let new_index = index + 1;

//     hidden_reading_extractor(&still_valid_inputs, bit_matcher, new_index)
// }

fn most_common(input: &Vec<Vec<char>>, target: char) -> Vec<char> {
    let input_length = input[0].len();
    let mut position_counter = vec![0; input_length];

    for binary_string in input {
        for i in 0..binary_string.len() {
            if binary_string[i] == target {
                position_counter[i] += 1
            } else {
                position_counter[i] -= 1
            }
        }
    }

    position_counter
        .iter()
        .copied()
        .map(|v| if v >= 0 { '1' } else { '0' })
        .collect()
}

fn get_input() -> Vec<Vec<char>> {
    let day_2_input = include_str!("input.txt");
    let input = day_2_input.split("\n").map(|s| s.trim());

    let values = input
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    values
}
