use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const BOARD_SIZE: usize = 3;

// 2D array - 3x3
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

// Function to initialize the board
fn initialize_board() -> Board {
    [[' '; BOARD_SIZE]; BOARD_SIZE]
}

// Function to print the current state of the board
fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

// Function to get the player's move
fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop {
        println!("Player {} input (row,col): ", current_player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let coordinates: Vec<usize> = input
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .collect();

        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
            }
        }
        println!("Invalid Input. Please enter row and column numbers within the range (0-2) and ensure the cell is empty.");
    }
}

// Function to check if the current player has won
fn check_winner(current_player: char, board: &Board) -> bool {
    // Check rows
    for row in 0..BOARD_SIZE {
        if board[row][0] == current_player && board[row][1] == current_player && board[row][2] == current_player {
            return true;
        }
    }
    
    // Check columns
    for col in 0..BOARD_SIZE {
        if board[0][col] == current_player && board[1][col] == current_player && board[2][col] == current_player {
            return true;
        }
    }
    
    // Check diagonals
    if (board[0][0] == current_player && board[1][1] == current_player && board[2][2] == current_player) ||
       (board[0][2] == current_player && board[1][1] == current_player && board[2][0] == current_player) {
        return true;
    }
    
    false
}

// Function to check if the game is a draw
fn check_draw(board: &Board) -> bool {
    for row in board {
        for cell in row {
            if *cell == ' ' {
                return false;
            }
        }
    }
    true
}

// Function to play the game
fn play_game() {
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;
    
    loop {
        println!("Current Board:");
        print_board(&board);

        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;
        
        if check_winner(current_player, &board) {
            println!("Player {} is the winner!", current_player);
            print_board(&board);
            break;
        }

        if check_draw(&board) {
            println!("The game is a draw!");
            print_board(&board);
            break;
        }

        // Switch players
        current_player = if current_player == PLAYER_X { PLAYER_O } else { PLAYER_X };
    }
}

// Function to start the game
pub fn tictactoe() {
    println!("Welcome to the Katam Kutta Game!");
    play_game();
}
