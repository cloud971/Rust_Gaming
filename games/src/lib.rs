
// 2 player hangman
pub struct Hangman {
    player1: String,
    player2: String,
    pub the_phrase: Vec<char>,
    temp: Vec<char>,    // temp string to hold underscores of word, ex: _ _ _
    pub guessed: Vec<char>, // set of guessed letters
    points: i32,        // keeps track of wrong guesses
}

// member functions for hangman struct
impl Hangman {
    /*creates a new hangman obj and initializes
      takes 3 arguments, player names and the string to be guessed
      player 1 is doing the guessing
      points is always 0 because no misses in the beginning
    */

    pub fn new(p1: String, p2: String, mut guess_word: String) -> Hangman {
        // code to split string into vector
        // initialize temp as equal size vector of underscores

        let remove_end = guess_word.trim_right().len();
        guess_word.truncate(remove_end);
        let phrase_vec: Vec<char> = guess_word.chars().collect();
        let mut underscore_vec: Vec<char> = Vec::new();

        for i in 0..phrase_vec.len() {
            if phrase_vec[i].is_alphanumeric() {
                underscore_vec.push('_');
            } else if phrase_vec[i].is_whitespace() {
                underscore_vec.push(' ');
            } else {
                underscore_vec.push(phrase_vec[i]);
            }
        }

        Hangman {
            player1: p1,
            player2: p2,
            the_phrase: phrase_vec,
            temp: underscore_vec,
            points: 0,
            guessed: Vec::new(),
        }
    }

    pub fn make_move(&mut self, c: char) -> bool {
        /* 
        Checks first if the guessed letter is in the guessed vector,
        then if not, checks if the guessed letter in the phrase_vector array.
        If so it will replace the underscore in underscore_vec with the letter
        at the appropriate index and add the letter to the guessed vector.
        */
        if self.guessed.contains(&c) {
            println!("You have already guessed that letter!");
            return false;
        } else {
            self.guessed.push(c);
            for i in 0..self.the_phrase.len() {
                if self.the_phrase[i] == c {
                    self.temp[i] = c;
                }
            }
        }
        return true;
    }

    pub fn show_board(&mut self) -> bool{
        /*
        Will calculate the score by checking how many elements are in guessed
        but not in the_phrase. It will be equal to |guessed| - |guessed intersection phrase|
        */
        for i in 0..self.guessed.len() {
            if !self.the_phrase.contains(&self.guessed[i]) {
                self.points += 1;
            }
        }

        match self.points {
            0 => zero_points(),
            1 => one_point(),
            2 => two_points(),
            3 => three_points(),
            4 => four_points(),
            5 => five_points(),
            6 => six_points(),
            7 => seven_points(),
            8 => eight_points(),
            9 => nine_points(),
            10 => ten_points(),
            11 => eleven_points(),
            _ => println!("Try again\n"),
        }

        for i in 0..self.temp.len() {
            print!("{} ", self.temp[i]);
        }

        if !self.temp.contains(&'_') {
            println!("\n{} wins\n", self.player1);
            self.points = 12;
        }
        else if self.points != 11{ self.points =0; }
        else if self.points == 11{ println!("\nthe game is over {} wins\n",self.player2); return false;}
        return true;
    }


    pub fn get_score(&self) ->i32{

        match self.points {
            12 => 1, // you win
            11 => 2, // you lose
            _ => 3, // keep playing

        }
    }
}

// for tic tac toe
pub struct tictac{
    pub board:Vec<Vec<String>>,
    pub moves:Vec<u32>,
}

impl tictac{

    // constructor
    pub fn new()->tictac{

        /*
        each square is a number the vector 'moves' holds
        9 numbers for each square
        */
        let mut tic_tac = vec![vec!["[ ]".to_string(),"[ ]".to_string(),"[ ]".to_string()],
                               vec!["[ ]".to_string(),"[ ]".to_string(),"[ ]".to_string()],
                               vec!["[ ]".to_string(),"[ ]".to_string(),"[ ]".to_string()]];

        // keeps track of move
        let mut the_moves = vec![1,2,3,4,5,6,7,8,9];

        tictac{
            board:tic_tac,
            moves:the_moves,
        }
    }

