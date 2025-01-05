use crate::helper::center_text;
use rand::Rng;
use std::io;
use terminal_size::{terminal_size, Width};

#[derive(PartialEq)]
enum GameMode {
    EasyAi,
    HardAi,
    TwoPlayer,
}

pub struct GameBoard {
    board: [[char; 5]; 5],
    game_over: bool,
    moves_count: u8,
    player_x_turn: bool,
    game_mode: Option<GameMode>,
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
            game_mode: None,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        center_text("Welcome to Tic Tac Toe");
        center_text("To play the game, Select a mode");
        center_text("1. Play vs AI, Type 1");
        center_text("2. Multiplayer, Type 2");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                center_text("Select Difficulty");
                center_text("1. Easy, Type 1");
                center_text("2. Hard, Type 2");
                input.clear(); //doubt
                io::stdin().read_line(&mut input)?;
                match input.trim() {
                    "1" => {
                        self.game_mode = Some(GameMode::EasyAi);
                        self.play_game()?;
                    }
                    "2" => {
                        self.game_mode = Some(GameMode::HardAi);
                        self.play_game()?;
                    }
                    _ => return Err("Invalid difficulty selection".into()), //doubt
                }
            }
            "2" => {
                self.game_mode = Some(GameMode::TwoPlayer);
                self.play_game()?;
            }
            _ => return Err("Invalid game mode selection".into()),
        }

        Ok(())
    }
    fn play_game(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure game mode is set before starting
        //  let game_mode = self.game_mode.as_ref()
        //  .ok_or("Game mode not set")?;
        while !self.game_over && self.moves_count < 9 {
            clearscreen::clear()?;
            center_text("Tic Tac Toe");
            self.draw_board();

            // if *game_mode == GameMode::TwoPlayer ||
            //    (*game_mode != GameMode::TwoPlayer && self.player_x_turn)
            if self.game_mode == Some(GameMode::TwoPlayer)
                || (self.game_mode != Some(GameMode::TwoPlayer) && self.player_x_turn)
            //doubt
            {
                center_text(if self.player_x_turn {
                    "Player X's turn"
                } else {
                    "Player O's turn"
                });
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let position: u8 = input.trim().parse()?;

                if let Some((row, col)) = self.get_coordinates(position) {
                    if !self.make_move(row, col) {
                        center_text("Invalid move! Try again.");
                        continue;
                    }
                } else {
                    center_text("Invalid input! Please enter a number between 1 and 9.");
                    continue;
                }
            } else {
                match self.game_mode {
                    Some(GameMode::EasyAi) => self.ai_move_easy()?,
                    Some(GameMode::HardAi) => self.ai_move_hard()?,
                    _ => unreachable!(), //hard
                }
            }

            self.moves_count += 1;

            if let Some(winner) = self.check_winner() {
                self.game_over = true;
                self.draw_board();
                center_text(&format!("Player {} wins!", winner));
                return Ok(());
            }

            self.player_x_turn = !self.player_x_turn;
        }

        if !self.game_over {
            center_text("Game Draw!");
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

    fn get_coordinates(&self, input: u8) -> Option<(usize, usize)> {
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
        for i in [0, 2, 4] {
            if self.board[i][0] == self.board[i][2] && self.board[i][2] == self.board[i][4] {
                return Some(self.board[i][0]);
            }
        }
        for i in [0, 2, 4] {
            if self.board[0][i] == self.board[2][i] && self.board[2][i] == self.board[4][i] {
                return Some(self.board[0][i]);
            }
        }
        if self.board[0][0] == self.board[2][2] && self.board[2][2] == self.board[4][4] {
            return Some(self.board[0][0]);
        }
        if self.board[0][4] == self.board[2][2] && self.board[2][2] == self.board[4][0] {
            return Some(self.board[0][4]);
        }
        None
    }

    fn make_move(&mut self, row: usize, column: usize) -> bool {
        if self.board[row][column] != 'X' && self.board[row][column] != 'O' {
            self.board[row][column] = if self.player_x_turn { 'X' } else { 'O' };
            true
        } else {
            false
        }
    }

    fn get_valid_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for i in [0, 2, 4] {
            for j in [0, 2, 4] {
                if self.board[i][j] != 'X' && self.board[i][j] != 'O' {
                    moves.push((i, j));
                }
            }
        }
        moves
    }

    fn ai_move_easy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let valid_moves = self.get_valid_moves();
        if valid_moves.is_empty() {
            return Err("No valid moves available".into());
        }
        let idx = rand::thread_rng().gen_range(0..valid_moves.len()); //doubt
        let (row, col) = valid_moves[idx];
        self.make_move(row, col);
        Ok(())
    }

    fn ai_move_hard(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let valid_moves = self.get_valid_moves();
        if valid_moves.is_empty() {
            return Err("No valid moves available".into());
        }

        // Simple implementation: try to block player's winning move or take center
        let center = (2, 2);
        if valid_moves.contains(&center) {
            self.make_move(center.0, center.1);
            return Ok(());
        }

        // If no strategic move, make a random move
        let idx = rand::thread_rng().gen_range(0..valid_moves.len());
        let (row, col) = valid_moves[idx];
        self.make_move(row, col);
        Ok(())
    }
}
