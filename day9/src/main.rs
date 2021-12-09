fn main() {
    let input = get_input();
    println!("{:?}", input);
}

fn get_input() -> Vec<Vec<u8>> {
    let raw_input = include_str!("testinput.txt");
    let input_lines = raw_input.lines();

    input_lines
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
