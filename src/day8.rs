
#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>
}

impl Node {
    fn checksum(&self) -> i32 {
        self.metadata.iter().sum::<i32>() + self.children.iter().map(|child| child.checksum()).sum::<i32>()
    }

    fn value(&self) -> i32 {
        match self.children.len() {
            0 => self.metadata.iter().sum(),
            _ => {
                self.metadata.iter()
                    .filter_map(|&i| self.child_by_metadata(i as usize))
                    .map(|node| node.value())
                    .sum()
            }
        }
    }

    fn child_by_metadata(&self, i: usize) -> Option<&Node> {
        match i {
            0 => None,
            i if i - 1 >= self.children.len() => None,
            i => {
                Some(&self.children[i - 1])
            }

        }
    }
}

fn parse_nodes(input: &[i32]) -> (Node, usize) {
    let num_children = input[0];
    let num_metadata = input[1];
    let mut i = 2usize;
    let mut children = vec![];

    for _ in 0..num_children {
        let (child, size) = parse_nodes(&input[i..]);
        children.push(child);
        i += size;
    }

    let end = i + num_metadata as usize;
    let metadata = Vec::from(&input[i..end]);

    let node = Node {
        children,
        metadata
    };

    (node, end)
}

fn parse_input(input: &str) -> Vec<i32> {
    input.split_whitespace().map(|s| { s.parse::<i32>().unwrap() }).collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
    let v = parse_input(input);
    let (node, _) = parse_nodes(&v);
    
    // println!("{:?}", node);
    node.checksum()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
    let v = parse_input(input);
    let (node, _) = parse_nodes(&v);

    node.value()
}