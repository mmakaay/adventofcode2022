use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let rucksacks = get_rucksacks(&input);
    let groups = rucksacks.chunks(3).map(|x| (x[0], x[1], x[2]));
    let priority: usize = groups
        .map(get_common_item_in_group)
        .map(get_score_for_item)
        .sum();

    println!("{priority:?}");
}

fn get_rucksacks(input: &String) -> Vec<&[u8]> {
    input
        .trim()
        .split("\n")
        .map(|sack| sack.as_bytes())
        .collect::<Vec<&[u8]>>()
}

fn get_common_item_in_group((sack_a, sack_b, sack_c): (&[u8], &[u8], &[u8])) -> u8 {
    *(sack_a
        .iter()
        .filter(|&item| sack_b.contains(item) && sack_c.contains(item))
        .next()
        .unwrap())
}

fn get_score_for_item(item: u8) -> usize {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as usize,
        _ => (item - b'A' + 27) as usize,
    }
}
