

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

    let mut last_value : Option<i32> = None;
    let mut output = 0;

    let windows = values.windows(WINDOW_SIZE);

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
    let day_1_input = include_str!("inputs/day1.txt");
    let input = day_1_input.split("\n").map(|s| s.trim());

    let values = input.map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

    values
}
