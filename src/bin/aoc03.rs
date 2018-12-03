extern crate regex;

use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/input03.txt").unwrap();
    let claims = parse_claims(input);

    let mut map: HashMap<(_, _), Vec<_>> = HashMap::new();
    for claim in &claims {
        for (x, y) in claim_tiles(claim) {
            map
                .entry((x, y))
                .and_modify(|ids| ids.push(claim.id))
                .or_insert(vec![claim.id]);
        }
    }

    let number_of_overlapping = map
        .values()
        .filter(|ids| ids.len() >= 2)
        .count();
    println!("Number of overlapping = {}", number_of_overlapping);

    // FIXME: rewrite this chain
    let non_overlapping_claim = claims
        .iter()
        .find(|claim|
            claim_tiles(claim)
                .all(|coord|
                    map
                        .get(&coord)
                        .map(|ids| ids.len())
                        .unwrap_or(0) == 1));

    println!("Non-overlapping claim = {}", non_overlapping_claim.unwrap().id);
}

fn claim_tiles(claim: &Claim) -> impl Iterator<Item=(u32, u32)> {
    // FIXME: get rid of the allocation
    let mut tiles = vec![];
    for x in claim.x..claim.x + claim.width {
        for y in claim.y..claim.y + claim.height {
            tiles.push((x, y));
        }
    }

    tiles.into_iter()
}

fn parse_claims(input: String) -> Vec<Claim> {
    let re = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    input
        .lines()
        .filter_map(|line| re.captures(line))
        .map(|c| {
            Claim {
                id: c[1].parse().unwrap(),
                x: c[2].parse().unwrap(),
                y: c[3].parse().unwrap(),
                width: c[4].parse().unwrap(),
                height: c[5].parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}