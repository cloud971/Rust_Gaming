/* Test file - Joseph Sands & Sam Cha - Rust Summer 2018 */

extern crate games;
use games::*;

/* -------------- Hangman Tests -------------- */

#[test]
fn test_spaces() {
    let mut object = games::test_hangman("Joseph".to_string(), "Sam".to_string(), "test this".to_string());
    let test_vec = vec!['t', 'e', 's', 't', ' ', 't', 'h', 'i', 's'];
    assert_eq!(test_vec, object.the_phrase);
}



#[test]
fn test_hangman_loss() {
    // tests that a player loses after the set amount of incorrect guesses
    let mut object = games::test_hangman("Joseph".to_string(), "Sam".to_string(), "bcd".to_string());
    let mut check = object.make_move('e');
    check = object.make_move('f');
    check = object.make_move('g');
    check = object.make_move('h');
    check = object.make_move('i');
    check = object.make_move('j');
    check = object.make_move('k');
    check = object.make_move('l');
    check = object.make_move('m');
    check = object.make_move('n');
    check = object.make_move('o');
    check = object.show_board();
    
    assert!(!(check));
}


#[test]
fn test_guessed_letter() {
    // tests that a guessing a duplicate letter will return false
    let mut object = games::test_hangman("Joseph".to_string(), "Sam".to_string(), "bcd".to_string());
    let mut check = object.make_move('e');
    check = object.make_move('e');

    assert!(!(check));
}


#[test]
fn test_guessed_array() {
    // tests that the guessed array is being initialized and updated correctly
    let mut object = games::test_hangman("Joseph".to_string(), "Sam".to_string(), "bcd".to_string());
    let mut test_vec = vec!['e', 'f', 'g','h'];
    let mut check = object.make_move('e');
    check = object.make_move('f');
    check = object.make_move('g');
    check = object.make_move('h');

    assert_eq!(&test_vec, &object.guessed);
}


/* -------------- Hangman Tests -------------- */

#[test]
fn test_init() {
    // tests that the board is being initialized correctly
    let mut object = games::test_tic_tac_toe();
    let mut test_vec = vec![vec!["[ ]".to_string(),"[ ]".to_string(),"[ ]".to_string()],
                      vec!["[ ]".to_string(),"[ ]".to_string(),"[ ]".to_string()],
                      vec!["[ ]".to_string(),"[ ]".to_string(),"[ ]".to_string()]];

    assert_eq!(test_vec, object.board);
}


#[test]
fn test_duplicate() {
    // tests that a duplicate play is handled correctly
    let mut tic_test = games::test_tic_tac_toe();
    let check = tic_test.mark_player(1,1);
    let check = tic_test.mark_player(1,1);

    assert_eq!(check, false);
}


#[test]
fn test_row_win() {
    // tests that three is a row is recognized as a win
    let mut object = games::test_tic_tac_toe();
    object.mark_player(1,1);
    object.mark_player(2,1);
    object.mark_player(3,1);

    assert_eq!("[O]".to_string(), object.check_win());
}


#[test]
fn test_diag_win() {
    // tests that three diagonal is recognized as a win
    let mut object = games::test_tic_tac_toe();
    object.mark_player(1,1);
    object.mark_player(5,1);
    object.mark_player(9,1);

    assert_eq!("[O]".to_string(), object.check_win());
}


#[test]
fn test_col_win() {
    // tests that three in a column is recognized as a win
    let mut object = games::test_tic_tac_toe();
    object.mark_player(1,1);
    object.mark_player(4,1);
    object.mark_player(7,1);

    assert_eq!("[O]".to_string(), object.check_win());
}


#[test]
fn test_no_win() {
    // tests that a win is not calculated without the above conditions
    let mut object = games::test_tic_tac_toe();
    object.mark_player(1,1);
    object.mark_player(2,1);
    object.mark_player(7,1);

    assert_eq!("z".to_string(), object.check_win());
}


#[test]
fn test_tic_duplicate() {
    // tests that duplicate guesses are handles correctly
    let mut object = games::test_tic_tac_toe();
    let mut check = object.mark_player(1,1);
    check = object.mark_player(1,1);
    
    assert!(!(check));
}













