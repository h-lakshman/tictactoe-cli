use crate::helper::center_text;
use std::io;
use terminal_size::{terminal_size, Width};
pub struct GameBoard {
    board: [[char; 5]; 5],
    game_over: bool,
    moves_count: u8,
    player_x_turn: bool,
}

impl GameBoard {
    pub fn new() -> Self {
        GameBoard {
            board: [
                ['1', '|', '2', '|', '3'],
                ['-', '+', '-', '+', '-'],
                ['4', '|', '5', '|', '6'],
                ['-', '+', '-', '+', '-'],
                ['7', '|', '8', '|', '9'],
            ],
            game_over: false,
            moves_count: 0,
            player_x_turn: true,
        }
    }
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            clearscreen::clear().expect("failed to clear screen");
            center_text("Welcome to Tic Tac Toe");
            self.draw_board();
            self.moves_count += 1;
            if self.moves_count == 10 {
                center_text("Match draw");
                break;
            }
            if self.player_x_turn {
                center_text("Player X,make a move");
            } else {
                center_text("Player O,make a move");
            }
            let (mut row, mut column): (usize, usize);
            loop {
                let mut prompt: String = String::new();
                io::stdin()
                    .read_line(&mut prompt)
                    .expect("Please choose the correct input");
                let prompt: u8 = prompt.trim().parse()?;
                if let Some((r, c)) = self.get_cordinates(prompt) {
                    (row, column) = (r, c)
                } else {
                    center_text("Please type a value from 1-9 only");
                    continue;
                }
                if self.board[row][column] != 'X' && self.board[row][column] != 'O' {
                    break;
                } else {
                    center_text("This is not valid cell,Please observe the valid cells and enter a number from 1-9:- ");
                }
            }
            self.board[row][column] = if self.player_x_turn { 'X' } else { 'O' };
            self.draw_board();
            if let Some(winner) = self.check_winner() {
                center_text(&format!("Player {winner} wins!!"));
                break;
            }
            self.player_x_turn = !self.player_x_turn;
        }
        Ok(())
    }
    fn draw_board(&self) {
        let terminal_width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80
        };
        let board_width = 13;

        let padding = (terminal_width - board_width) / 2;

        println!("{:padding$}+-----------+", "", padding = padding);

        for row in 0..5 {
            print!("{:padding$}| ", "", padding = padding);
            for column in 0..5 {
                print!("{}", self.board[row][column]);
                if column < 4 {
                    print!(" ");
                }
            }
            println!(" |");
        }

        println!("{:padding$}+-----------+", "", padding = padding);
    }
    fn get_cordinates(&self, input: u8) -> Option<(usize, usize)> {
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
    fn check_winner(&self) -> Option<char> {
        if self.board[0][0] == self.board[0][2] && self.board[0][2] == self.board[0][4] {
            Some(self.board[0][0])
        } else if self.board[2][0] == self.board[2][2] && self.board[2][2] == self.board[2][4] {
            Some(self.board[2][0])
        } else if self.board[4][0] == self.board[4][2] && self.board[4][2] == self.board[4][4] {
            Some(self.board[4][0])
        } else if self.board[0][0] == self.board[2][0] && self.board[2][0] == self.board[4][0] {
            Some(self.board[0][0])
        } else if self.board[0][2] == self.board[2][2] && self.board[2][2] == self.board[4][2] {
            Some(self.board[0][2])
        } else if self.board[0][4] == self.board[2][4] && self.board[2][4] == self.board[4][4] {
            Some(self.board[0][4])
        } else if self.board[0][0] == self.board[2][2] && self.board[2][2] == self.board[4][4] {
            Some(self.board[0][0])
        } else if self.board[0][4] == self.board[2][2] && self.board[2][2] == self.board[4][0] {
            Some(self.board[0][4])
        } else {
            None
        }
    }
}
