fn main() {
    let input = day1_input();

    println!("Day 1 Part 1: {}", day1_part1(&input));
    println!("Day 1 Part 2: {}", day1_part2(&input));
}

fn day1_part1(input: &Vec<i32>) -> i32 {
    let mut last_value: Option<i32> = None;
    let mut output = 0;

    for current_value in input.iter().copied() {
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

fn day1_part2(input: &Vec<i32>) -> i32 {
    const WINDOW_SIZE: usize = 3;
    let mut last_value: Option<i32> = None;
    let mut output = 0;

    let windows = input.windows(WINDOW_SIZE);

    for w in windows {
        let current_average = w.iter().sum();
        match last_value {
            Some(last) => {
                if current_average > last { output += 1 };
                last_value = Some(current_average);
            }
            None => {
                last_value = Some(current_average);
            }
        }
    }

    output
}

fn day1_input() -> Vec<i32> {
    let day_1_input = include_str!("input.txt");
    let input = day_1_input.split("\n").map(|s| s.trim());

    let values = input.map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

    values
}
