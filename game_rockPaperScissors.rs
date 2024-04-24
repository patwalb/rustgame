extern crate rand;
use std::io;

fn main() {
    loop {
        let player_move = get_player_move();

        if is_valid_move(&player_move) {
            play_round(&player_move);
        } else {
            println!("Invalid move. Please enter rock, paper, or scissors.");
        }
    }
}

fn get_player_move() -> String {
    println!("Please enter your move (rock, paper, or scissors):");

    let mut player_move = String::new();
    io::stdin().read_line(&mut player_move).expect("Failed to read line");

    player_move.trim().to_lowercase()
}

fn is_valid_move(player_move: &str) -> bool {
    player_move == "rock" || player_move == "paper" || player_move == "scissors"
}

fn play_round(player_move: &str) {
    let computer_move = get_computer_move();
    println!("Computer's move: {}", computer_move);

    let result = get_game_result(player_move, &computer_move);
    println!("Result: {}", result);
}

fn get_computer_move() -> String {
    let moves = vec!["rock", "paper", "scissors"];
    let random_index = rand::random::<usize>() % moves.len();
    String::from(moves[random_index])
}

fn get_game_result<'a>(player_move: &'a str, computer_move: &'a str) -> &'a str {
    if player_move == computer_move {
        "It's a tie!"
    } else if (player_move == "rock" && computer_move == "scissors")
        || (player_move == "paper" && computer_move == "rock")
        || (player_move == "scissors" && computer_move == "paper")
    {
        "You win!"
    } else {
        "Computer wins!"
    }
}