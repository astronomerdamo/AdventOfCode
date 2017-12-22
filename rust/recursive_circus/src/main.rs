use std::fs;
use std::env;
use std::io::Read;
use std::collections::HashMap;

#[derive(Clone)]
struct Node {
    children: Vec<String>,
    weight: u32,
    branch_weight: u32,
}

impl Node {
    fn find_tree_root(tree: &HashMap<String, Node>) -> String {
        let root = match tree.keys().cloned()
            .filter(|k| !tree.values().any(|n| n.children.contains(k)))
            .next() {
                Some(r) => r,
                None => panic!("FAILURE : NO TREE ROOT"),
            };
        println!("Tree Root: {:?}", &root);
        root
    }

    fn find_tree_key_by_value(tree: &HashMap<String, Node>, value: &u32) -> String {
        match tree.keys().cloned()
            .filter(|k| {
                tree.get(k).unwrap().branch_weight == *value
            })
            .next() {
                Some(r) => r,
                None => panic!("FAILURE : NO TREE KEY WITH VALUE"),
            }
    }

    fn get_node_level_map(tree: &HashMap<String, Node>, name: &String) -> HashMap<String, u32> {
        Node::get_node_children(&tree, &name)
            .iter()
            .map(|c| {
                let fn_child = c.clone();
                let fn_branch_weight = tree.get(&fn_child).unwrap().branch_weight;
                (fn_child, fn_branch_weight)
            })
            .collect()
    }

    fn find_node_parent_from_child(tree: &HashMap<String, Node>, name: &String) -> String {
        match tree.keys().cloned()
            .filter(|k| {
                tree.get(k).unwrap().children.contains(name)
            })
            .next() {
                Some(r) => r,
                None => panic!("FAILURE : NO TREE KEY WITH CHILD"),
            }
    }

    fn find_out_of_balance_node(tree: &HashMap<String, Node>, name: &String) {

        let mut bad_node = name.clone();
        bad_node = loop {
            match Node::get_node_level_map(&tree, &bad_node)
                .iter()
                .fold(HashMap::new(), |mut acc, t| {
                    let fn_t = t.clone();
                    *acc.entry(*fn_t.1).or_insert(0) += 1;
                    acc
                })
                .iter()
                .map(|h| {
                    let fn_h = h.clone();
                    let fn_branch_weight: u32 = *(fn_h.0);
                    let fn_occurances: u32 = *(fn_h.1);
                    (fn_branch_weight, fn_occurances)
                })
                .filter(|t| {
                    let fn_t = t.clone();
                    fn_t.1 == 1
                })
                .next() {
                    Some(t) => {
                        let fn_t = t.clone();
                        bad_node = Node::find_tree_key_by_value(&tree, &fn_t.0)
                    },
                    None => break bad_node,
                }
        };

        let bad_node_level_map = Node::get_node_level_map(
            &tree,
            &Node::find_node_parent_from_child(&tree, &bad_node.clone())
        );

        let ok_node = match bad_node_level_map.keys().cloned()
            .filter(|k| {*k != bad_node})
            .next() {
                Some(k) => k,
                None => panic!("FUUUUUCK"),
            };

        let bad_node_branch_weight = tree.get(&bad_node).unwrap().branch_weight;
        let ok_node_branch_weight = tree.get(&ok_node).unwrap().branch_weight;
        let weight_corrector = bad_node_branch_weight - ok_node_branch_weight;
        let bad_node_weight = tree.get(&bad_node).unwrap().weight;

        println!("Bad Node: {:?},\nBad Node Weight: {:?},\nBad Node Corrected Weight: {:?}",
            &bad_node,
            &bad_node_weight,
            &(bad_node_weight - weight_corrector)
        );

    }

    fn get_node_weight(tree: &HashMap<String, Node>, name: &String) -> u32 {
        match tree.get(name) {
            Some(n) => n.weight,
            None => panic!("FAILURE : NO NODE PRESENT"),
        }
    }

    fn get_node_children(tree: &HashMap<String, Node>, name: &String) -> Vec<String> {
        match tree.get(name) {
            Some(n) => n.children.clone(),
            None => panic!("FAILURE : NO NODE PRESENT"),
        }
    }

    fn get_branch_weight(tree: &HashMap<String, Node>, name: &String) -> u32 {
        let mut branch_weight: u32 = Node::get_node_weight(&tree, &name);
        let mut node_branch_children: Vec<String> = Node::get_node_children(&tree, &name);

        loop {
            let children_weight: u32 = node_branch_children.iter().map(|c| Node::get_node_weight(&tree, &c)).sum();
            node_branch_children = node_branch_children.iter().cloned().flat_map(|x| Node::get_node_children(&tree, &x)).collect();
            branch_weight += children_weight;

            if node_branch_children.is_empty() {
                break branch_weight
            }
        }
    }

    fn enrich_node_tree_with_branch_weight(tree: &HashMap<String, Node>) -> HashMap<String, Node> {
        tree.iter()
            .map(|(name, node)| {
                let fn_name = name.clone();
                let fn_node = node.clone();
                (fn_name, Node{
                    children: fn_node.children,
                    weight: fn_node.weight,
                    branch_weight: Node::get_branch_weight(&tree, &name),
                })
            })
            .collect()
    }

    fn initialize_node_tree() -> HashMap<String, Node> {
        open_file_read_contents()
            .lines().map(|line| {(
                Node::parse_node_name(line),
                Node {
                    children: Node::parse_node_children(line),
                    weight: Node::parse_node_weight(line),
                    branch_weight: 0u32,
                })
            }).collect()
    }

    fn parse_node_name(line: &str) -> String {
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

    let mut tree: HashMap<String, Node> = Node::initialize_node_tree();
    tree = Node::enrich_node_tree_with_branch_weight(&tree);

    // Part A: Find Tree Root
    let tree_root: String = Node::find_tree_root(&tree);
    // Part B: Find and correct out of balance node
    Node::find_out_of_balance_node(&tree, &tree_root);
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
