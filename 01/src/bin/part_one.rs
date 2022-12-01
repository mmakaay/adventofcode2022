fn main() {
    let mut total = 0;
    let mut sum = 0;
    for line in read_lines_from_stdin() {
        match line.parse::<usize>() {
            Ok(n) => sum = sum + n,
            Err(_) => {
                total = core::cmp::max(total, sum);
                sum = 0; 
            }
        }
    }
    println!("{total}");
}

fn read_lines_from_stdin() -> Vec<String> {
    std::io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<String>>()
}
