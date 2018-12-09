fn main() {
    let players = slow(452, 70784);
    println!("players = {:#?}", players);

    let players = fast(452, 7078400);
    println!("players = {:#?}", players);
}

fn slow(number_of_players: usize, last_marble: usize) -> usize {
    let mut circle = Vec::with_capacity(last_marble);
    circle.push(0);
    let mut last_insert = 0;
    let mut players = vec![0; number_of_players];
    let mut next_to_insert = 1;
    for i in 0..=last_marble {
        let player = i % number_of_players;

        if next_to_insert % 23 != 0 {
            last_insert = (last_insert + 1) % circle.len() + 1;
            circle.insert(last_insert, next_to_insert);
        } else {
            players[player] += next_to_insert;

            for _ in 0..7 {
                if last_insert == 0 {
                    last_insert = circle.len() - 1;
                } else {
                    last_insert -= 1;
                }
            }

            players[player] += circle.remove(last_insert);
        }

        next_to_insert += 1;
    }

    players.into_iter().max().unwrap()
}

fn fast(number_of_players: usize, last_marble: usize) -> usize {
    slow(number_of_players, last_marble)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slow() {
        assert_eq!(slow(9, 25), 32);
        assert_eq!(slow(10, 1618), 8317);
        assert_eq!(slow(13, 7999), 146373);
        assert_eq!(slow(17, 1104), 2764);
        assert_eq!(slow(21, 6111), 54718);
        assert_eq!(slow(30, 5807), 37305);
    }

    #[test]
    fn test_fast() {
        assert_eq!(fast(9, 25), 32);
        assert_eq!(fast(10, 1618), 8317);
        assert_eq!(fast(13, 7999), 146373);
        assert_eq!(fast(17, 1104), 2764);
        assert_eq!(fast(21, 6111), 54718);
        assert_eq!(fast(30, 5807), 37305);
    }
}