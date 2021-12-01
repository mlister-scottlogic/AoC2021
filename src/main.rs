

fn main() {
    day1_part1();
}

fn day1_part1() {
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

    println!("Day 1 part 1 output: {}", output);
}

fn day1_input() -> Vec<i32> {
    let day_1_input = include_str!("inputs/day1.txt");
    let input = day_1_input.split("\n").map(|s| s.trim());

    let values = input.map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

    values
}
