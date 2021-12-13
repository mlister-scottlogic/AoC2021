fn main() {
    let input = get_input();

    println!("Day 11 part 1 {}", part1(&input));
    println!("Day 11 part 2 {}", part2(&input));
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let grid_size = 10;

    let mut input = input.clone();

    let mut not_synced = true;
    let mut step = 0;

    while not_synced {
        step += 1;
        // Increment
        for row in input.iter_mut() {
            for cell in row {
                *cell += 1;
            }
        }

        let mut unchanged_grid = true;

        while unchanged_grid {
            unchanged_grid = false;

            for y in 0..grid_size {
                for x in 0..grid_size {
                    if input[y][x] == 10 {
                        unchanged_grid = true;

                        input[y][x] += 1;

                        for y_offset in -1..=1 {
                            for x_offset in -1..=1 {
                                // println!("x {} y {} x_off {} y_off {}", x, y, x_offset, y_offset);
                                let y_position: i32 = y as i32 + y_offset;
                                let x_position: i32 = x as i32 + x_offset;

                                // println!("x_position {} y_position {}", x_position, y_position);

                                if x_position >= grid_size as i32
                                    || x_position < 0
                                    || y_position >= grid_size as i32
                                    || y_position < 0
                                {
                                    continue;
                                }

                                if input[y_position as usize][x_position as usize] < 10 {
                                    input[y_position as usize][x_position as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // reset
        for row in input.iter_mut() {
            for cell in row {
                if *cell > 9 {
                    *cell = 0;
                }
            }
        }

        not_synced = input.iter().any(|row| row.iter().any(|&c| c != 0));
    }

    step
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let grid_size = 10;

    let mut input = input.clone();

    let mut flashes = 0;

    for _i in 0..100 {
        // Increment
        for row in input.iter_mut() {
            for cell in row {
                *cell += 1;
            }
        }

        let mut unchanged_grid = true;

        while unchanged_grid {
            unchanged_grid = false;

            for y in 0..grid_size {
                for x in 0..grid_size {
                    if input[y][x] == 10 {
                        unchanged_grid = true;

                        flashes += 1;

                        input[y][x] += 1;

                        for y_offset in -1..=1 {
                            for x_offset in -1..=1 {
                                // println!("x {} y {} x_off {} y_off {}", x, y, x_offset, y_offset);
                                let y_position: i32 = y as i32 + y_offset;
                                let x_position: i32 = x as i32 + x_offset;

                                // println!("x_position {} y_position {}", x_position, y_position);

                                if x_position >= grid_size as i32
                                    || x_position < 0
                                    || y_position >= grid_size as i32
                                    || y_position < 0
                                {
                                    continue;
                                }

                                if input[y_position as usize][x_position as usize] < 10 {
                                    input[y_position as usize][x_position as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // reset
        for row in input.iter_mut() {
            for cell in row {
                if *cell > 9 {
                    *cell = 0;
                }
            }
        }
    }

    flashes
}

fn get_input() -> Vec<Vec<u32>> {
    let input = include_str!("input.txt");

    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
