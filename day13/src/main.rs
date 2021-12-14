fn main() {
    let input = get_input();
    let instructions = get_instructions();

    println!("{:?}", input);
    println!("{:?}", instructions);
}

fn get_input() -> Vec<Vec<bool>> {
    let input = include_str!("testinput.txt");

    let points = input
        .lines()
        .map(|l| {
            let split = l.split(",").collect::<Vec<_>>();
            (
                split[0].parse::<usize>().unwrap(),
                split[1].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let biggest_x = points
        .iter()
        .map(|p| p.0)
        .max()
        .expect("Unexpected empty collection")
        + 1;
    let biggest_y = points
        .iter()
        .map(|p| p.1)
        .max()
        .expect("Unexpected empty collection")
        + 1;

    let mut grid = vec![vec![false; biggest_x]; biggest_y];

    for p in points {
        grid[p.1][p.0] = true;
    }

    grid
}

fn get_instructions() -> Vec<Instruction> {
    let input = include_str!("testinput2.txt");

    input
        .lines()
        .map(|l| {
            let split = l.split("fold along ").collect::<Vec<_>>();
            let split = split[1].split("=").collect::<Vec<_>>();

            match split[0] {
                "x" => Instruction::X(split[1].parse().unwrap()),
                "y" => Instruction::Y(split[1].parse().unwrap()),
                _ => panic!("Unexpected command"),
            }
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
enum Instruction {
    X(usize),
    Y(usize),
}
