use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use csv::Writer;

fn check_winner(current_player: i32, player1: &mut i32, player2: &mut i32) {
    if current_player % 2 == 0 {
        println!("Player 2 wins!");
        *player2 += 1;
    } else {
        println!("Player 1 wins!");
        *player1 += 1;
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


fn log_move(writer: &mut Writer<File>, player: &str, row: usize, column: usize) -> Result<(), Box<dyn Error>> {
    writer.write_record(&[player, &row.to_string(), &column.to_string()])?;
    writer.flush()?;
    Ok(())
}

fn log_score(writer: &mut Writer<File>, player1_score: i32, player2_score: i32) -> Result<(), Box<dyn Error>> {
    writer.write_record(&["Scores", &player1_score.to_string(), &player2_score.to_string()])?;
    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::create("game_log.csv")?;
    let mut writer = Writer::from_writer(file);

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
                    println!("Invalid input! Please enter numbers between 0 and 2.");
                    continue;
                }
            },
            None => {
                println!("Invalid input! Please enter numbers between 0 and 2.");
                continue;
            }
        };

        let column: usize = match iter.next() {
            Some(value) => match value.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input! Please enter numbers between 0 and 2.");
                    continue;
                }
            },
            None => {
                println!("Invalid input! Please enter numbers between 0 and 2.");
                continue;
            }
        };

        if row >= 3 || column >= 3 {
            println!("Invalid input! Please enter numbers between 0 and 2.");
            continue;
        }

        if board[row][column] != ' ' {
            println!("Occupied. Please choose another one.");
            continue;
        }

        if current_player % 2 == 0 {
            board[row][column] = 'X';
        } else {
            board[row][column] = 'O';
        }

        log_move(&mut writer, if current_player % 2 == 0 { "Player 2" } else { "Player 1" }, row, column)?;

        print_board(&board);

        for i in 0..3 {
            if (board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != ' ')
                || (board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != ' ')
            {
                check_winner(current_player, &mut player1, &mut player2);
                log_score(&mut writer, player1, player2)?;
                println!("Player 1: {} | Player 2: {}", player1, player2);
                annulate(&mut board);
                current_player = 1;
                continue;
            }
        }

        if (board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != ' ')
            || (board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != ' ')
        {
            check_winner(current_player, &mut player1, &mut player2);
            log_score(&mut writer, player1, player2)?;
            println!("Player 1: {} | Player 2: {}", player1, player2);
            annulate(&mut board);
            current_player = 1;
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
            log_score(&mut writer, player1, player2)?;
            annulate(&mut board);
            current_player = 1;
            continue;
        }

        current_player += 1;
    }
}
