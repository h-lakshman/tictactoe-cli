# Tic Tac Toe CLI Game

Welcome to the **Tic Tac Toe CLI Game**! This is a simple, interactive game that can be played directly from your terminal.

## Features

- **Two-player mode**: Play against a friend.
- **AI mode**: Choose between Easy and Hard difficulty levels to challenge yourself.
- **Clear and intuitive gameplay**: A simple 3x3 grid for moves and easy-to-follow instructions.

---

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/) installed on your system.
  
### Steps
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-folder>
   ```

2. Build the game using Cargo:
   ```bash
   cargo build --release
   ```

3. Run the executable:
   ```bash
   cargo run
   ```

---

## How to Play

### Starting the Game
1. Run the game using the command:
   ```bash
   cargo run
   ```

2. Follow the on-screen instructions to:
   - Select a game mode (Play vs AI or Two-player).
   - If playing vs AI, choose the difficulty (Easy or Hard).

### Gameplay
- The game board is represented as a 3x3 grid:
  ```
   1 | 2 | 3
  ---+---+---
   4 | 5 | 6
  ---+---+---
   7 | 8 | 9
  ```

- Players take turns to enter the number corresponding to the cell where they want to place their marker (`X` or `O`).

- The game ends when:
  - A player wins by getting three markers in a row (horizontal, vertical, or diagonal).
  - The board is full, resulting in a draw.

### AI Mode
- In Easy mode, the AI makes random moves.
- In Hard mode, the AI uses advanced strategies to make optimal moves.

---

## Example Game
```
Welcome to Tic Tac Toe
To play the game, select a mode:
1. Play vs AI (Type 1)
2. Multiplayer (Type 2)

> 1

Select Difficulty:
1. Easy (Type 1)
2. Hard (Type 2)

> 2

Player X's turn
1 | 2 | 3
---+---+---
4 | 5 | 6
---+---+---
7 | 8 | 9

Enter your move (1-9):
> 5

Player O's turn...
...
```

---

## Dependencies

This game uses the following Rust crates:
- **clearscreen**: For clearing the terminal screen.
- **terminal_size**: To adjust the game layout based on the terminal dimensions.
- **rand**: For generating random moves in Easy AI mode.

---


