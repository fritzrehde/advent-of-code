use anyhow::Result;
use derive_more::From;
use itertools::Itertools;
use num_integer::Integer;
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
    fn find_starting_nodes(&self) -> impl Iterator<Item = &Node> {
        self.0.keys().filter(|node| node.is_starting_node())
    }

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

#[derive(Debug, From, Clone, Hash, PartialEq, Eq)]
struct Node {
    internal: String,
    node_type: NodeType,
}

impl str::FromStr for Node {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node_type = if s.ends_with('A') {
            NodeType::Start
        } else if s.ends_with('Z') {
            NodeType::End
        } else {
            NodeType::Other
        };
        Ok(Self {
            internal: s.to_string(),
            node_type,
        })
    }
}

impl Node {
    fn is_starting_node(&self) -> bool {
        matches!(self.node_type, NodeType::Start)
    }

    fn is_ending_node(&self) -> bool {
        matches!(self.node_type, NodeType::End)
    }
}

#[derive(Debug, FromStr, From, Clone, Hash, PartialEq, Eq)]
enum NodeType {
    Start,
    End,
    Other,
}

/// This naive solution iterates from the starting nodes until all end nodes are
/// found at the same time. Unfortunately, this solution is incredibly slow.
fn _brute_force_solution(instructions: Instructions, network: Network) -> usize {
    let starting_nodes = network.find_starting_nodes().collect_vec();
    let mut cur_nodes = starting_nodes;

    let steps_to_reach_end: usize = instructions
        .0
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(i, inst)| {
            let mut all_end_nodes = true;
            for cur_node in cur_nodes.iter_mut() {
                if !cur_node.is_ending_node() {
                    all_end_nodes = false;
                }

                // Always iterate to next node, regardless of whether we are an
                // ending node, since our other `cur_nodes` might not all be
                // ending nodes, in which case we must continue searching.
                let next_node = match inst {
                    Instruction::Left => network.traverse_left(cur_node),
                    Instruction::Right => network.traverse_right(cur_node),
                };
                *cur_node = next_node;
            }

            all_end_nodes.then_some(i)
        })
        .expect("infinitely cycled iterator cannot terminate without finding something");
    steps_to_reach_end
}

/// Calculate the `Largest Common Multiplier` (`lcm`) of a list of numbers.
fn lcm(numbers: impl IntoIterator<Item = usize>) -> usize {
    numbers.into_iter().fold(1, |acc, num| acc.lcm(&num))
}

#[test]
fn test_lcm() {
    assert_eq!(6, lcm([2, 3]));
    assert_eq!(24, lcm([4, 6, 8]));
    assert_eq!(180, lcm([20, 30, 45]));
    assert_eq!(5 * 7 * 11, lcm([5, 7, 11]));
}

/// With this solution, we use the fact that the all paths from any starting
/// node to its end node are repeating. Now, we only have to compute the length
/// of the cycle of each starting node to end node path, and check when the
/// total step count of all cycles line up (by calculating the `Largest Common
/// Multiplier`, short `lcm`, of all cycles).
fn lcm_of_cycles_solution(instructions: Instructions, network: Network) -> usize {
    let starting_nodes = network.find_starting_nodes().collect_vec();
    let mut cur_nodes = starting_nodes;

    let cycles_iter = cur_nodes.iter_mut().map(|cur_node| {
        instructions
            .0
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(i, inst)| {
                let is_ending_node = cur_node.is_ending_node();

                // Always iterate to next node, regardless of whether we are an
                // ending node, since our other `cur_nodes` might not all be
                // ending nodes, in which case we must continue searching.
                let next_node = match inst {
                    Instruction::Left => network.traverse_left(cur_node),
                    Instruction::Right => network.traverse_right(cur_node),
                };
                *cur_node = next_node;

                is_ending_node.then_some(i)
            })
            .expect("infinitely cycled iterator cannot terminate without finding something")
    });

    lcm(cycles_iter)
}

pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let mut lines = puzzle_input.lines();

    // TODO: don't use unwrap() to parse, use nom instead
    let instructions: Instructions = lines.next().unwrap().parse()?;

    // New line between `Instructions` and `NodeConnection`s.
    lines.next().unwrap();

    let node_connections = lines.map(|line| line.parse::<NodeConnection>().unwrap());
    let network: Network = node_connections.collect();

    let steps_to_reach_end = lcm_of_cycles_solution(instructions, network);

    Ok(steps_to_reach_end.to_string())
}
