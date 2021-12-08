use array_tool::vec::Uniq;
use itertools::Itertools;

fn main() {
    let input = get_input();

    println!("Day 8 Part 1 {}", part1(&input));

    println!("Day 8 Part 2 {:?}", part2(&input));
}

fn part2(input: &Vec<SignalLine>) -> usize {
    input.iter().fold(0, |overall_total, line| {
        let decoded_input = decode_input(&line.input);

        let total = line.output.iter().fold(0, |total, o| {
            let n = decoded_input.iter().position(|s| s == o).unwrap();
            total * 10 + n
        });

        overall_total + total
    })
}

fn decode_input(input: &Vec<Vec<char>>) -> Vec<String> {
    let all_chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    // There is only 1 value with 2 characters
    let one_chars = input.iter().filter(|c| c.len() == 2).collect::<Vec<_>>()[0];

    // println!("One {:?}", one_chars);

    // There is only 1 group of 7, which includes the group of 2 + 1
    let seven_chars = input.iter().filter(|c| c.len() == 3).collect::<Vec<_>>()[0];
    let top_char = seven_chars.clone().uniq(one_chars.clone())[0];
    // println!("Top {}", top_char);

    // Get chars in the 4 character
    let four_chars = input.iter().filter(|c| c.len() == 4).collect::<Vec<_>>()[0];
    // println!("Four {:?}", four_chars);

    // We know that 9 is made up of the 4 + top, and the 1 remaining
    let mut almost_9 = four_chars.clone();
    almost_9.extend(vec![top_char]);

    // println!("Almost 9 {:?}", almost_9);

    let nine_chars = input
        .iter()
        .filter(|c| c.len() == 6)
        .filter(|s| (*s).uniq(almost_9.clone()).len() == 1)
        .collect::<Vec<_>>()[0];
    let bottom_char = nine_chars.uniq(almost_9.clone())[0];

    // println!("Bottom {}", bottom_char);

    // bottom left must be the 1 character not in 9
    let bottom_left_char = all_chars.clone().uniq(nine_chars.clone())[0];

    // 0 is rhs, top, bottom, and bottom left
    let mut almost_0 = one_chars.clone();
    almost_0.extend(vec![top_char, bottom_char, bottom_left_char]);

    let zero_chars = input
        .iter()
        .filter(|c| c.len() == 6)
        .filter(|s| (*s).uniq(almost_0.clone()).len() == 1)
        .collect::<Vec<_>>()[0];

    let top_left_char = zero_chars.uniq(almost_0.clone())[0];

    // println!("Top left {:?}", top_left_char);

    let mut almost_4 = one_chars.clone();
    almost_4.push(top_left_char);

    let middle_char = four_chars.clone().uniq(almost_4)[0];
    // println!("Middle {:?}", middle_char);

    let mut unique_to_3 = one_chars.clone();
    unique_to_3.extend(vec![top_char, middle_char, bottom_char]);

    // println!("Unique to 3 {:?}", unique_to_3);

    let three_chars = input
        .iter()
        .filter(|c| c.len() == 5)
        .filter(|s| s.uniq(unique_to_3.clone()).len() == 0)
        .collect::<Vec<_>>()[0];

    let unique_to_2 = vec![top_char, middle_char, bottom_left_char, bottom_char];
    let two_chars = input
        .iter()
        .filter(|c| c.len() == 5)
        .filter(|s| s.uniq(unique_to_2.clone()).len() == 1)
        .collect::<Vec<_>>()[0];

    let unique_to_5 = vec![top_char, middle_char, top_left_char, bottom_char];
    let five_chars = input
        .iter()
        .filter(|c| c.len() == 5)
        .filter(|s| s.uniq(unique_to_5.clone()).len() == 1)
        .collect::<Vec<_>>()[0];

    let unique_to_6 = vec![
        top_char,
        top_left_char,
        bottom_left_char,
        middle_char,
        bottom_char,
    ];
    let six_chars = input
        .iter()
        .filter(|c| c.len() == 6)
        .filter(|s| s.uniq(unique_to_6.clone()).len() == 1)
        .collect::<Vec<_>>()[0];

    vec![
        zero_chars,
        one_chars,
        two_chars,
        three_chars,
        four_chars,
        five_chars,
        six_chars,
        seven_chars,
        &all_chars,
        nine_chars,
    ]
    .iter()
    .map(|c| c.iter().collect::<String>())
    .collect::<Vec<_>>()
}

fn part1(input: &Vec<SignalLine>) -> u32 {
    let mut one_count = 0;
    let mut four_count = 0;
    let mut seven_count = 0;
    let mut eight_count = 0;

    for l in input {
        for s in l.output.iter() {
            match s.chars().count() {
                2 => one_count += 1,
                4 => four_count += 1,
                3 => seven_count += 1,
                7 => eight_count += 1,
                _ => (),
            }
        }
    }

    one_count + four_count + seven_count + eight_count
}

struct SignalLine {
    input: Vec<Vec<char>>,
    output: Vec<String>,
}

fn get_input() -> Vec<SignalLine> {
    let raw_input = include_str!("input.txt");
    let input_lines = raw_input.lines();

    input_lines
        .map(|s| {
            let splits = s.split('|').collect::<Vec<_>>();

            let input = splits[0]
                .split_whitespace()
                .map(|s| s.chars().sorted().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let output = splits[1]
                .split_whitespace()
                .map(|s| s.chars().sorted().collect::<String>())
                .collect::<Vec<_>>();

            SignalLine { input, output }
        })
        .collect::<Vec<_>>()
}
