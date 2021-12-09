fn main() {
    let input = get_input();
    // println!("{:?}", input);

    println!("{:?}", part1(&input));
}

fn part1(heightmap: &Vec<Vec<u32>>) -> u32 {
    let mut total_risk: u32 = 0;

    let column_height = heightmap.len();
    let row_width = heightmap[0].len();

    println!("x:{} y:{}", row_width, column_height);

    for y in 0..column_height {
        for x in 0..row_width {
            let current_value = heightmap[y][x];

            if x > 0 {
                let left = heightmap[y][x - 1];

                if current_value >= left {
                    continue;
                }
            }

            if x < (row_width - 1) {
                let right = heightmap[y][x + 1];

                if current_value >= right {
                    continue;
                }
            }

            if y > 0 {
                let up = heightmap[y - 1][x];

                if current_value >= up {
                    continue;
                }
            }

            if y < (column_height - 1) {
                let down = heightmap[y + 1][x];

                if current_value >= down {
                    continue;
                }
            }

            // println!("x:{} y:{} value:{}", x, y, current_value);

            total_risk = total_risk + current_value + 1;
        }
    }

    total_risk
}

fn get_input() -> Vec<Vec<u32>> {
    let raw_input = include_str!("input.txt");
    let input_lines = raw_input.lines();

    input_lines
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
