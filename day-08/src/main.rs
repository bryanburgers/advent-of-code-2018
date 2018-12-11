use std::io::{Read, self};

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let input : Vec<usize> = buffer.split(" ")
        .map(|i| i.trim())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let node = Node::parse(&input[..]);
    println!("{:?}", node.metadata_sum());
    println!("{:?}", node.value());
}

#[derive(Debug, Clone)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn parse(input: &[usize]) -> Node {
        let (node, _) = Self::parse_helper(input);
        node
    }

    fn parse_helper(input: &[usize]) -> (Node, usize) {
        assert!(input.len() >= 2);
        let child_entries = input[0];
        let metadata_entries = input[1];

        if child_entries == 0 {
            assert!(input.len() >= metadata_entries + 2);
            let node = Node {
                children: vec![],
                metadata: input[2..2+metadata_entries].to_owned(),
            };
            (node, 2 + metadata_entries)
        }
        else {
            let mut children = Vec::new();
            let mut offset = 2;
            for _ in 0..child_entries {
                let (node, read) = Self::parse_helper(&input[offset..]);
                children.push(node);
                offset += read;
            }
            let node = Node {
                children: children,
                metadata: input[offset..offset+metadata_entries].to_owned(),
            };
            (node, offset+metadata_entries)
        }
    }

    fn metadata_sum(&self) -> usize {
        let metadata_sum = self.metadata.iter().fold(0, |acc, i| acc + i);
        let children_sum = self.children.iter().fold(metadata_sum, |acc, child| acc + child.metadata_sum());
        children_sum
    }

    fn value(&self) -> usize {
        if self.children.len() == 0 {
            self.metadata.iter().fold(0, |acc, i| acc + i)
        }
        else {
            let mut sum = 0;
            for i in &self.metadata {
                if *i == 0 {
                    continue
                }
                let index = *i - 1;
                if index >= self.children.len() {
                    continue
                }
                let child = &self.children[index];
                sum += child.value();
            }
            sum
        }
    }
}
