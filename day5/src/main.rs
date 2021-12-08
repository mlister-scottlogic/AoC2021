fn main() {
    let instructions = get_input();

    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
}

fn part1(instructions: &Vec<Instruction>) -> usize {
    const GRID_SIZE: usize = 1000;
    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];

    for instruction in instructions {
        let covered_points = instruction.get_covered_points(false);

        for point in covered_points {
            grid[point.y][point.x] += 1;
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|&n| *n >= 2).count())
        .sum::<usize>()
}

fn part2(instructions: &Vec<Instruction>) -> usize {
    const GRID_SIZE: usize = 1000;
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    for instruction in instructions {
        let covered_points = instruction.get_covered_points(true);

        for point in covered_points {
            grid[point.y][point.x] += 1;
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|&n| *n >= 2).count())
        .sum::<usize>()
}

fn get_input() -> Vec<Instruction> {
    let raw_input = include_str!("input.txt");
    let input = raw_input.lines().map(|s| s.trim());

    input
        .map(|s| {
            let points = s.split(" -> ").collect::<Vec<_>>();

            let start_values = points[0]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let start = Point {
                x: start_values[0],
                y: start_values[1],
            };

            let end_values = points[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let end = Point {
                x: end_values[0],
                y: end_values[1],
            };

            Instruction { start, end }
        })
        .collect::<Vec<_>>()
}

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

struct Instruction {
    start: Point,
    end: Point,
}

impl Instruction {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn get_covered_points(&self, include_diag: bool) -> Vec<Point> {
        let mut covered_points = vec![];

        if self.is_horizontal() {
            let smallest_x = if self.start.x >= self.end.x {
                self.end.x
            } else {
                self.start.x
            };

            let largest_x = if self.start.x <= self.end.x {
                self.end.x
            } else {
                self.start.x
            };

            for i in smallest_x..=largest_x {
                covered_points.push(Point {
                    x: i,
                    y: self.start.y,
                });
            }
        }

        if self.is_vertical() {
            let smallest_y = if self.start.y >= self.end.y {
                self.end.y
            } else {
                self.start.y
            };

            let largest_y = if self.start.y <= self.end.y {
                self.end.y
            } else {
                self.start.y
            };

            for i in smallest_y..=largest_y {
                covered_points.push(Point {
                    x: self.start.x,
                    y: i,
                });
            }
        }

        if include_diag && !self.is_vertical() && !self.is_horizontal() {
            let smallest_x_point = if self.start.x >= self.end.x {
                self.end
            } else {
                self.start
            };

            let largest_x_point = if self.start.x <= self.end.x {
                self.end
            } else {
                self.start
            };

            if smallest_x_point.y < largest_x_point.y {
                let diff = largest_x_point.y - smallest_x_point.y + 1;
                for i in 0..diff {
                    covered_points.push(Point {
                        x: smallest_x_point.x + i,
                        y: smallest_x_point.y + i,
                    });
                }
            } else {
                let diff = smallest_x_point.y - largest_x_point.y + 1;
                for i in 0..diff {
                    covered_points.push(Point {
                        x: largest_x_point.x - i,
                        y: largest_x_point.y + i,
                    });
                }
            }
        }

        covered_points
    }
}
