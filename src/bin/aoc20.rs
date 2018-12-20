use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/input20.txt").unwrap();

    let mut pos = (0, 0);
    let mut map: HashMap<(i32, i32), Tile> = HashMap::new();

    let mut doors = HashMap::new();
    doors.insert((0, 0), 0);

    let mut anchors = vec![];
    for c in input[1..input.len() - 1].chars() {
        let old = pos;
        match c {
            'N' => {
                pos.1 -= 1;
                map.insert(pos, Tile::Door2);
                pos.1 -= 1;
                map.insert(pos, Tile::Empty);
            }
            'S' => {
                pos.1 += 1;
                map.insert(pos, Tile::Door2);
                pos.1 += 1;
                map.insert(pos, Tile::Empty);
            }
            'W' => {
                pos.0 -= 1;
                map.insert(pos, Tile::Door1);
                pos.0 -= 1;
                map.insert(pos, Tile::Empty);
            }
            'E' => {
                pos.0 += 1;
                map.insert(pos, Tile::Door1);
                pos.0 += 1;
                map.insert(pos, Tile::Empty);
            }
            '(' => {
                anchors.push(pos);
                continue;
            }
            ')' => {
                pos = anchors.pop().unwrap();
                continue;
            }
            '|' => {
                pos = *anchors.last().unwrap();
                continue;
            }
            _ => {
                unimplemented!();
            }
        }

        if doors[&old] + 1 < doors.get(&pos).cloned().unwrap_or(i32::max_value()) {
            doors.insert(pos, doors[&old] + 1);
        }
    }

    println!("Largest number of rooms = {}", doors.values().cloned().max().unwrap());
    println!("Number of rooms requiring passing through at least 1000 doors = {}", doors.values().cloned().filter(|&x| x >= 1000).count());

//    print(&map)
}

fn print(map: &HashMap<(i32, i32), Tile>) {
    let min_x = map.keys().map(|v| v.0).min().unwrap();
    let max_x = map.keys().map(|v| v.0).max().unwrap();
    let min_y = map.keys().map(|v| v.1).min().unwrap();
    let max_y = map.keys().map(|v| v.1).max().unwrap();
    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            let c = if (x, y) == (0, 0) {
                'X'
            } else {
                match map.get(&(x, y)).cloned().unwrap_or(Tile::Wall) {
                    Tile::Door1 => '|',
                    Tile::Door2 => '-',
                    Tile::Wall => '#',
                    Tile::Empty => '.',
                }
            };
            print!("{}", c);
        }
        println!();
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Door1,
    Door2,
    Wall,
    Empty,
}