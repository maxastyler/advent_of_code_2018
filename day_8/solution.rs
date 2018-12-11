#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn new(children: usize, metadata: usize) -> Node {
        Node {
            children: Vec::with_capacity(children),
            metadata: Vec::with_capacity(metadata),
        }
    }

    fn build_node(nums: &mut std::slice::IterMut<'_, usize>) -> Node {
        let num_children = nums.next().unwrap();
        let num_metadata = nums.next().unwrap();
        let mut node = Node::new(*num_children, *num_metadata);
        for _ in 0..*num_children {
            node.children.push(Node::build_node(nums));
        }
        for _ in 0..*num_metadata {
            node.metadata.push(*nums.next().unwrap());
        }
        node
    }

    fn metadata_sum(&self) -> usize {
        self.children
            .iter()
            .map(|c| c.metadata_sum())
            .sum::<usize>()
            + self.metadata.iter().sum::<usize>()
    }

    fn second_check(&self) -> usize {
        match self.children.len() {
            0 => self.metadata.iter().sum::<usize>(),
            _ => self
                .metadata
                .iter()
                .map(|i| match self.children.get(i - 1) {
                    Some(node) => node.second_check(),
                    None => 0,
                })
                .sum::<usize>(),
        }
    }
}

fn main() {
    let mut input: Vec<usize> = include_str!("./input.txt")
        .trim()
        .split(" ")
        .filter(|&x| x != "\n")
        .map(|x| x.parse().unwrap())
        .collect();

    let root = Node::build_node(&mut input.iter_mut());
    println!("First check: {}", root.metadata_sum());
    println!("Second check: {}", root.second_check());
}
