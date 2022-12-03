use std::io;

// Input legend:
// - A, B, C = rock, paper, scissors
// - X, Y, Z = lose, draw, win
//
// Score per turn:
// - base score based on rock=1, paper=2, scissors=3
// - additional score based on win=6, draw=3 (and lose=0)

fn main() {
    let score: usize = io::read_to_string(io::stdin()).unwrap()
        .split("\n")
        .map(|turn| match turn {
            "A X" => 3 + 0, // lose of rock = scissors
            "A Y" => 1 + 3, // draw with rock
            "A Z" => 2 + 6, // win of rock = paper 
            "B X" => 1 + 0, // lose of paper = rock 
            "B Y" => 2 + 3, // draw with paper = paper
            "B Z" => 3 + 6, // win of paper = scissors
            "C X" => 2 + 0, // lose of scissors = paper
            "C Y" => 3 + 3, // draw with scissors = scissors
            "C Z" => 1 + 6, // win of scissors = rock
            _ => 0, 
        })
        .sum();
    
    println!("{score:?}");
}