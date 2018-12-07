use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("inputs/input07.txt").unwrap();

    let rules = parse_rules(input);
    let rules = sorted_rules(&rules);
    let sorted = find_order(&rules);

    println!("Order = {}", sorted);
    check(&rules, &sorted);
}

fn find_order(edges: &[Edge]) -> String {
    let mut sequence = String::new();

    let mut visited_vertices: HashSet<char> = HashSet::new();

    loop {
        let mut pending: HashSet<char> = find_roots(edges)
            .into_iter()
            .filter(|v| !visited_vertices.contains(v))
            .collect();

        for Edge { from, to } in edges {
            if visited_vertices.contains(&from) && !visited_vertices.contains(&to) {
                let ancestors_visited = edges
                    .iter()
                    .filter(|edge| edge.to == *to)
                    .all(|edge| visited_vertices.contains(&edge.from));

                if ancestors_visited {
                    pending.insert(*to);
                }
            }
        }


        if let Some(v) = pending.into_iter().min() {
            visited_vertices.insert(v);
            sequence.push(v);
        } else {
            break;
        }
    }

    sequence
}

fn sorted_rules(constraints: &[Edge]) -> Vec<Edge> {
    let mut rules = constraints.to_vec();
    rules.sort_by(|lhs, rhs|
        lhs.to.cmp(&rhs.to).then(lhs.from.cmp(&rhs.from)));
    rules
}

fn find_roots(rules: &[Edge]) -> Vec<char> {
    let posterior: HashSet<_> = rules
        .iter()
        .map(|rule| rule.to)
        .collect();

    rules
        .iter()
        .filter(|rule| !posterior.contains(&rule.from))
        .map(|rule| rule.from)
        .unique()
        .sorted()
}

fn find_vertices(rules: &[Edge]) -> HashSet<char> {
    let mut points = HashSet::new();
    for rule in rules {
        points.insert(rule.from);
        points.insert(rule.to);
    }
    points
}

fn check(rules: &[Edge], order: &str) {
    for rule in rules {
        let p1 = order.chars().position(|x| x == rule.from).unwrap();
        let p2 = order.chars().position(|x| x == rule.to).unwrap();

        assert!(p1 < p2);
    }
}

fn parse_rules(input: String) -> Vec<Edge> {
    let re = regex::Regex::new(r"Step (\D+) must be finished before step (\D+) can begin.").unwrap();

    input.lines().filter_map(|line| {
        let captures = re.captures(line)?;
        let from = captures[1].chars().next()?;
        let to = captures[2].chars().next()?;
        Some(Edge { from, to })
    }).collect()
}

#[derive(Copy, Clone)]
struct Edge {
    from: char,
    to: char,
}
