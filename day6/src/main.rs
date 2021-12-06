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
    let mut previous_gen = FishLifeCycle { lifetime: [0; 9] };

    for l in lanterns {
        previous_gen.lifetime[l] += 1
    }

    for _i in 0..generations {
        println! {"Generation {}", _i};

        let mut new_life_cycle = [0; 9];

        // Decrement all by 1 life
        for j in 0..8 {
            new_life_cycle[j] = previous_gen.lifetime[j + 1];
        }

        // Add new fishies at 8 life
        new_life_cycle[8] += previous_gen.lifetime[0];

        // Set 0 lifetime fishies to 6 life
        new_life_cycle[6] += previous_gen.lifetime[0];

        previous_gen = FishLifeCycle {
            lifetime: new_life_cycle,
        };
    }

    previous_gen.lifetime.iter().sum()
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
