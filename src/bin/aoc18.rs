use std::collections::HashMap;

const WIDTH: usize = 50;
const HEIGHT: usize = 50;

fn main() {
    let input = std::fs::read_to_string("inputs/input18.txt").unwrap();
    let mut map = parse_input(&input);

    let mut history = HashMap::new();
    history.insert(compress(&map), 0);

    for i in 1.. {
        let mut new_map = map.clone();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                new_map[y][x] = {
                    let mut trees = 0;
                    let mut lamberyards = 0;

                    for (x, y) in adjacent(x, y, WIDTH, HEIGHT) {
                        match map[y][x] {
                            Tile::Lumberyard => lamberyards += 1,
                            Tile::Tree => trees += 1,
                            _ => {}
                        }
                    }

                    match map[y][x] {
                        Tile::OpenGround if trees >= 3 => Tile::Tree,
                        Tile::Tree if lamberyards >= 3 => Tile::Lumberyard,
                        Tile::Lumberyard => {
                            if lamberyards >= 1 && trees >= 1 {
                                Tile::Lumberyard
                            } else {
                                Tile::OpenGround
                            }
                        }
                        old => old
                    }
                };
            }
        }

        map = new_map;

        if i == 10 {
            let mut l = 0;
            let mut t = 0;
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    match map[y][x] {
                        Tile::OpenGround => {}
                        Tile::Lumberyard => { l += 1; }
                        Tile::Tree => { t += 1; }
                    }
                }
            }

            println!("Total resource value after 10 minutes = {}", l * t);
        }

        let key = compress(&map);
        if let Some(&old_index) = history.get(&key) {
            let offset = old_index;
            let cycle = i - old_index;
            let x = (1_000_000_000 - offset) % cycle + offset;
            let cached: &Box<[Tile]> = history.iter().find(|(_key, index)| **index == x).unwrap().0;


            let l = cached
                .iter()
                .filter(|&x| *x == Tile::Lumberyard)
                .count();

            let t = cached
                .iter()
                .filter(|&x| *x == Tile::Tree)
                .count();


            println!("Total resource value after 1000000000 minutes = {}", l * t);
            break;
        }
        history.insert(key, i);
    }
}

fn compress(map: &Map) -> Box<[Tile]> {
    map.iter().flat_map(|x| x.iter().cloned()).collect::<Vec<Tile>>().into_boxed_slice()
}

type Map = [[Tile; WIDTH]; HEIGHT];

fn parse_input(input: &str) -> Map {
    let mut map = [[Tile::OpenGround; WIDTH]; HEIGHT];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[y][x] = match c {
                '#' => Tile::Lumberyard,
                '.' => Tile::OpenGround,
                '|' => Tile::Tree,
                _ => unimplemented!()
            }
        }
    }

    map
}

fn adjacent(x: usize, y: usize, width: usize, height: usize) -> impl Iterator<Item=(usize, usize)> {
    let xp = x.checked_sub(1).unwrap_or(width);
    let yp = y.checked_sub(1).unwrap_or(height);
    let xn = x.checked_add(1).unwrap_or(width);
    let yn = y.checked_add(1).unwrap_or(height);

    let is_ok = move |x: usize, y: usize| x < width && y < height;

    [(xp, yp), (x, yp), (xn, yp), (xp, y), (xn, y), (xp, yn), (x, yn), (xn, yn)]
        .iter()
        .cloned()
        .filter(move |(x, y)| is_ok(*x, *y))
        .collect::<Vec<_>>()
        .into_iter()
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Tile {
    OpenGround,
    Lumberyard,
    Tree,
}