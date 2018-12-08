use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("inputs/input07.txt").unwrap();

    let edges = parse_rules(input);
    let edges = sorted_edges(&edges);
    let sorted = find_order(&edges);

    println!("Order = {}", sorted);
    println!("Required time = {}", calculate_time(&edges, 5, 60));
}

#[derive(Copy, Clone)]
struct Edge {
    from: char,
    to: char,
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

fn find_order(edges: &[Edge]) -> String {
    let mut sequence = String::new();
    let mut visited: HashSet<char> = HashSet::new();
    loop {
        if let Some(v) = get_available(edges, &visited).into_iter().min() {
            visited.insert(v);
            sequence.push(v);
        } else {
            break;
        }
    }
    sequence
}

fn sorted_edges(constraints: &[Edge]) -> Vec<Edge> {
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

fn calculate_time(
    edges: &[Edge],
    number_of_workers: u32,
    base_time: u32
) -> u32 {
    let mut workers = vec![Worker::Free; number_of_workers as usize];
    let mut elapsed = 0;

    let mut done = HashSet::new();
    let mut queued = HashSet::new();

    loop {
        for worker in &mut workers {
            *worker = match *worker {
                Worker::Working(task, remaining) if remaining > 1 => Worker::Working(task, remaining - 1),
                Worker::Working(t, _) => {
                    done.insert(t);
                    Worker::Free
                }
                _ => Worker::Free,
            };

            if *worker == Worker::Free {
                let available = get_available(edges, &done)
                    .into_iter()
                    .filter(|v| !queued.contains(v))
                    .min();

                if let Some(v) = available {
                    *worker = Worker::Working(v, base_time + (v as u8 - b'A' + 1) as u32);
                    queued.insert(v);
                }
            }
        }

        if done.len() == find_vertices(edges).len() {
            break;
        }

        elapsed += 1;
    }

    elapsed
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Worker {
    Free,
    Working(char, u32),
}

fn get_available(
    edges: &[Edge],
    visited: &HashSet<char>
) -> Vec<char> {
    let mut pending: HashSet<char> = find_roots(edges)
        .into_iter()
        .filter(|v| !visited.contains(v))
        .collect();

    for Edge { from, to } in edges {
        if visited.contains(&from) && !visited.contains(&to) {
            let ancestors_visited = edges
                .iter()
                .filter(|edge| edge.to == *to)
                .all(|edge| visited.contains(&edge.from));

            if ancestors_visited {
                pending.insert(*to);
            }
        }
    }

    pending.into_iter().collect()
}
