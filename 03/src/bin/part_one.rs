use std::io;

fn main() {
    let answer: usize = get_rucksacks()
        .into_iter()
        .map(get_items_for_left_and_right_rucksack_bags)
        .map(get_common_item_in_left_and_right_bags)
        .map(get_score_for_item)
        .sum();
        
    println!("{answer:?}");
}

fn get_rucksacks() -> Vec<String> {
    let input = io::read_to_string(io::stdin()).unwrap();
    input
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn get_items_for_left_and_right_rucksack_bags(sack: String) -> (String, String) {
    let n = sack.len() / 2;
    (sack[..n].to_string(), sack[n..].to_string())
}

fn get_common_item_in_left_and_right_bags((left, right): (String, String)) -> char {
    left.chars()
        .filter(|item| right.contains(*item))
        .next()
        .unwrap()
}

fn get_score_for_item(item: char) -> usize {
    match item as u8 {
        n if (b'a'..=b'z').contains(&n) => (n - b'a' + 1) as usize,
        n => (n - b'A' + 27) as usize,
    }
}
