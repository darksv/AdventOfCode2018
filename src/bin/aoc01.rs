use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("inputs/input01.txt").unwrap();
    let changes = parse_input(&input);

    println!("Resulting frequency = {}", find_effective(&changes));
    println!("First frequency occurring twice = {}", find_first_occurring_twice(&changes));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|line| -> Option<i32> {
            let sign = match &line[0..1] {
                "+" => 1,
                "-" => -1,
                _ => return None,
            };
            let value: i32 = line[1..].parse().ok()?;
            Some(value * sign)
        })
        .collect()
}

fn find_effective(changes: &[i32]) -> i32 {
    changes.iter().sum::<i32>()
}

fn find_first_occurring_twice(changes: &[i32]) -> i32 {
    let mut current = 0;
    let mut seen = HashSet::new();
    seen.insert(current);
    for &change in changes.iter().cycle() {
        current += change;
        if seen.contains(&current) {
            return current;
        }
        seen.insert(current);
    }
    unreachable!();
}