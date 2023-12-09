use anyhow::Result;
use derive_more::From;
use parse_display::{FromStr, ParseError};
use std::{collections::HashMap, str};

#[derive(Debug)]
struct Instructions(Vec<Instruction>);

// TODO: this feels like boilerplate some crate should generate
impl str::FromStr for Instructions {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let insts: Vec<Instruction> = s
            .chars()
            .map(|c| c.to_string().as_str().parse())
            .collect::<Result<_, ParseError>>()?;
        Ok(Self(insts))
    }
}

#[derive(Debug, FromStr)]
enum Instruction {
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

/// A tree structure of `Node`s.
#[derive(Debug)]
struct Network(HashMap<Node, (Node, Node)>);

impl FromIterator<NodeConnection> for Network {
    fn from_iter<T: IntoIterator<Item = NodeConnection>>(iter: T) -> Self {
        let map = iter
            .into_iter()
            .map(|nodes| (nodes.from, (nodes.left, nodes.right)))
            .collect();
        Self(map)
    }
}

impl Network {
    fn traverse_left(&self, from: &Node) -> &Node {
        self.0
            .get(from)
            .map(|(left, _right)| left)
            .expect("all instructed traversals should be valid")
    }

    fn traverse_right(&self, from: &Node) -> &Node {
        self.0
            .get(from)
            .map(|(_left, right)| right)
            .expect("all instructed traversals should be valid")
    }
}

#[derive(Debug, FromStr)]
#[display("{from} = ({left}, {right})")]
struct NodeConnection {
    from: Node,
    left: Node,
    right: Node,
}

#[derive(Debug, FromStr, From, Clone, Hash, PartialEq, Eq)]
struct Node(String);

pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let mut lines = puzzle_input.lines();

    // TODO: don't use unwrap() to parse, use nom instead
    let instructions: Instructions = lines.next().unwrap().parse()?;

    // New line between `Instructions` and `NodeConnection`s.
    lines.next().unwrap();

    let node_connections = lines.map(|line| line.parse::<NodeConnection>().unwrap());
    let network: Network = node_connections.collect();

    let start_node = &Node::from("AAA".to_string());
    let end_node = &Node::from("ZZZ".to_string());
    let mut cur_node = start_node;

    let steps_to_reach_end: usize = instructions
        .0
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(i, inst)| {
            if cur_node == end_node {
                Some(i)
            } else {
                let next_node = match inst {
                    Instruction::Left => network.traverse_left(cur_node),
                    Instruction::Right => network.traverse_right(cur_node),
                };
                cur_node = next_node;
                None
            }
        })
        .expect("infinitely cycled iterator cannot terminate without finding something");

    Ok(steps_to_reach_end.to_string())
}
