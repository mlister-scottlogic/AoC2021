use std::convert::TryInto;

fn main() {
    let part1 = part1();

    println!("Day 4 part 1: {}", part1);
}

fn part1() -> i32 {
    let day_4_input = include_str!("input.txt");
    let input = day_4_input.lines().collect::<Vec<_>>();

    let bingo_numbers = input[0]
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut row_counter = 2;
    let mut bingo_cards = vec![];

    while row_counter < input.len() {
        let mut array: [[BingoSquare; 5]; 5] = Default::default();

        for i in 0..5 {
            let nums = input[row_counter + i]
                .split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .map(|n| BingoSquare {
                    value: n,
                    marked: false,
                })
                .collect::<Vec<_>>();

            array[i] = nums.try_into().unwrap_or_else(|v: Vec<_>| {
                panic!("Expected a Vec of length {} but it was {}", 5, v.len())
            })
        }

        bingo_cards.push(BingoCard { squares: array });

        row_counter += 6
    }

    println!("{}", bingo_cards.iter_mut().len());
    println!("{:?}", bingo_numbers);

    for called_number in bingo_numbers {
        // bingo cards .try_mark
        for card in bingo_cards.iter_mut() {
            card.try_mark_card(called_number);
        }

        for card in bingo_cards.iter_mut() {
            if is_bingo(&card) {
                return winning_card_value(&card) * called_number;
            }
        }

        // bingo cards  find any bingos
        // if a bingo return card .total
    }

    panic!("No bingos found")
}

// fn get_input() -> (Vec<i32>, Vec<BingoCard>) {
//     let day_4_input = include_str!("input.txt");
//     let input = day_4_input.lines().collect::<Vec<_>>();

//     let bingo_numbers = input[0]
//         .split(',')
//         .map(|n| n.parse().unwrap())
//         .collect::<Vec<_>>();

//     let mut row_counter = 2;
//     let mut bingo_cards = vec![];

//     while row_counter < input.len() {
//         let mut array: [[BingoSquare; 5]; 5] = Default::default();

//         for i in 0..5 {
//             let nums = input[row_counter + i]
//                 .split(' ')
//                 .map(|s| s.trim())
//                 .filter(|s| !s.is_empty())
//                 .map(|n| n.parse::<i32>().unwrap())
//                 .map(|n| BingoSquare {
//                     value: n,
//                     marked: false,
//                 })
//                 .collect::<Vec<_>>();

//             array[i] = nums.try_into().unwrap_or_else(|v: Vec<_>| {
//                 panic!("Expected a Vec of length {} but it was {}", 5, v.len())
//             })
//         }

//         bingo_cards.push(BingoCard { squares: array });

//         row_counter += 6
//     }

//     (bingo_numbers, bingo_cards)
// }

#[derive(Default, Copy, Clone)]
struct BingoSquare {
    value: i32,
    marked: bool,
}

struct BingoCard {
    squares: [[BingoSquare; 5]; 5],
}

impl BingoCard {
    fn try_mark_card(&mut self, called_number: i32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.squares[i][j].value == called_number {
                    self.squares[i][j] = BingoSquare {
                        value: called_number,
                        marked: true,
                    };
                }
            }
        }
    }
}

fn is_bingo(card: &BingoCard) -> bool {
    for row in card.squares.iter() {
        let un_marked = row.iter().filter(|s| !s.marked).collect::<Vec<_>>();

        if un_marked.len() == 0 {
            return true;
        }
    }

    for i in 0..card.squares.len() {
        let mut column: Vec<bool> = vec![];

        for j in 0..card.squares.len() {
            column.push(card.squares[j][i].marked);
        }

        let un_marked = column.into_iter().filter(|&s| !s).collect::<Vec<_>>();

        if un_marked.len() == 0 {
            return true;
        }
    }

    false
}

fn winning_card_value(card: &BingoCard) -> i32 {
    card.squares
        .iter()
        .map(|row| {
            let un_marked = row.iter().filter(|s| !s.marked).collect::<Vec<_>>();
            un_marked.iter().map(|c| c.value).sum::<i32>()
        })
        .sum()
}
