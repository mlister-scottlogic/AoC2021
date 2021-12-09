fn main() {
    let input = include_str!("input.txt");

    println!("Day 6 part 1 {}", lantern_fish_naive(input, 80));
    println!("Day 6 part 2 {}", latern_fish_quicker_hopefully(input, 256));
}

struct FishLifeCycle {
    lifetime: [usize; 9],
}

fn latern_fish_quicker_hopefully(initial_seed: &str, generations: usize) -> usize {
    let lanterns = initial_seed
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut fish_lifetime = FishLifeCycle { lifetime: [0; 9] };

    for l in lanterns {
        fish_lifetime.lifetime[l] += 1
    }

    for _i in 0..generations {
        fish_lifetime.lifetime.rotate_left(1);

        // Set 0 lifetime fishies to 6 life
        fish_lifetime.lifetime[6] += fish_lifetime.lifetime[8];
    }

    fish_lifetime.lifetime.iter().sum()
}

fn lantern_fish_naive(initial_seed: &str, generations: usize) -> usize {
    let mut lanterns = initial_seed
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    for _i in 0..generations {
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
    }

    lanterns.len()
}
