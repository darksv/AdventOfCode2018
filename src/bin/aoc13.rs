const WIDTH: usize = 150;
const HEIGHT: usize = 150;

struct Map {
    tiles: [[Tile; WIDTH]; HEIGHT],
    width: usize,
    height: usize,
}

fn main() {
    let input = std::fs::read_to_string("inputs/input13.txt").unwrap();
    let (map, mut carts) = parse_input(input);

    let mut collision = None;

    while carts.iter().filter(|c| !c.killed).count() > 1 {
        carts.sort_by(|a, b| {
            a.y.cmp(&b.y).then_with(|| a.x.cmp(&b.x))
        });

        for first in 0..carts.len() {
            if carts[first].killed {
                continue;
            }

            tick(&map, &mut carts[first]);

            for second in 0..carts.len() {
                if second == first {
                    continue;
                }

                if carts[second].killed {
                    continue;
                }

                let (x, y) = (carts[first].x, carts[first].y);
                if carts[second].x == x && carts[second].y == y {
                    carts[second].killed = true;
                    carts[first].killed = true;

                    if collision.is_none() {
                        collision = Some((x, y));
                    }
                }
            }
        }

//        print_map(&map, &carts);
    }

    let (x, y) = collision.unwrap();
    println!("First collision = {},{}", x, y);
    let Cart {x, y, .. } = carts.iter().find(|c| !c.killed).unwrap();
    println!("Last cart = {},{}", x, y);
}

fn print_map(map: &Map, carts: &[Cart]) {
    for y in 0..map.height {
        for x in 0..map.width {
            print!("{}", {
                if let Some(cart) = carts.iter().find(|c| c.x == x && c.y == y) {
                    match cart.facing {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    }
                } else {
                    match map.tiles[y][x] {
                        Tile::Empty => ' ',
                        Tile::Road => '#',
                        Tile::Intersection => '+',
                        Tile::DownRight => '/',
                        Tile::UpLeft => '\\',
                    }
                }
            });
        }
        println!();
    }
}

fn tick(map: &Map, cart: &mut Cart) {
    match cart.facing {
        Direction::Up => {
            cart.y -= 1;

            match map.tiles[cart.y][cart.x] {
                Tile::DownRight => cart.facing.turn(Turn::Right),
                Tile::UpLeft => cart.facing.turn(Turn::Left),
                Tile::Intersection => cart.turn(),
                Tile::Road => (),
                tile => panic!("invalid state for {:?} at {:?}", cart, tile),
            }
        }
        Direction::Down => {
            cart.y += 1;

            match map.tiles[cart.y][cart.x] {
                Tile::DownRight => cart.facing.turn(Turn::Right),
                Tile::UpLeft => cart.facing.turn(Turn::Left),
                Tile::Intersection => cart.turn(),
                Tile::Road => (),
                tile => panic!("invalid state for {:?} at {:?}", cart, tile),
            }
        }
        Direction::Left => {
            cart.x -= 1;

            match map.tiles[cart.y][cart.x] {
                Tile::DownRight => cart.facing.turn(Turn::Left),
                Tile::UpLeft => cart.facing.turn(Turn::Right),
                Tile::Intersection => cart.turn(),
                Tile::Road => (),
                tile => panic!("invalid state for {:?} at {:?}", cart, tile),
            }
        }
        Direction::Right => {
            cart.x += 1;

            match map.tiles[cart.y][cart.x] {
                Tile::DownRight => cart.facing.turn(Turn::Left),
                Tile::UpLeft => cart.facing.turn(Turn::Right),
                Tile::Intersection => cart.turn(),
                Tile::Road => (),
                tile => panic!("invalid state for {:?} at {:?}", cart, tile),
            }
        }
    }
}

fn parse_input(input: String) -> (Map, Vec<Cart>) {
    let mut map = Map {
        tiles: [[Tile::Empty; WIDTH]; HEIGHT],
        width: WIDTH,
        height: HEIGHT,
    };
    let mut carts = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '|' | '-' => Tile::Road,
                '+' => Tile::Intersection,
                '/' => Tile::DownRight,
                '\\' => Tile::UpLeft,
                '>' => {
                    carts.push(Cart::new(x, y, Direction::Right));
                    Tile::Road
                }
                '<' => {
                    carts.push(Cart::new(x, y, Direction::Left));
                    Tile::Road
                }
                '^' => {
                    carts.push(Cart::new(x, y, Direction::Up));
                    Tile::Road
                }
                'v' => {
                    carts.push(Cart::new(x, y, Direction::Down));
                    Tile::Road
                }
                ' ' => Tile::Empty,
                _ => unimplemented!()
            };

            map.tiles[y][x] = tile;
        }
    }
    (map, carts)
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,
    Road,
    Intersection,
    DownRight,
    UpLeft,
}


#[derive(Debug)]
struct Cart {
    x: usize,
    y: usize,
    facing: Direction,
    mem: u8,
    killed: bool,
}

impl Cart {
    fn new(x: usize, y: usize, facing: Direction) -> Self {
        Self { x, y, facing, mem: 0, killed: false }
    }

    fn turn(&mut self) {
        match self.mem {
            0 => self.facing.turn(Turn::Left),
            1 => self.facing.turn(Turn::None),
            2 => self.facing.turn(Turn::Right),
            _ => unimplemented!(),
        };

        self.mem = (self.mem + 1) % 3;
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&mut self, turn: Turn) {
        use crate::Direction::*;

        *self = match (*self, turn) {
            (d, Turn::None) => d,
            (Up, Turn::Right) => Right,
            (Up, Turn::Left) => Left,
            (Right, Turn::Right) => Down,
            (Right, Turn::Left) => Up,
            (Down, Turn::Right) => Left,
            (Down, Turn::Left) => Right,
            (Left, Turn::Right) => Up,
            (Left, Turn::Left) => Down,
        }
    }
}

#[derive(Copy, Clone)]
enum Turn {
    None,
    Left,
    Right,
}