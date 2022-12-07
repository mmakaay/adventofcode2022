use std::{
    io::{read_to_string, stdin},
    time::Instant,
};

use aoc2022_07::CumulativeDirSizer;

fn main() {
    let now = Instant::now();

    let input = read_to_string(stdin()).unwrap();
    let mut sizer = CumulativeDirSizer::new();
    input
        .trim()
        .split("\n")
        .for_each(|line| sizer.process(line));
    let sizes = sizer.get_sizes();

    let disk_size = 70000000usize;
    let required_space = 30000000usize;
    let used_space = sizes.iter().max().unwrap();
    let remaining_space = disk_size - used_space;
    let size_of_dir_to_remove = sizes
        .iter()
        .filter(|&size| remaining_space + size >= required_space)
        .min()
        .unwrap();

    println!("time: {:?}", now.elapsed());
    println!("anser: {}", size_of_dir_to_remove);
}