    // marking the board
    pub fn mark_player(&mut self,position:u32,player:u32) ->bool{

        let mut xo = String::new();

        if player == 1 {
            xo = "[O]".to_string();
        }else {
            xo = "[X]".to_string();
        }

        if self.moves.contains(&position){

            match position{

                1=> self.board[0][0] = xo.to_string(),
                2=> self.board[0][1] = xo.to_string(),
                3=> self.board[0][2] = xo.to_string(),
                4=> self.board[1][0] = xo.to_string(),
                5=> self.board[1][1] = xo.to_string(),
                6=> self.board[1][2] = xo.to_string(),
                7=> self.board[2][0] = xo.to_string(),
                8=> self.board[2][1] = xo.to_string(),
                9=> self.board[2][2] = xo.to_string(),
                _=> (),
            }

            //removes the number from the 'moves_left' vector so it can't be marked again
            self.moves.retain(|&x| x !=  position);
            true
        }else{ // the square was already marked
            println!("Try again");
            false
        }
    }
/*
    pub fn mark_player2(&mut self,position:u32) ->bool{

        let xo = "[X]".to_string();
        if self.moves.contains(&position){

            match position{

                1=> self.board[0][0] = xo.to_string(),
                2=> self.board[0][1] = xo.to_string(),
                3=> self.board[0][2] = xo.to_string(),
                4=> self.board[1][0] = xo.to_string(),
                5=> self.board[1][1] = xo.to_string(),
                6=> self.board[1][2] = xo.to_string(),
                7=> self.board[2][0] = xo.to_string(),
                8=> self.board[2][1] = xo.to_string(),
                9=> self.board[2][2] = xo.to_string(),
                _=> (),
            }

            //removes the number from the 'moves_left' vector so it can't be marked again
            self.moves.retain(|&x| x !=  position);
            true
        }else{ // the square was already marked
            println!("Try again");
            false
        }
    }
*/
    pub fn check_win(&self)->String{

        /*
        first three cases check for 3 in a row
        second three cases check for 3 columns
        next two cases check for diagonals
        last case says no one has won
        */
        if self.board[0][0]!= "[ ]".to_string() && self.board[0][0] == self.board[0][1] && self.board[0][2] == self.board[0][0]{
            self.board[0][0].clone()
        }else if self.board[1][0]!= "[ ]".to_string()&&self.board[1][0] == self.board[1][1] && self.board[1][2] == self.board[1][0]{
            self.board[1][0].clone()
        }else if self.board[2][0]!= "[ ]".to_string()&&self.board[2][0] == self.board[2][1] && self.board[2][2] == self.board[2][0]{
            self.board[2][0].clone()
        }else if self.board[0][0]!= "[ ]".to_string()&&self.board[1][0] == self.board[0][0] && self.board[2][0] == self.board[0][0]{
            self.board[0][0].clone()
        }else if self.board[0][1]!= "[ ]".to_string()&&self.board[0][1] == self.board[2][1] && self.board[0][1] == self.board[2][1]{
            self.board[0][1].clone()
        }else if self.board[0][2]!= "[ ]".to_string()&&self.board[0][2] == self.board[2][2] && self.board[0][2] == self.board[2][2]{
            self.board[0][2].clone()
        }else if self.board[0][0]!= "[ ]".to_string()&&self.board[0][0] == self.board[1][1] && self.board[0][0] == self.board[2][2]{
            self.board[0][0].clone()
        }else if self.board[2][0]!= "[ ]".to_string()&&self.board[2][0] == self.board[1][1] && self.board[2][0] == self.board[0][2]{
            self.board[2][0].clone()
        }else{
            "z".to_string()
        }
    }

    pub fn moves_left(&self) -> bool{

        if self.moves.is_empty(){
            false
        }else { true }
    }


    pub fn tic_board(&self){

        for x in self.board.iter(){
            for things in x{

                print!("{}",things);
            }
            println!("\n");
        }
    }

}


// will be called in the test file
pub fn test_hangman(p1: String, p2: String, guess_word: String) -> Hangman {
    let phrase_vec: Vec<char> = guess_word.chars().collect();
    let mut underscore_vec: Vec<char> = Vec::new();

    for i in 0..phrase_vec.len() {
        if phrase_vec[i].is_alphanumeric() {
            underscore_vec.push('_');
        } else if phrase_vec[i].is_whitespace() {
            underscore_vec.push(' ');
        } else {
            underscore_vec.push(phrase_vec[i]);
        }
    }

    Hangman {
        player1: p1,
        player2: p2,
        the_phrase: phrase_vec,
        temp: underscore_vec,
        points: 0,
        guessed: Vec::new(),
    }
}


pub fn test_tic_tac_toe() -> tictac {

    let mut tic_tac = vec![vec!["[ ]".to_string(),"[ ]".to_string(),"[ ]".to_string()],
                      vec!["[ ]".to_string(),"[ ]".to_string(),"[ ]".to_string()],
                      vec!["[ ]".to_string(),"[ ]".to_string(),"[ ]".to_string()]];

    // keeps track of move
    let mut the_moves = vec![1,2,3,4,5,6,7,8,9];

    tictac{
        board:tic_tac,
        moves:the_moves,
    }
}

pub fn zero_points() {
    println!("Your hangman board: \n");
    println!(" _________ ");
    println!("|         |");
    println!("|          ");
    println!("           ");
}

pub fn one_point() {
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
}

pub fn two_points() {
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("          |   ");
    println!("              ");
}

pub fn three_points() {
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|   ");
}

pub fn four_points() {
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|   ");
    println!("          |   ");
}

pub fn five_points() {
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\  ");
    println!("          |   ");
}

pub fn six_points() {
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("          | * ");
}

pub fn seven_points() {
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("        * | * ");
}

pub fn eight_points() {
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("        * | * ");
    println!("         /    ");
}

fn nine_points() {
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("        * | * ");
    println!("        _/    ");
}

fn ten_points() {
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("        * | * ");
    println!("        _/ \\ ");
}

fn eleven_points() {
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("        * | * ");
    println!("        _/ \\_");
}
