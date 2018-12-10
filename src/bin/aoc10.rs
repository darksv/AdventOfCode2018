fn main() {
    let input = std::fs::read_to_string("inputs/input10.txt").unwrap();
    let mut points = parse_points(&input);
    let elapsed = simulate(&mut points);
    print_points(&points);
    println!("Elapsed seconds = {}", elapsed);
}

fn simulate(points: &mut [Point]) -> i32 {
    let mut previous_diagonal = u64::max_value();
    let mut elapsed = 0;
    loop {
        let (min_x, max_x, min_y, max_y) = bounds(&points);
        let width = (max_x - min_x) as u64;
        let height = (max_y - min_y) as u64;
        let diagonal = width.pow(2) + height.pow(2);

        if previous_diagonal < diagonal {
            break;
        }

        previous_diagonal = diagonal;

        for point in points.iter_mut() {
            point.position += point.velocity;
        }

        elapsed += 1;
    }

    for point in points.iter_mut() {
        point.position -= point.velocity;
    }

    elapsed -= 1;
    elapsed
}

fn print_points(points: &[Point]) {
    let (min_x, max_x, min_y, max_y) = bounds(points);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.iter().any(|p| p.position.x == x && p.position.y == y) {
                print!("x");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn parse_points(input: &str) -> Vec<Point> {
    let re = regex::Regex::new(r"position=<\s*(\-?\d+),\s*(\-?\d+)> velocity=<\s*(\-?\d+),\s*(\-?\d+)>").unwrap();
    let points: Vec<_> = input.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        Point {
            position: Vector2 {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
            },
            velocity: Vector2 {
                x: caps[3].parse().unwrap(),
                y: caps[4].parse().unwrap(),
            },
        }
    }).collect();
    points
}

fn bounds(points: &[Point]) -> (i32, i32, i32, i32) {
    let mut min_x = i32::max_value();
    let mut max_x = i32::min_value();
    let mut min_y = i32::max_value();
    let mut max_y = i32::min_value();
    for point in points {
        if point.position.x < min_x {
            min_x = point.position.x;
        }

        if point.position.x > max_x {
            max_x = point.position.x;
        }

        if point.position.y < min_y {
            min_y = point.position.y;
        }

        if point.position.y > max_y {
            max_y = point.position.y;
        }
    }
    (min_x, max_x, min_y, max_y)
}

#[derive(Copy, Clone, Debug)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl std::ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        *self = *self - rhs;
    }
}

#[derive(Debug)]
struct Point {
    position: Vector2,
    velocity: Vector2,
}
