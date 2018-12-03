use std::collections::HashMap;
use itertools::iproduct;

fn main() {
    let input = std::fs::read_to_string("inputs/input03.txt").unwrap();
    let claims = parse_claims(input);

    let mut claims_by_coord = HashMap::new();
    for claim in &claims {
        for coord in claim_tiles(claim) {
            claims_by_coord
                .entry(coord)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    let number_of_overlapping = claims_by_coord
        .values()
        .filter(|count| **count >= 2)
        .count();
    println!("Number of overlapping = {}", number_of_overlapping);

    let non_overlapping_claim = claims
        .iter()
        .find(|claim| claim_tiles(claim).all(|coord| claims_by_coord[&coord] == 1));
    println!("Non-overlapping claim = {}", non_overlapping_claim.unwrap().id);
}

fn claim_tiles(claim: &Claim) -> impl Iterator<Item=(u32, u32)> {
    iproduct!(
        claim.x..claim.x + claim.width,
        claim.y..claim.y + claim.height
    )
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