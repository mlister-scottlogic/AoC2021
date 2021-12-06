fn main() {
    let input = include_str!("input.txt");

    println!("Day 6 part 1 {}", lantern_fish_naive(input, 80));
    // println!("Day 6 part 2 {}", lantern_fish_naive(input, 256));
}

fn lantern_fish_naive(initial_seed: &str, generations: usize) -> usize {
    let mut lanterns = initial_seed
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    for _i in 0..generations {
        println! {"Generation {}", _i};

        let mut num_new_lanterns: usize = 0;

        for l in &mut lanterns {
            if *l == 0 as u8 {
                num_new_lanterns += 1;
                *l = 6 as u8;
            } else {
                *l -= 1;
            }
        }

        let new_fish: Vec<u8> = vec![8; num_new_lanterns];
        lanterns.extend(new_fish);
        // println! {"After {} day {:?}", _i, lanterns};
    }

    lanterns.len()
}
