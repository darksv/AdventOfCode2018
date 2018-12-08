fn main() {
    let input = std::fs::read_to_string("inputs/input08.txt").unwrap();
    let numbers: Vec<u32> = input.split(' ').filter_map(|n| n.parse().ok()).collect();

    let mut it = numbers.clone().into_iter();
    let root = parse_node(&mut it);

    println!("Sum of metadata numbers = {:?}", root.value1());
    println!("Value of root = {:?}", root.value2());
}

fn parse_node(it: &mut impl Iterator<Item=u32>) -> Node {
    let mut node = Node { children: vec![], metadata: vec![] };

    let children = it.next().unwrap();
    let metadata = it.next().unwrap();

    for _ in 0..children {
        node.children.push(parse_node(it));
    }

    for _ in 0..metadata {
        node.metadata.push(it.next().unwrap());
    }

    node
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn value1(&self) -> u32 {
        let child_sum: u32 = self.children.iter().map(|x| x.value1()).sum();
        let self_sum: u32 = self.metadata.iter().sum();

        child_sum + self_sum
    }

    fn value2(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata
                .iter()
                .sum()
        } else {
            self.metadata
                .iter()
                .filter(|i| **i > 0)
                .filter_map(|i| self.children.get((i - 1) as usize))
                .map(|node| node.value2())
                .sum()
        }
    }
}