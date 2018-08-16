# Rust_Gaming

This is a very simple library of games in rust.

1. Hangman
2. Tic-tac-toe

Main.rs contains an example file of how to interact with the library. In the main.rs the user will be able
to play hangman and tic-tac-toe.

# Test
The test directory contains test files for the functionality of the game.

# How to play

1  2  3
[X] [] []
[] [X] []
[] [] [x]
 7  8  9
 
To play tic-tac-toe there are 2 players required. In order to make a move the users type in number
corresponding to the area of the bpard they want to mark. For example to mark the top left square
on the board, player 1 would enter a 1. There are 9 squares and counting from top left to right is
how the board is numbered for a particular move. The game ends when a user gets 3 in a row.

To play hangman the there are 2 players required. Player 1 does the guessing of letters and player 2
enters the word that is to be guessed. The game ends when player 1 misses on 11 letters or player one
is able to guess all of the letters in the phrase.


     _________
    |         |
              @ 
             /|\
            * | * 
            _/ \_
            
 H _ W  _ r _  y _ _?
