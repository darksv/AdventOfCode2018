use std::collections::{HashSet, HashMap};

fn main() {
    let input = std::fs::read_to_string("inputs/input06.txt").unwrap();

    let points = parse_points(input);

    println!("Size of largest inferior area = {}", part_1(&points).unwrap());
    println!("Size of safe area = {}", part_2(&points));
}

fn part_1(points: &Vec<Point>) -> Option<usize> {
    let (min_x, max_x, min_y, max_y) = bounds(&points);
    let mut area_size = HashMap::with_capacity(points.len());
    let mut boundary_areas = HashSet::new();
    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            let point = Point::new(x, y);

            let area = match index_of_single_nearest(&points, &point) {
                Some(area) => area,
                None => continue
            };

            area_size
                .entry(area)
                .and_modify(|size| *size += 1)
                .or_insert(1);

            if x == min_x - 1 || x == max_x + 1 || y == min_y - 1 || y == max_y + 1 {
                boundary_areas.insert(area);
            }
        }
    }
    let size_of_largest_inferior_area = area_size
        .iter()
        .filter(|x| !boundary_areas.contains(x.0))
        .max_by_key(|x| x.1);

    size_of_largest_inferior_area.map(|x| *x.1)
}

fn part_2(points: &[Point]) -> i32 {
    let mut area_size = 0;
    let (min_x, max_x, min_y, max_y) = bounds(&points);
    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            let point = Point::new(x, y);

            let total_distance: i32 = points
                .iter()
                .map(|p| manhattan_distance(p, &point))
                .sum();

            if total_distance < 10000 {
                area_size += 1;
            }
        }
    }
    area_size
}


fn parse_points(input: String) -> Vec<Point> {
    input.lines().filter_map(|line| {
        let mut values = line.split(", ").filter_map(|x| x.parse().ok());
        let x = values.next()?;
        let y = values.next()?;
        Some(Point::new(x, y))
    }).collect()
}

fn index_of_single_nearest(points: &[Point], point: &Point) -> Option<usize> {
    let (index, nearest_point) = points
        .iter()
        .enumerate()
        .min_by_key(|(_, p)| manhattan_distance(p, point))?;

    let distance = manhattan_distance(point, nearest_point);
    let number_of_equally_distant = points
        .iter()
        .filter(|p| manhattan_distance(p, point) == distance)
        .count();

    if number_of_equally_distant == 1 {
        Some(index)
    } else {
        None
    }
}

fn bounds(points: &[Point]) -> (i32, i32, i32, i32) {
    let mut min_x = i32::max_value();
    let mut max_x = i32::min_value();
    let mut min_y = i32::max_value();
    let mut max_y = i32::min_value();
    for p in points {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }

    (min_x, max_x, min_y, max_y)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
