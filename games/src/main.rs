extern crate games as Games;
use std::io;

fn main() {
    /* ToDo: add an actually prompt here and some test cases */

    /*
    let mut tester = hangman::Hangman::new(
        "Joseph".to_string(),
        "Sam".to_string(),
        "hello was here!".to_string(),
    );*/

    // looping for options
    loop {

        println!("Play the game\n1.Hangman\n2.Tic Tac Toe\n3.Quit");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess{
            1 => hangman_game(),
            2 => tic_tac_game(),
            3 => break,
            _ => println!("Try again\n"),
        }
    }
}


// hangman options
fn hangman_game(){

    loop {

        let mut user_option = String::new();

        println!("\n\n1. 1 vs 1\n2. 1 vs CPU\n3. Quit");

        io::stdin().read_line(&mut user_option)
            .ok()
            .expect("failed to read line");

        let guess: u32 = match user_option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess{
            1 => one_on_one(),
            2 => one_vs_cpu(),
            3 => break,
            _ => println!("Try again\n"),
        }
    }

    println!("Thank you for playing hangman!\n");
}


fn one_on_one(){

    let mut phrase = String::new();
    let mut player1 = String::new();
    let mut player2 = String::new();

    println!("Player 1 Name : ");

    io::stdin().read_line(&mut player1)
        .ok()
        .expect("failed to read line");

    let remove_end = player1.trim_right().len();
    player1.truncate(remove_end);

    println!("Player 2 Name : ");
    io::stdin().read_line(&mut player2)
        .ok()
        .expect("failed to read line");

    let remove_end = player2.trim_right().len();
    player2.truncate(remove_end);

    println!("Player 2 enter a phrase : ");
    io::stdin().read_line(&mut phrase)
        .ok()
        .expect("failed to read line");

    /*
    let remove_end = phrase.trim_right().len();
    phrase.truncate(remove_end);
    let char_vec: Vec<char> = phrase.chars().collect();
    */
    let mut tester = Games::Hangman::new(player1, player2, phrase);

    while tester.get_score() != 2 && tester.get_score() !=1{

        let mut guess = String::new();
        println!("\nPlayer 1 enter a guess : ");
        io::stdin().read_line(&mut guess)
        .ok()
        .expect("failed to read line");

        let ch = guess.chars().next().unwrap();
        tester.make_move(ch);
        tester.show_board();
    }
}


fn one_vs_cpu(){
    unimplemented!();
}


fn tic_tac_game(){

    /*
    let mut player1 = String::new();
    let mut player2 = String::new();

    println!("Player 1 Name : ");

    io::stdin().read_line(&mut player1)
        .ok()
        .expect("failed to read line");

    let remove_end = player1.trim_right().len();
    player1.truncate(remove_end);

    println!("Player 2 Name : ");
    io::stdin().read_line(&mut player2)
        .ok()
        .expect("failed to read line");

    let remove_end = player2.trim_right().len();
    player2.truncate(remove_end);
    */
    let mut tic_game = Games::tictac::new();

    let mut count:u32 = 1;
    let mut tie  = true;
    while(tic_game.moves_left()){

        tic_game.tic_board();
        let mut player_move:u32 = 0;

        if count %2 != 0{
            player_move = 1;
        }else {
            player_move=2;
        }

        println!("Player {} enter move : ",player_move);

        let mut sq_move = String::new();
        io::stdin().read_line(&mut sq_move)
            .ok()
            .expect("failed to read line");

        let sq_move: u32 = match sq_move.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // move successful
        if tic_game.mark_player(sq_move,player_move){
            count += 1;

            let winner = tic_game.check_win();

            if  winner  == "[O]".to_string(){
                tie = false;
                println!("Player 1 has won!\n");
                break;
            }else if winner == "[X]".to_string(){
                tie = false;
                println!("Player 2 has won");
                break;
            }
        }
    }

    match tie {
        true => println!("Tie game!"),
        false=>(),
    }
}