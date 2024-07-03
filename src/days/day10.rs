use crate::util;
use anyhow::{Ok, Result};
use std::collections::HashMap;

pub fn run() -> Result<()> {
    let raw_input = util::read_input("inputs/day10.txt")?;
    let world = World::from(raw_input.as_str());
    let boundary = world.explore();
    println!("Part 1: {}", boundary.len() / 2);
    println!("Part 2: {}", interior_area(&boundary));

    Ok(())
}

#[derive(Debug)]
struct World {
    data: HashMap<(usize, usize), Node>,
    start: (usize, usize),
}

impl World {
    fn explore(&self) -> Vec<(usize, usize)> {
        let mut visited = Vec::new();
        let mut curr_node = self.data.get(&self.start).unwrap();
        while !visited.contains(&curr_node.coords) {
            let temp = curr_node;
            if curr_node.has_upper_connection()
                && !visited.contains(&(curr_node.coords.0 - 1, curr_node.coords.1))
            {
                visited.push(curr_node.coords);
                curr_node = self
                    .data
                    .get(&(curr_node.coords.0 - 1, curr_node.coords.1))
                    .unwrap();
            }

            if curr_node.has_left_connection()
                && !visited.contains(&(curr_node.coords.0, curr_node.coords.1 - 1))
            {
                visited.push(curr_node.coords);
                curr_node = self
                    .data
                    .get(&(curr_node.coords.0, curr_node.coords.1 - 1))
                    .unwrap();
            }

            if curr_node.has_lower_connection()
                && !visited.contains(&(curr_node.coords.0 + 1, curr_node.coords.1))
            {
                visited.push(curr_node.coords);
                curr_node = self
                    .data
                    .get(&(curr_node.coords.0 + 1, curr_node.coords.1))
                    .unwrap();
            }

            if curr_node.has_right_connection()
                && !visited.contains(&(curr_node.coords.0, curr_node.coords.1 + 1))
            {
                visited.push(curr_node.coords);
                curr_node = self
                    .data
                    .get(&(curr_node.coords.0, curr_node.coords.1 + 1))
                    .unwrap();
            }

            if curr_node.coords == temp.coords {
                visited.push(curr_node.coords);
                break;
            }
        }

        visited
    }
}

impl From<&str> for World {
    fn from(value: &str) -> Self {
        let data = HashMap::new();
        let mut world = World {
            data,
            start: (0, 0),
        };
        value.lines().enumerate().for_each(|(row, line)| {
            line.char_indices()
                .filter(|(_, p)| *p != '.')
                .for_each(|(col, pipe)| {
                    let coords = (row, col);
                    if pipe == 'S' {
                        world.start = coords
                    }
                    world.data.insert(coords, Node::new(coords, pipe.into()));
                })
        });

        world
    }
}

#[derive(Debug)]
struct Node {
    coords: (usize, usize),
    connections: u8,
}

impl Node {
    fn new(coords: (usize, usize), pipe_type: PipeType) -> Self {
        Node {
            coords,
            connections: Node::set_connections(pipe_type),
        }
    }

    fn has_upper_connection(&self) -> bool {
        self.connections & 1 == 1
    }

    fn has_left_connection(&self) -> bool {
        self.connections >> 1 & 1 == 1
    }

    fn has_lower_connection(&self) -> bool {
        self.connections >> 2 & 1 == 1
    }

    fn has_right_connection(&self) -> bool {
        self.connections >> 3 & 1 == 1
    }

    fn set_connections(pipe_type: PipeType) -> u8 {
        let mut connections = u8::default();
        match pipe_type {
            PipeType::Horizontal | PipeType::Start => {
                connections |= 1 << 1;
                connections |= 1 << 3;
            }
            PipeType::Vertical => {
                connections |= 1;
                connections |= 1 << 2;
            }
            PipeType::NtoE => {
                connections |= 1;
                connections |= 1 << 3;
            }
            PipeType::NtoW => {
                connections |= 1;
                connections |= 1 << 1;
            }
            PipeType::StoE => {
                connections |= 1 << 2;
                connections |= 1 << 3;
            }
            PipeType::StoW => {
                connections |= 1 << 2;
                connections |= 1 << 1;
            }
            _ => (),
        }
        connections
    }
}

enum PipeType {
    Vertical,
    Horizontal,
    NtoE,
    NtoW,
    StoE,
    StoW,
    Start,
    None,
}

impl From<char> for PipeType {
    fn from(value: char) -> Self {
        match value {
            '|' => PipeType::Vertical,
            '-' => PipeType::Horizontal,
            'L' => PipeType::NtoE,
            'J' => PipeType::NtoW,
            '7' => PipeType::StoW,
            'F' => PipeType::StoE,
            'S' => PipeType::Start,
            '.' => PipeType::None,
            _ => unreachable!(),
        }
    }
}

fn shoelace_area(vertices: &[(usize, usize)]) -> usize {
    let left_lace: usize = (0..vertices.len() - 1)
        .map(|n| vertices[n].0 * vertices[n + 1].1)
        .sum::<usize>()
        + vertices[vertices.len() - 1].0 * vertices[0].1;

    let right_lace: usize = (0..vertices.len() - 1)
        .map(|n| vertices[n + 1].0 * vertices[n].1)
        .sum::<usize>()
        + vertices[0].0 * vertices[vertices.len() - 1].1;
    let discrim = left_lace as isize - right_lace as isize;
    discrim.unsigned_abs() / 2
}

fn interior_area(vertices: &[(usize, usize)]) -> usize {
    let shoelace_area = shoelace_area(vertices);
    shoelace_area + 1 - (vertices.len() / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_connections() {
        let node = Node::new((1, 1), PipeType::Vertical);

        assert!(node.has_upper_connection());
        assert!(!node.has_left_connection());
        assert!(node.has_lower_connection());
        assert!(!node.has_right_connection());

        let node_b = Node::new((2, 2), PipeType::Horizontal);

        assert!(!node_b.has_upper_connection());
        assert!(node_b.has_left_connection());
        assert!(!node_b.has_lower_connection());
        assert!(node_b.has_right_connection());

        let node_c = Node::new((3, 3), PipeType::NtoE);
        assert!(node_c.has_upper_connection());
        assert!(!node_c.has_left_connection());
        assert!(!node_c.has_lower_connection());
        assert!(node_c.has_right_connection());

        let node_d = Node::new((1, 1), PipeType::StoW);
        assert!(!node_d.has_upper_connection());
        assert!(node_d.has_left_connection());
        assert!(node_d.has_lower_connection());
        assert!(!node_d.has_right_connection());
    }
}
