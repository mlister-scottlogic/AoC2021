

fn main() {
    day1();
}

fn day1() {
    let day_1_input = include_str!("inputs/day1.txt");
    let input = day_1_input.split("\n").map(|s| s.trim());

    let values = input.map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

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

    println!("Day 1 output: {}", output);
}
