use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

const WINS_FILE: &str = "wins.json";
const INVALID_INPUT_MSG: &str = "Invalid input! Please enter numbers between 0 and 2.";
const OCCUPIED_MSG: &str = "Occupied. Please choose another one.";
const PLAYER_1_SYMBOL: char = 'O';
const PLAYER_2_SYMBOL: char = 'X';

#[derive(Serialize, Deserialize, Debug)]
struct Wins {
    X: i32,
    O: i32,
}

impl Wins {
    fn new() -> Self {
        Wins { X: 0, O: 0 }
    }
}

fn read_wins_from_file() -> Wins {
    if Path::new(WINS_FILE).exists() {
        let data = fs::read_to_string(WINS_FILE).expect("Unable to read file");
        serde_json::from_str(&data).expect("Unable to parse JSON")
    } else {
        Wins::new()
    }
}

fn write_wins_to_file(wins: &Wins) {
    let data = serde_json::to_string(wins).expect("Unable to serialize data");
    fs::write(WINS_FILE, data).expect("Unable to write file");
}

fn check_winner(current_player: i32, player1: &mut i32, player2: &mut i32, wins: &mut Wins) {
    if current_player % 2 == 0 {
        println!("Player 2 wins!");
        *player2 += 1;
        wins.X += 1;
    } else {
        println!("Player 1 wins!");
        *player1 += 1;
        wins.O += 1;
    }
}

fn annulate(board: &mut Vec<Vec<char>>) {
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            *cell = ' ';
        }
    }
}

fn print_board(board: &Vec<Vec<char>>) {
    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            print!("{} ", cell);
            if j < 2 {
                print!("| ");
            }
        }
        println!();
        if i < 2 {
            println!("---------");
        }
    }
}

fn print_score(player1: i32, player2: i32) {
    println!("Player 1: {} | Player 2: {}", player1, player2);
}

fn main() {
    let mut wins = read_wins_from_file();
    let mut player1 = 0;
    let mut player2 = 0;
    let mut board = vec![vec![' '; 3]; 3];
    let mut current_player = 1;

    loop {
        println!("Enter the field (row and column): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split_whitespace();
        let row: usize = match iter.next() {
            Some(value) => match value.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}", INVALID_INPUT_MSG);
                    continue;
                }
            },
            None => {
                println!("{}", INVALID_INPUT_MSG);
                continue;
            }
        };

        let column: usize = match iter.next() {
            Some(value) => match value.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}", INVALID_INPUT_MSG);
                    continue;
                }
            },
            None => {
                println!("{}", INVALID_INPUT_MSG);
                continue;
            }
        };

        if row >= 3 || column >= 3 {
            println!("{}", INVALID_INPUT_MSG);
            continue;
        }

        if board[row][column] != ' ' {
            println!("{}", OCCUPIED_MSG);
            continue;
        }

        if current_player % 2 == 0 {
            board[row][column] = PLAYER_2_SYMBOL;
        } else {
            board[row][column] = PLAYER_1_SYMBOL;
        }

        print_board(&board);

        for i in 0..3 {
            if (board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != ' ')
                || (board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != ' ')
            {
                check_winner(current_player, &mut player1, &mut player2, &mut wins);
                print_score(player1, player2);
                annulate(&mut board);
                current_player = 1;
                write_wins_to_file(&wins);
                continue;
            }
        }

        if (board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != ' ')
            || (board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != ' ')
        {
            check_winner(current_player, &mut player1, &mut player2, &mut wins);
            print_score(player1, player2);
            annulate(&mut board);
            current_player = 1;
            write_wins_to_file(&wins);
            continue;
        }

        let mut draw = true;
        for row in &board {
            for cell in row {
                if *cell == ' ' {
                    draw = false;
                }
            }
        }
        if draw {
            println!("Draw!");
            annulate(&mut board);
            current_player = 1;
            continue;
        }

        current_player += 1;
    }
}
