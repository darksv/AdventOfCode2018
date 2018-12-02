use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/input02.txt").unwrap();
    let ids: Vec<_> = input.lines().collect();

    eprintln!("Checksum = {}", checksum(&ids));
    eprintln!("Common letters = {}", common_letters(&ids));
}

fn checksum(ids: &[&str]) -> i32 {
    let mut count2 = 0;
    let mut count3 = 0;
    for id in ids {
        let mut counts = HashMap::new();
        for c in id.chars() {
            counts
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut has2 = false;
        let mut has3 = false;

        for (_, count) in counts {
            match count {
                2 => has2 = true,
                3 => has3 = true,
                _ => ()
            }
        }

        if has2 {
            count2 += 1;
        }
        if has3 {
            count3 += 1;
        }
    }
    count3 * count2
}

fn common_letters(ids: &[&str]) -> String {
    let mut smallest_diff = usize::max_value();
    let mut pair = None;

    for (index, &first) in ids.iter().enumerate() {
        for &second in &ids[index + 1..] {
            let diff = difference(first, second);
            if diff < smallest_diff {
                smallest_diff = diff;
                pair = Some((first, second));
            }
        }
    }

    if let Some((first, second)) = pair {
        first
            .chars()
            .zip(second.chars())
            .filter_map(|(a, b)|
                if a == b {
                    Some(a)
                } else {
                    None
                })
            .collect()
    } else {
        String::new()
    }
}

fn difference(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .filter(|&(a, b)| a != b)
        .count()
}

