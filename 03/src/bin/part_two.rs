use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let rucksacks: Vec<&str> = input.lines().collect();
    let groups: Vec<&[&str]> = rucksacks.chunks(3).collect();
    let common_item_per_group: Vec<char> = groups
        .iter()
        .map(|g| {
            g[0].chars()
                .filter(|&c| g[1].contains(c) && g[2].contains(c))
                .next()
                .unwrap()
        })
        .collect();
    let priority_per_group: Vec<usize> = common_item_per_group
        .iter()
        .map(|&item| match item {
            'a'..='z' => ((item as u8) - b'a' + 1) as usize,
            'A'..='Z' => ((item as u8) - b'A' + 27) as usize,
            _ => panic!("Invalid item encountered"),
        })
        .collect();

    let answer: usize = priority_per_group.iter().sum();
    println!("{answer:?}");
}
