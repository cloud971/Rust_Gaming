extern crate games as hangman;

fn main(){
    /* ToDo: add an actually prompt here and some test cases */
    let mut tester = hangman::test_build("Joseph".to_string(), "Sam".to_string(), "hello".to_string());
    tester.make_move('h');
    tester.make_move('l');
    tester.make_move('l');
    tester.make_move('z');
    tester.show_board();
    tester.make_move('x');
    tester.show_board();
}