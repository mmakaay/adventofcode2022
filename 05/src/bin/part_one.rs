use std::io::{read_to_string, stdin};

type Stack = Vec<char>;
type Stacks = Vec<Stack>;
type ProcedureStep = (usize, usize, usize);
type Procedure = Vec<ProcedureStep>;

fn main() {
    let (mut stacks, procedure) = read_input();
    for step in procedure {
        apply_procecure_step(&mut stacks, step);
    }
    send_top_crates_message(&stacks);
}

fn read_input() -> (Stacks, Procedure) {
    let input = read_to_string(stdin()).unwrap();
    let stacks = parse_stacks_from_input(&input);
    let procedure = parse_procedure_from_input(&input);
    (stacks, procedure)
}

fn parse_stacks_from_input(input: &String) -> Stacks {
    let lines = input.split("\n");
    let stack_lines: Vec<&str> = lines.take_while(|&line| line.contains('[')).collect();
    let number_of_stacks = (stack_lines.iter().map(|s| s.len()).max().unwrap() + 1) / 4;
    let mut stacks = Stacks::with_capacity(number_of_stacks);
    for _ in 0..number_of_stacks {
        stacks.push(Stack::new());
    }
    for &l in stack_lines.iter().rev() {
        for (pos, c) in l.char_indices() {
            if c.is_uppercase() && (pos as isize - 1) % 4 == 0 {
                let stack_nr = (pos - 1) / 4;
                stacks[stack_nr].push(c);
            }
        }
    }
    stacks
}

fn parse_procedure_from_input(input: &String) -> Procedure {
    let num = |s: &str| s.parse::<usize>().unwrap();
    input
        .split("\n")
        .filter(|l| l.starts_with("move"))
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|parts| (num(parts[1]), num(parts[3]) - 1, num(parts[5]) - 1))
        .collect()
}

fn apply_procecure_step(stacks: &mut Stacks, (n, from, to): ProcedureStep) {
    for _ in 0..n {
        if let Some(c) = stacks[from].pop() {
            stacks[to].push(c);
        }
    }
}

fn send_top_crates_message(stacks: &Stacks) {
    stacks.iter().map(|s| s[s.len()-1]).for_each(|c| print!("{c}"));
    println!();
}