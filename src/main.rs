use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut board: [[char; 5]; 5] = [
        ['1', '|', '2', '|', '3'],
        ['-', '+', '-', '+', '-'],
        ['4', '|', '5', '|', '6'],
        ['-', '+', '-', '+', '-'],
        ['7', '|', '8', '|', '9'],
    ];
    let mut count: u8 = 0;
    let mut player_x_turn: bool = true;
    println!("Welcome to TIC TAC TOE");
    draw_board(&board);
    loop {
        let mut prompt: String = String::new();
        count += 1;
        if count == 10 {
            println!("Match draw");
            break;
        }
        if player_x_turn {
            println!("Player X,make a move");
        } else {
            println!("Player O,make a move");
        }
        io::stdin()
            .read_line(&mut prompt)
            .expect("Please choose the correct input");
        let prompt: u8 = prompt.trim().parse()?;
        let (row, column): (usize, usize) = get_cordinates(prompt).unwrap();
        board[row][column] = if player_x_turn { 'X' } else { 'O' };
        draw_board(&mut board);
        if let Some(winner) = check_winner(&board) {
            println!("Player {winner} wins!!");
            break;
        }
        player_x_turn = !player_x_turn;
    }
    Ok(())
}
fn draw_board(board: &[[char; 5]; 5]) {
    for row in 0..5 {
        for column in 0..5 {
            print!("{}", board[row][column]);
        }
        print!("\n")
    }
}
fn get_cordinates(input: u8) -> Option<(usize, usize)> {
    match input {
        1 => Some((0, 0)),
        2 => Some((0, 2)),
        3 => Some((0, 4)),
        4 => Some((2, 0)),
        5 => Some((2, 2)),
        6 => Some((2, 4)),
        7 => Some((4, 0)),
        8 => Some((4, 2)),
        9 => Some((4, 4)),
        _ => None,
    }
}

fn check_winner(board: &[[char; 5]; 5]) -> Option<char> {
    if board[0][0] == board[0][2] && board[0][2] == board[0][4] {
        Some(board[0][0])
    } else if board[2][0] == board[2][2] && board[2][2] == board[2][4] {
        Some(board[2][0])
    } else if board[4][0] == board[4][2] && board[4][2] == board[4][4] {
        Some(board[4][0])
    } else if board[0][0] == board[2][0] && board[2][0] == board[4][0] {
        Some(board[0][0])
    } else if board[0][2] == board[2][2] && board[2][2] == board[4][2] {
        Some(board[0][2])
    } else if board[0][4] == board[2][4] && board[2][4] == board[4][4] {
        Some(board[0][4])
    } else if board[0][0] == board[2][2] && board[2][2] == board[4][4] {
        Some(board[0][0])
    } else if board[0][4] == board[2][2] && board[2][2] == board[4][0] {
        Some(board[0][4])
    } else {
        None
    }
}
