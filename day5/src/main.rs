use std::thread;

const STACK_SIZE: usize = 16 * 1024 * 1024;

fn run() {
    let instructions = get_input();

    println!("{}", part1(&instructions));
}

fn main() {
    // I'm arguing memory is not a big bottleneck on any PC I'll run this solution on
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

fn part1(instructions: &Vec<Instruction>) -> usize {
    const GRID_SIZE: usize = 1000;
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    for instruction in instructions {
        // println!(
        //     "Start x{} y{}  End x{} y{}",
        //     instruction.start.x, instruction.start.y, instruction.end.x, instruction.end.y
        // );

        let covered_points = instruction.get_covered_points();

        for point in covered_points {
            grid[point.y][point.x] += 1;
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|&n| *n >= 2).count())
        .sum::<usize>()
}

fn get_input() -> Vec<Instruction> {
    let raw_input = include_str!("testInput.txt");
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

    fn get_covered_points(&self) -> Vec<Point> {
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
                })
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
                })
            }
        }

        covered_points
    }
}
