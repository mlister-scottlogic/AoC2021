

fn main() {
    println!("Day 1 Part 1: {}", day1_part1());
    println!("Day 1 Part 2: {}", day1_part2());
}

fn day1_part1() -> i32 {
    let values = day1_input();

    let mut last_value : Option<i32> = None;
    let mut output = 0;

    for current_value in values {
        match last_value {
            Some(last) => {
                if current_value > last { output += 1 };
                last_value = Some(current_value);
            }
            None => {
                last_value = Some(current_value);
            }
        }
    }

    output
}

fn day1_part2() -> i32 {
    const WINDOW_SIZE: usize = 3;

    let values = day1_input();

    let mut previous_average : i32 = (&values[..WINDOW_SIZE]).iter().sum();
    let mut output = 0;

    for i in 1..=(values.len() - WINDOW_SIZE) {
        let current_average : i32 = (&values[i..i+WINDOW_SIZE]).iter().sum();

        if current_average > previous_average { output += 1; }

        previous_average = current_average;
    }

    output
}

fn day1_input() -> Vec<i32> {
    let day_1_input = include_str!("inputs/day1.txt");
    let input = day_1_input.split("\n").map(|s| s.trim());

    let values = input.map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

    values
}
