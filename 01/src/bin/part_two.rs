fn main() {
    let mut calories_per_elve = read_calories_per_elve();
    let top3 = calories_per_elve.top3();
    let total_calories: usize = top3.iter().sum();

    println!("{:?}", total_calories);
}

fn read_calories_per_elve() -> Vec<usize> {
    read_lines_from_stdin().into_iter()
        .fold(vec![0], |mut elves, food| {
            match food.is_empty() {
                true => elves.push(0),
                false => {
                    let calories_for_food = food.parse::<usize>().unwrap();
                    let current_elf_calories = elves.last_mut().unwrap();
                    *current_elf_calories += calories_for_food;
                }
            }
            elves
        })
}

fn read_lines_from_stdin() -> Vec<String> {
    std::io::stdin()
        .lines()
        .map(|s| s.unwrap())
        .collect::<Vec<String>>()
}

// Just for fun, use a trait to give Vec<usize> a top3() method.
trait Top3 {
    fn top3(&mut self) -> Vec<usize>;
}

impl Top3 for Vec<usize> {
    fn top3(&mut self) -> Vec<usize> {
        self.sort_by(|a, b| b.cmp(a));
        self.iter().take(3).map(|x| *x).collect::<Vec<usize>>()
    }
}