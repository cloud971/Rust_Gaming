
// 2 player hangman
pub struct Hangman{

    player1:String,
    player2:String,
    the_phrase: Vec<char>,
    temp:Vec<char>, // temp string to hold underscores of word, ex: _ _ _
    guessed:Vec<char>, // set of guessed letters
    points:i32, // keeps track of wrong guesses
}


// member functions for hangman struct
impl Hangman{

    /*creates a new hangman obj and initializes
      takes 3 arguments, player names and the string to be guessed
      player 1 is doing the guessing
      points is always 0 because no misses in the beginning
    */
    pub fn new(p1:String,p2:String,guess_word:String)-> Hangman{
        // code to split string into vector
        // initialize temp as equal size vector of underscores
        let phrase_vec: Vec<char> = guess_word.chars().collect();
        let mut underscore_vec: Vec<char> = Vec::new();

        for i in 0..phrase_vec.len(){
            if phrase_vec[i].is_alphanumeric() {
                underscore_vec.push('_');

            }
        }
        
        

        Hangman{

            player1:p1,
            player2:p2,
            the_phrase:phrase_vec, // change to vector
            temp:underscore_vec,   // change to vector
            points:0,
            guessed:Vec::new(),
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

    Hangman{
        player1: p1,
        player2: p2,
        the_phrase: phrase_vec,
        temp: underscore_vec,
        points:0,
        guessed:Vec::new(),
    }
}

pub fn zero_points(){

        println!(" _________ ");
        println!("|         |");
        println!("|          ");
        println!("           ");
}

pub fn one_point(){

        println!(" _________    ");
        println!("|         |   ");
        println!("|         @   ");
}

pub fn two_points (){

        println!(" _________    ");
        println!("|         |   ");
        println!("|         @   ");
        println!("          |   ");
        println!("              ");


}

pub fn show_right_points(){

        println!(" _________    ");
        println!("|         |   ");
        println!("|         @   ");
        println!("         /|   ");
    }

pub fn die(){

        println!(" _________    ");
        println!("|         |   ");
        println!("|         @   ");
        println!("         /|\\ ");
        println!("        * | * ");
        println!("        _/ \\_");
}
