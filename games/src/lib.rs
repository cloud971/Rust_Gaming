use std::collections::HashSet;

// 2 player hangman
struct Hangman{

    player1:String,
    player2:String,
    the_phrase:String,
    temp:String, // temp string to hold underscores of word, ex: _ _ _
    board:String, // underscore word
    letters:HashSet<char>, // set of letters from the word
    guessed:HashSet<char>, // set of guessed letters
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

        Hangman{

            player1:p1,
            player2:p2,
            the_phrase:guess_word,
            temp:"".to_string(),
            points:0,
            board:"".to_string(),
            letters:HashSet::new(),
            guessed:HashSet::new(),
        }
    }

    // replaces the word with underscore

    pub fn underscore_word(&mut self) -> Result<String,i32>{

        // finds and replace all the letters/numbers with "_" in the phrase that is being guessed
        for c in self.the_phrase.chars() {

            if c.is_alphanumeric() {
                self.temp.push('_');
                self.letters.insert(c);
            }
        }

        match self.temp.len() {

            0 => Err(0),
            _ => Ok("Success".to_string()),
        }
    }
}