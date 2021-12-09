fn main() {
    let input = get_input();
    // println!("{:?}", input);

    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}

fn part2(heightmap: &Vec<Vec<u32>>) -> u32 {
    let mut visited_map = heightmap
        .iter()
        .map(|row| row.iter().map(|&i| i == 9).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut basin_sizes = find_basin_sizes(&mut visited_map);

    // println!("{:?}", basin_sizes);

    basin_sizes.sort();
    basin_sizes.reverse();

    return basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
}

fn find_basin_sizes(visited_map: &mut Vec<Vec<bool>>) -> Vec<u32> {
    let column_height = visited_map.len();
    let row_width = visited_map[0].len();

    let mut basin_sizes = vec![];

    for y in 0..column_height {
        for x in 0..row_width {
            if !visited_map[y][x] {
                basin_sizes.push(crawl_basin(visited_map, x, y));
            }
        }
    }

    basin_sizes
}

fn crawl_basin(visited_map: &mut Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    if visited_map[y][x] {
        return 0;
    }

    let column_height = visited_map.len();
    let row_width = visited_map[0].len();

    let mut points_in_basin = 1;
    visited_map[y][x] = true;

    if x > 0 {
        points_in_basin += crawl_basin(visited_map, x - 1, y);
    }

    if x < (row_width - 1) {
        points_in_basin += crawl_basin(visited_map, x + 1, y);
    }

    if y > 0 {
        points_in_basin += crawl_basin(visited_map, x, y - 1);
    }

    if y < (column_height - 1) {
        points_in_basin += crawl_basin(visited_map, x, y + 1);
    }

    points_in_basin
}

fn part1(heightmap: &Vec<Vec<u32>>) -> u32 {
    let mut total_risk: u32 = 0;

    let column_height = heightmap.len();
    let row_width = heightmap[0].len();

    // println!("x:{} y:{}", row_width, column_height);

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
