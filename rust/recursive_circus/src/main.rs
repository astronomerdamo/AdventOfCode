use std::fs;
use std::env;
use std::io::Read;
// use std::collections::HashSet;
// use std::iter::FromIterator;

#[derive(Clone)]
struct Node {
    parent: String,
    weight: u32,
    children: Vec<String>,
}

impl Node {
    fn find_tree_root(tree_nodes: &Vec<Node>) -> String {
        let mut start: String = match tree_nodes.iter().cloned().nth(0) {
            Some(n) => n.parent,
            None => panic!("FAILURE : EMPTY TREE NODE VECTOR"),
        };

        loop {
            match tree_nodes.iter().cloned()
                .filter(|n| n.children.contains(&start))
                .map(|n| n.parent).next() {
                    Some(p) => start = p,
                    None => break start,
            }
        }
    }

    fn parse_node_parent(line: &str) -> String {
        match line.split_whitespace().nth(0) {
            Some(p) => String::from(p),
            None => panic!("FAILURE : PARSE NODE > PARENT"),
        }
    }

    fn parse_node_weight(line: &str) -> u32 {
        match line.split_whitespace().nth(1) {
            Some(p) => p.trim_matches(|c| c == '(' || c == ')').parse().unwrap(),
            None => panic!("FAILURE : PARSE NODE > WEIGHT"),
        }
    }

    fn parse_node_children(line: &str) -> Vec<String> {
        match line.split("->").nth(1) {
            Some(p) => p.split(", ").map(|x| String::from(x.trim())).collect(),
            None => Vec::new(),
        }
    }
}

fn main() {

    let contents: String = open_file_read_contents();

    let tree_nodes: Vec<Node> = contents.lines().map(|line| {
        Node {
            parent: Node::parse_node_parent(line),
            weight: Node::parse_node_weight(line),
            children: Node::parse_node_children(line),
        }
    }).collect();

    let tree_root: String = Node::find_tree_root(&tree_nodes);
    println!("TREE ROOT: {:?}", &tree_root);

}

fn open_file_read_contents() -> String {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => panic!("FAILURE : FILE PATH"),
    };

    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => panic!("FAILURE : OPEN FILE"),
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(s) => s,
        Err(_) => panic!("FAILURE : READ FILE"),
    };
    buffer
}
