use std::fs;
use std::result;
use std::error::Error;
use std::collections::HashMap;

const FILE_NAME: &str = "./input.real.txt";

type Result<T> = result::Result<T, Box<Error>>;

#[derive(Debug, Clone)]
struct Node {
    id: i32,
    parent: Option<i32>,
    value: i32,
    childs_count: i32,
    childs: Vec<i32>,
    metadatas_count: i32,
    metadatas: Vec<i32>,
}

impl Node {
    fn create (id: i32, childs_count: i32, parent: Option<i32>) -> Node {
        Node {
            id,
            parent,
            value: -1,
            childs_count,
            childs: vec![],
            metadatas_count: -1,
            metadatas: vec![],
        }
    }

    fn is_header_parsed (&self) -> bool {
        self.metadatas_count > -1
        && self.childs_count > -1
    }

    fn has_childs (&self) -> bool {
        self.childs_count == self.childs.len() as i32
    }

    fn has_metadatas (&self) -> bool {
        self.metadatas_count == self.metadatas.len() as i32
    }

    fn add_metadata (&mut self, metadata: i32, nodes: HashMap<i32, Node>) {
        self.metadatas.push(metadata);

        if self.has_metadatas() {
            self.value = self.metadatas.iter().fold(
                0,
                |acc, curr| {
                    if self.childs_count == 0 {
                        return acc + curr;
                    }
                    return acc + match self.childs.get(*curr as usize - 1) {
                        None => 0,
                        Some(child_id) => nodes.get(&child_id).unwrap().clone().value,
                    }
                }
            );
        }
    }
}

fn main() -> Result<()> {
    let content = fs::read_to_string(FILE_NAME)?;
    let content: Vec<_> = content.split_whitespace().collect();

    let mut nodes = HashMap::<i32, Node>::new();
    let mut parent_id = None;
    let mut current_id = -1;

    for (index, c) in content.iter().enumerate() {
        if let Ok(value) = c.parse::<i32>() {
            let id = index as i32;

            if parent_id.is_none() {
                if index > 0 {
                    // back to parent node, we then parse metadatas
                    let ref_nodes = nodes.clone();
                    nodes.entry(current_id)
                        .and_modify(|node| node.add_metadata(value, ref_nodes));

                    continue;
                }

                // parent node (first iteration)
                current_id = id;
                parent_id = Some(id);

                nodes.insert(current_id, Node::create(id, value, None));

                continue;
            }

            let current_node = nodes.get(&current_id).unwrap().clone();
            if !current_node.is_header_parsed() { // setting metadatas_count (header not parsed)
                nodes.entry(current_id)
                    .and_modify(|node| node.metadatas_count = value);
            } else if !current_node.has_childs() { // header parsed, take a new child
                parent_id = Some(current_id);
                current_id = id;

                nodes.insert(current_id, Node::create(id, value, parent_id));

                nodes.entry(parent_id.unwrap())
                    .and_modify(|node| node.childs.push(current_id));
            } else if !current_node.has_metadatas() { // parsing metadatas
                let ref_nodes = nodes.clone();
                nodes.entry(current_id)
                    .and_modify(|node| node.add_metadata(value, ref_nodes));
            } else { // metadatas parsed
                let parent_node = nodes.get(&parent_id.unwrap()).unwrap().clone();

                if parent_node.has_childs() { // parsing metadata from parent
                    current_id = parent_node.id;
                    parent_id = parent_node.parent;

                    let ref_nodes = nodes.clone();
                    nodes.entry(current_id)
                        .and_modify(|node| node.add_metadata(value, ref_nodes));
                } else { // sibling
                    current_id = id;
                    parent_id = current_node.parent;

                    nodes.insert(current_id, Node::create(id, value, parent_id));

                    nodes.entry(parent_id.unwrap())
                        .and_modify(|node| node.childs.push(current_id));
                }
            }
        }
    }

    let result = nodes.values().map(|node| node.metadatas.iter().sum::<i32>()).sum::<i32>();

    println!("part1: {}", result);
    println!("part2: {}", nodes.get(&0).unwrap().value);
    Ok(())
}
