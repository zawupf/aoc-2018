struct Node<'a> {
    children: Vec<Node<'a>>,
    metadata: &'a [u32],
}

impl<'a> Node<'a> {
    fn new(data: &'a [u32]) -> Node<'a> {
        let (node, _) = Node::from(data);
        node
    }

    fn from(data: &'a [u32]) -> (Node<'a>, &'a [u32]) {
        let children_len = data[0] as usize;
        let metadata_len = data[1] as usize;

        let (children, data): (Vec<Node<'a>>, &'a [u32]) =
            (0..children_len).fold((vec![], &data[2..]), |(mut children, data), _| {
                let (node, data) = Node::from(data);
                children.push(node);
                (children, data)
            });
        let metadata = &data[..metadata_len];

        (Node { children, metadata }, &data[metadata_len..])
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata.iter().fold(0, |acc, index| {
                let index = (*index - 1) as usize;
                if let Some(node) = self.children.get(index) {
                    acc + node.value()
                } else {
                    acc
                }
            })
        }
    }
}

fn read_data(data: &str) -> Vec<u32> {
    data.split_whitespace()
        .map(|value| value.parse().unwrap())
        .collect()
}

pub fn metadata_sum(data: &str) -> u32 {
    let data = read_data(data);
    metadata_sum_helper(0, &Node::new(&data))
}

fn metadata_sum_helper(sum: u32, node: &Node) -> u32 {
    node.children
        .iter()
        .fold(sum + node.metadata.iter().sum::<u32>(), |sum, node| {
            metadata_sum_helper(sum, node)
        })
}

pub fn root_value(data: &str) -> u32 {
    Node::new(&read_data(data)).value()
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &'static str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_read_data() {
        assert_eq!(
            vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2],
            read_data(DATA)
        );
    }

    #[test]
    fn test_node_new() {
        let data = read_data(DATA);
        let (_node, data) = Node::from(&data);
        assert_eq!(0, data.len());
    }

    #[test]
    fn test_metadata_sum() {
        assert_eq!(138, metadata_sum(DATA));
    }

    #[test]
    fn test_node_value() {
        assert_eq!(66, root_value(DATA));
    }
}
