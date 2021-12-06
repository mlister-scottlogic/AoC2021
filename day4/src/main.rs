fn main() {
    println!("Hello, world!");
}

struct BingoSquare {
    value: i32,
    marked: bool,
}

struct BingoCard {
    squares: [[BingoSquare; 5]; 5],
}

fn is_bingo(card: BingoCard) -> bool {
    for row in card.squares.iter() {
        let un_marked = row.iter().filter(|s| !s.marked).collect::<Vec<_>>();

        if un_marked.len() == 0 {
            return true;
        }
    }

    for i in 0..card.squares.len() {
        let mut column: Vec<bool> = vec![];

        for j in 0..card.squares.len() {
            column.push(card.squares[i][j].marked);
        }

        let un_marked = column.into_iter().filter(|&s| !s).collect::<Vec<_>>();

        if un_marked.len() == 0 {
            return true;
        }
    }

    false
}

fn winning_card_value(card: BingoCard) -> i32 {
    card.squares
        .iter()
        .map(|row| {
            let un_marked = row.iter().filter(|s| !s.marked).collect::<Vec<_>>();
            un_marked.iter().map(|c| c.value).sum::<i32>()
        })
        .sum()
}
