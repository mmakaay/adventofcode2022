use std::io;

fn main() {
    let rucksacks = get_rucksacks();
    let groups = rucksacks.chunks(3);
    let priority: usize = groups
        .map(get_common_item_in_group)
        .map(get_score_for_item)
        .sum();

    println!("{priority:?}");
}

fn get_rucksacks() -> Vec<String> {
    let input = io::read_to_string(io::stdin()).unwrap();
    input
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn get_common_item_in_group(group: &[String]) -> char {
    group[0]
        .chars()
        .filter(|item| group[1].contains(*item) && group[2].contains(*item))
        .next()
        .unwrap()
}

fn get_score_for_item(item: char) -> usize {
    match item as u8 {
        n if (b'a'..=b'z').contains(&n) => (n - b'a' + 1) as usize,
        n => (n - b'A' + 27) as usize,
    }
}
