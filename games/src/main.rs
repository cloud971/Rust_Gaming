extern crate games as hangman;

fn main(){
    /* ToDo: add an actually prompt here and some test cases */
    let mut tester = hangman::test_build("Joseph".to_string(), "Sam".to_string(), "hello was here!".to_string());
    tester.make_move('h');
    tester.make_move('l');
    tester.make_move('l');
    tester.make_move('e');
    tester.show_board();
    tester.make_move('o');
    tester.show_board();
    tester.make_move('w');
    tester.make_move('a');
    tester.make_move('s');
    tester.show_board();
}