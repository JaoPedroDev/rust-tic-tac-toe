use std::io::{self, Write};

fn main() {
    // Create an 3x3 2d array
    let mut board: [[char; 3]; 3] = [['⬜';3];3];

    loop {
        player_move(&mut board, '❌', 1);
        render_board(&board);
        did_someone_win(&board);
        player_move(&mut board, '⭕', 2);
        render_board(&board);
        did_someone_win(&board);
    }
}

fn render_board(arr: &[[char; 3]; 3]) {
    println!();
    for (_i, row) in arr.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {
            print!("{}", col);
            io::stdout().flush().unwrap(); // To "print!" macro correct
        }
        println!(); // New line after each row
    }
    println!();
}
// Get board array, simbol char and number u8 for a player
// Call one time for each player
fn player_move(arr: &mut [[char; 3]; 3], simbol: char, number: u8) {
    loop {
        let mut position = String::new();
        let coord: [usize; 2];

        println!("PLAYER {}", number);
        print!("Type position number(1 - 9): ");
        io::stdout().flush().unwrap(); // To "print!" macro correct
        io::stdin()
            .read_line(&mut position)
            .expect("Error handling writing");

        // Parse String to u8
        let position: u8 = position
            .trim()
            .parse()
            .expect("Given row is not a number or out of range(1 - 9)");

        // Translate the given position into the corresponding coordinate
        match position {
            1 => coord = [2, 0],
            2 => coord = [2, 1],
            3 => coord = [2, 2],
            4 => coord = [1, 0],
            5 => coord = [1, 1],
            6 => coord = [1, 2],
            7 => coord = [0, 0],
            8 => coord = [0, 1],
            9 => coord = [0, 2],
            _ => {
                panic!()
            }
        }

        // If position is diferent to ' ' then is not a valid move
        if arr[coord[0]][coord[1]] != '⬜' {
            println!("\nInvalid place! Try again...\n");
            continue;
        };

        arr[coord[0]][coord[1]] = simbol;

        break;
    }
}

fn did_someone_win(arr: &[[char;3];3]) {
    // Hardcoded winning patterns OwO)b
    let col_0 = [arr[0][0], arr[1][0], arr[2][0]];
    let col_1 = [arr[0][1], arr[1][1], arr[2][1]];
    let col_2 = [arr[0][2], arr[1][2], arr[2][2]];
    let left_diag = [arr[0][0], arr[1][1], arr[2][2]];
    let right_diag = [arr[0][2], arr[1][1], arr[2][0]];

    let winning_patterns: [[char; 3]; 8] = [
        arr[0], arr[1], arr[2], col_0, col_1, col_2, left_diag, right_diag
        ];

    for (_i, pattern) in winning_patterns.iter().enumerate() {
        if (pattern.iter().min() == pattern.iter().max()) && !(pattern.contains(&'⬜')) {
            println!("###########");
            println!("SOMEONE WON");
            println!("###########");
        }
    }
}
