use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("inputs/input05.txt").unwrap();

    let chain = parse_input(&input);

    println!("Final length = {}", full_iterate(&chain).len());
    println!("Shortest polymer = {}", find_shortest(&chain).unwrap());
}

fn find_shortest(chain: &[Unit]) -> Option<usize> {
    find_unique_units(&chain)
        .iter()
        .map(|unit| full_iterate(&with_removed(&chain, *unit)).len())
        .min()
}

fn find_unique_units(chain: &[Unit]) -> Vec<Unit> {
    chain
        .iter()
        .unique()
        .cloned()
        .collect()
}

fn with_removed(chain: &[Unit], unit_to_remove: Unit) -> Vec<Unit> {
    chain
        .iter()
        .filter(|unit| !unit.is_similar_to(&unit_to_remove))
        .cloned()
        .collect()
}

fn iterate(chain: &[Unit], new_chain: &mut Vec<Unit>) {
    let mut i = 0;
    let mut j = 1;

    loop {
        match (chain.get(i), chain.get(j)) {
            (Some(u1), Some(u2)) => {
                if u1.neutralizes(u2) {
                    i = j + 1;
                    j = i + 1;
                } else {
                    new_chain.push(*u1);
                    i += 1;
                    j += 1;
                }
            }
            (Some(u), None) => {
                new_chain.push(*u);
                break;
            }
            (None, None) => break,
            (None, Some(_)) => unreachable!(),
        }
    }
}

fn full_iterate(chain: &[Unit]) -> Vec<Unit> {
    let mut old = chain.to_vec();
    let mut new = vec![];

    loop  {
        iterate(&old, &mut new);
        if new.len() == old.len() {
            return new;
        }

        std::mem::swap(&mut old, &mut new);
        new.clear();
    }
}

fn parse_input(input: &String) -> Vec<Unit> {
    input.chars().filter_map(|c| parse_unit(c).ok()).collect()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Unit(i8);

impl Unit {
    fn neutralizes(&self, other: &Unit) -> bool {
        self.0.saturating_add(other.0) == 0
    }

    fn is_similar_to(&self, other: &Unit) -> bool {
        *self == *other || self.neutralizes(&other)
    }
}

fn parse_unit(value: char) -> Result<Unit, ()> {
    let unit = match value {
        'a'...'z' => -((value as u8 - b'a') as i8) - 1,
        'A'...'Z' => (value as u8 - b'A') as i8 + 1,
        _ => return Err(()),
    };
    Ok(Unit(unit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unit() {
        assert_eq!(parse_unit('a'), Ok(Unit(-1)));
        assert_eq!(parse_unit('z'), Ok(Unit(-26)));
        assert_eq!(parse_unit('A'), Ok(Unit(1)));
        assert_eq!(parse_unit('Z'), Ok(Unit(26)));
        assert_eq!(parse_unit('3'), Err(()));
    }

    fn simple_iterate(chain: &[Unit]) -> Vec<Unit> {
        let mut new = vec![];
        iterate(chain, &mut new);
        new
    }

    #[test]
    fn test_iterate() {
        assert_eq!(simple_iterate(&[Unit(-1), Unit(1), Unit(-1)]), vec![Unit(-1)]);
    }
}