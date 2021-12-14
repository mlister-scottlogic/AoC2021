fn main() {
    let input = get_input();
    let instructions = get_instructions();

    // println!("{:?}", input);
    // println!("{:?}", instructions);

    println!("{:?}", part1(input.clone(), instructions[0]));
    part2(input.clone(), instructions);
}

fn part1(grid: Vec<Vec<bool>>, instruction: Instruction) -> usize {
    let new_grid = fold_paper(grid, instruction);

    let counts = new_grid
        .iter()
        .map(|row| row.into_iter().filter(|&&b| b).count())
        .collect::<Vec<_>>();

    counts.iter().sum()
}

fn part2(grid: Vec<Vec<bool>>, instructions: Vec<Instruction>) {
    let mut new_grid = grid;

    for instruction in instructions {
        new_grid = fold_paper(new_grid, instruction);
    }

    for row in new_grid {
        println!(
            "{:?}",
            row.iter()
                .map(|&b| if b { '#' } else { '.' })
                .collect::<Vec<_>>()
        );
    }
}

fn fold_paper(grid: Vec<Vec<bool>>, instruction: Instruction) -> Vec<Vec<bool>> {
    let x_size = grid[0].len();
    let y_size = grid.len();

    println!(
        "x_size:{} y_size:{} instruction{:?}",
        x_size, y_size, instruction
    );

    match instruction {
        Instruction::X(v) => {
            let mut half_grid = vec![vec![false; v]; y_size];

            for y in 0..y_size {
                for x in 0..v {
                    let current_val = grid[y][x];
                    let opposite_val = grid[y][(x_size - 1) - x];

                    half_grid[y][x] = current_val || opposite_val;
                }
            }

            half_grid
        }
        Instruction::Y(v) => {
            let mut half_grid = vec![vec![false; x_size]; v];

            for y in 0..v {
                for x in 0..x_size {
                    let current_val = grid[y][x];
                    let opposite_val = grid[(y_size - 1) - y][x];

                    half_grid[y][x] = current_val || opposite_val;
                }
            }

            half_grid
        }
    }
}

fn get_input() -> Vec<Vec<bool>> {
    let input = include_str!("input.txt");

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

    let mut biggest_x = points
        .iter()
        .map(|p| p.0)
        .max()
        .expect("Unexpected empty collection")
        + 1;
    let mut biggest_y = points
        .iter()
        .map(|p| p.1)
        .max()
        .expect("Unexpected empty collection")
        + 1;

    if biggest_x % 2 == 0 {
        biggest_x += 1;
    }

    if biggest_y % 2 == 0 {
        biggest_y += 1;
    }

    let mut grid = vec![vec![false; biggest_x]; biggest_y];

    for p in points {
        grid[p.1][p.0] = true;
    }

    grid
}

fn get_instructions() -> Vec<Instruction> {
    let input = include_str!("input2.txt");

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

#[derive(Debug, Copy, Clone)]
enum Instruction {
    X(usize),
    Y(usize),
}
