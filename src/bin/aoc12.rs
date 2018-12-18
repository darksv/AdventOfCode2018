use std::collections::{HashSet, HashMap};

fn main() {
    let input = std::fs::read_to_string("inputs/input12.txt").unwrap();

    let mut initial_state = None;
    let mut patterns = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        match i {
            0 => {
                initial_state = Some(parse(&line[15..]))
            }
            1 => {}
            _ => {
                let mut it = line.splitn(2, " => ");
                let a = parse(it.next().unwrap());
                let b = parse(it.next().unwrap());
                let mut key = [Pot::Empty; 5];
                key.copy_from_slice(&a);
                patterns.insert(key, b[0]);
            }
        }
    }

    let mut state = HashSet::new();
    for (i, &pot) in initial_state.clone().unwrap().iter().enumerate() {
        if pot == Pot::Seed {
            state.insert(i as i32);
        }
    }

    let mut states = HashMap::new();
    states.insert(initial_state.clone().unwrap(), (0, state.iter().sum::<i32>()));

    for i in 1.. {
        let min_index = state.iter().min().cloned().unwrap();
        let max_index = state.iter().max().cloned().unwrap();

        let mut new_state = HashSet::new();

        for i in min_index - 2..=max_index + 2 {
            let mut key = [Pot::Empty; 5];
            for (offset, idx) in (-2..=2).zip(0..) {
                if state.contains(&(i + offset)) {
                    key[idx] = Pot::Seed;
                }
            }

            if let Some(Pot::Seed) = patterns.get(&key) {
                new_state.insert(i);
            }
        }

        state = new_state;

        let min_index = state.iter().min().cloned().unwrap();
        let max_index = state.iter().max().cloned().unwrap();

        let key = (min_index..=max_index)
            .map(|i| if state.contains(&i) { Pot::Seed } else { Pot::Empty })
            .collect::<Vec<_>>()
            .into_boxed_slice();


        let sum = state.iter().sum::<i32>();
        if i == 20 {
            println!("Sum of numbers after 20th generation = {}", sum);
        }

        if let Some(&(prev_index, prev_sum)) = states.get(&key) {
            let shift =  (sum - prev_sum) as i64;
            let sum = (50000000000i64 - prev_index as i64) * shift + prev_sum as i64;
            println!("Sum of numbers after 50000000000th generation = {}", sum);
            break;
        }

        states.insert(key, (i, sum));
    }
}


fn parse(pattern: &str) -> Box<[Pot]> {
    pattern
        .chars()
        .filter_map(|c| match c {
            '#' => Some(Pot::Seed),
            '.' => Some(Pot::Empty),
            _ => None
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Pot {
    Seed,
    Empty,
}