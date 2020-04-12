#[macro_use]
extern crate lazy_static;

use chess::{Board, Rank, File, Square, ChessMove, Action, MoveGen};
use std::fmt;
use std::io;
use std::collections::hash_map::HashMap;
use rand;
use rand::Rng;

lazy_static! {
    static ref LETTERS_TO_FILES: HashMap<char, File> = [
        ('a', File::A),
        ('b', File::B),
        ('c', File::C),
        ('d', File::D),
        ('e', File::E),
        ('f', File::F),
        ('g', File::G),
        ('h', File::H),
    ].iter().cloned().collect();
}

fn input (message: &'_ impl fmt::Display) -> String
{
    println!("{}", message);
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn square_from_str(s: &str) -> Square{
    let r = s.chars().nth(1).expect("wrong string size");
    let f = s.chars().nth(0).expect("wrong string size");
    let rank = Rank::from_index(r as usize - 1);
    let file = LETTERS_TO_FILES[&f];
    Square::make_square(rank, file)
}


fn main() {

    let mut b = Board::default();

    // let action_std = ChessMove::new(Square::E2, Square::E4, None);
    // println!("action_std {:?}", action_std);

    let mut rng = rand::thread_rng();

    loop {
        println!("{}", b.to_string());
        let user_input = input(&String::from("Enter your turn as SQUARE_FROM SQUARE_TO, e.g. e2 e4"));
        let sq_from = square_from_str(&user_input[0..2]);
        let sq_to = square_from_str(&user_input[3..5]);

        let action = ChessMove::new(sq_from, sq_to, None);
        println!("Trying to make move {:?}", action);
        b = b.make_move_new(action);

        let mut movegen = MoveGen::new_legal(&b);
        let idx: usize = rng.gen();
        let idx = idx % movegen.len();
        let response = movegen.nth(idx).expect("Boom");

        b = b.make_move_new(response);
        println!("Computer makes step: {:?}", response);
    }


    //
    // println!("{}", b.to_string());
    // let b = b.make_move_new(action);
    // println!("{}", b.to_string());


}
