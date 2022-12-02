use std::io;

// Input legend:
// - A, B, C = rock, paper, scissors
// - X, Y, Z = rock, paper, scissors
//
// Score per turn:
// - base score based on rock=1, paper=2, scissors=3
// - additional score based on win=6, draw=3 (and lose=0)

fn main() {
    let score: usize = io::read_to_string(io::stdin()).unwrap()
        .split("\n")
        .map(|turn| match turn {
            "A X" => 1 + 3, // rock     = rock      draw
            "A Y" => 2 + 6, // paper    > rock      win
            "A Z" => 3 + 0, // scissors < rock      lose
            "B X" => 1 + 0, // rock     < paper     lose
            "B Y" => 2 + 3, // paper    = paper     draw
            "B Z" => 3 + 6, // scissors > paper     win
            "C X" => 1 + 6, // rock     > scissors  win
            "C Y" => 2 + 0, // paper    < scissors  lose
            "C Z" => 3 + 3, // scissors = scissors  draw
            _ => 0, 
        })
        .sum();
    
    println!("{score:?}");
}