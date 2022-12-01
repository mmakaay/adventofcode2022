fn main() {
    let mut top3 = (0, 0, 0);

    let mut add_to_top3 = |n: usize| {
        top3 = match top3 {
            (a, b, c) if n > a && a <= b && a <= c => (n, b, c),
            (a, b, c) if n > b && b <= a && b <= c => (a, n, c),
            (a, b, c) if n > c && c <= a && c <= b => (a, b, n),
            _ => top3,
        };
    };

    let last_elf_total = std::io::stdin()
        .lines()
        .fold(0, |elf_total, food_calories| {
            match food_calories.unwrap().parse::<usize>() {
                Err(_) => {
                    add_to_top3(elf_total);
                    0
                }
                Ok(n) => elf_total + n,
            }
        });
    add_to_top3(last_elf_total);

    let top3_total = top3.0 + top3.1 + top3.2;
    println!("{top3_total}");
}
