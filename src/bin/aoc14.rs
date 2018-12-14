use itertools::Itertools;

fn main() {
    println!("Sequence = {:}", find_sequence(765071).iter().map(|x| x.to_owned()).join(""));
    println!("Sequence position = {:?}", find_sequence_position(&[7, 6, 5, 0, 7, 1]));
}

fn find_sequence(n: usize) -> [u32; 10] {
    let mut recipes: Vec<u32> = vec![3, 7];
    let mut i = 0;
    let mut j = 1;
    while recipes.len() < n + 10 {
        let mut new = recipes[i] + recipes[j];
        let mut c = 0;
        if new == 0 {
            recipes.push(0);
        } else {
            while new > 0 {
                recipes.push(new % 10);
                new /= 10;
                c += 1;
            }
            let range = recipes.len() - c..;
            recipes[range].reverse();
        }

        i = (i + 1 + recipes[i] as usize) % recipes.len();
        j = (j + 1 + recipes[j] as usize) % recipes.len();
    }

    let mut seq = [0u32; 10];
    seq.copy_from_slice(&recipes[n..n + 10]);
    seq
}

fn find_sequence_position(s: &[u32]) -> usize {
    let mut recipes: Vec<u32> = vec![3, 7];
    let mut i = 0;
    let mut j = 1;
    let mut skip = 0;
    loop {
        let mut new = recipes[i] + recipes[j];
        let mut c = 0;
        if new == 0 {
            recipes.push(0);
        } else {
            while new > 0 {
                recipes.push(new % 10);
                new /= 10;
                c += 1;
            }
            let range = recipes.len() - c..;
            recipes[range].reverse();
        }

        i = (i + 1 + recipes[i] as usize) % recipes.len();
        j = (j + 1 + recipes[j] as usize) % recipes.len();

        if let Some((i, _)) = recipes.windows(s.len()).enumerate().skip(skip).find(|x| &x.1 == &s) {
            return i;
        }

        skip = recipes.len().saturating_sub(s.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sequence() {
        assert_eq!(find_sequence(9), [5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);
        assert_eq!(find_sequence(5), [0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
        assert_eq!(find_sequence(18), [9, 2, 5, 1, 0, 7, 1, 0, 8, 5]);
        assert_eq!(find_sequence(2018), [5, 9, 4, 1, 4, 2, 9, 8, 8, 2]);
    }

    #[test]
    fn test_find_sequence_position() {
        assert_eq!(find_sequence_position(&[5, 1, 5, 8, 9]), 9);
        assert_eq!(find_sequence_position(&[0, 1, 2, 4, 5]), 5);
        assert_eq!(find_sequence_position(&[9, 2, 5, 1, 0]), 18);
        assert_eq!(find_sequence_position(&[5, 9, 4, 1, 4]), 2018);
    }
}