// 2 player hangman
pub struct Hangman {

    player1:String,
    player2:String,
    the_phrase: Vec<char>,
    temp:Vec<char>, // temp string to hold underscores of word, ex: _ _ _
    guessed:Vec<char>, // set of guessed letters
    points:i32, // keeps track of wrong guesses
}


// member functions for hangman struct
impl Hangman {

    /*creates a new hangman obj and initializes
      takes 3 arguments, player names and the string to be guessed
      player 1 is doing the guessing
      points is always 0 because no misses in the beginning
    */

    pub fn new(p1:String,p2:String,guess_word:String)-> Hangman {
        // code to split string into vector
        // initialize temp as equal size vector of underscores
        let phrase_vec: Vec<char> = guess_word.chars().collect();
        let mut underscore_vec: Vec<char> = Vec::new();

        for i in 0..phrase_vec.len() {
            if phrase_vec[i].is_alphanumeric() {
                underscore_vec.push('_');
            } else {
                underscore_vec.push(' ');
            }
        }

        Hangman {
            player1:p1,
            player2:p2,
            the_phrase:phrase_vec, 
            temp:underscore_vec,   
            points:0,
            guessed:Vec::new(),
        }
    }


    pub fn make_move(&mut self,c:char){
        /* 
        Checks first if the guessed letter is in the guessed vector,
        then if not, checks if the guessed letter in the phrase_vector array.
        If so it will replace the underscore in underscore_vec with the letter
        at the appropriate index and add the letter to the guessed vector.
        */
        if self.guessed.contains(&c) {
            println!("You have already guessed that letter!")
        }
        else {
            self.guessed.push(c);
            for i in 0..self.the_phrase.len() {
                if self.the_phrase[i] == c {
                    self.temp[i] = c;
                }
            }
        }
    }


    pub fn show_board(&self){
        /*
        Will calculate the score by checking how many elements are in guessed
        but not in the_phrase. It will be equal to |guessed| - |guessed intersection phrase|
        */
        let mut score = 0i32;
        for i in 0..self.guessed.len() {
            if !self.the_phrase.contains(&self.guessed[i]) {
                score += 1;
            }
        }

        match score {
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
            12 => println!("Game over\n"),
            _ => println!("Try again\n"),
        }
    }
}

// will be called in the test file
pub fn test_build(p1:String, p2:String, guess_word:String) -> Hangman {

    let phrase_vec: Vec<char> = guess_word.chars().collect();
    let mut underscore_vec: Vec<char> = Vec::new();

    for i in 0..phrase_vec.len(){
        if phrase_vec[i].is_alphanumeric() {
            underscore_vec.push('_');
        }
    }
    
    println!("{:?}", phrase_vec);
    println!("{:?}", underscore_vec);

    Hangman {
        player1: p1,
        player2: p2,
        the_phrase: phrase_vec,
        temp: underscore_vec,
        points:0,
        guessed:Vec::new(),
    }
}

pub fn zero_points(){
    println!("Your hangman board: \n");
    println!(" _________ ");
    println!("|         |");
    println!("|          ");
    println!("           ");
}

pub fn one_point(){
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
}

pub fn two_points (){
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("          |   ");
    println!("              ");
}

pub fn three_points(){
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|   ");
}

pub fn four_points(){
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|   ");
    println!("          |   ");
}

pub fn five_points(){
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\  ");
    println!("          |   ");
}

pub fn six_points(){
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("          | * ");
}

pub fn seven_points(){
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("        * | * ");
}

pub fn eight_points(){
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("        * | * ");
    println!("         /    ");
}

fn nine_points(){
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("        * | * ");
    println!("        _/    ");
}

fn ten_points(){
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("        * | * ");
    println!("        _/ \\ ");
}

fn eleven_points(){
    println!("Your hangman board: \n");
    println!(" _________    ");
    println!("|         |   ");
    println!("|         @   ");
    println!("         /|\\ ");
    println!("        * | * ");
    println!("        _/ \\_");
}