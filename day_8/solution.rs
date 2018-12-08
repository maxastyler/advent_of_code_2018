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

    fn add_child(&mut self, child: Node) -> &mut Node {
        self.children.push(child);
        self.children.last_mut().unwrap()
    }

    fn add_metadata(&mut self, m: usize) {
        self.metadata.push(m);
    }

    fn build_node(nums: &mut std::slice::IterMut<'_, usize>) -> Node {
        let num_children = nums.next().unwrap();
        let num_metadata = nums.next().unwrap();
        let mut node = Node::new(*num_children, *num_metadata);
        for _ in 0..*num_children {
            node.add_child(Node::build_node(nums));
        }
        for _ in 0..*num_metadata {
            node.add_metadata(*nums.next().unwrap());
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
}

fn main() {
    let mut input: Vec<usize> = include_str!("./input.txt")
        // let mut input: Vec<usize> = include_str!("./input_test.txt")
        .trim()
        .split(" ")
        .filter(|&x| x != "\n")
        .map(|x| x.parse().unwrap())
        .collect();

    let root = Node::build_node(&mut input.iter_mut());
    println!("{:?}", root.metadata_sum());
}
