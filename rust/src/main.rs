use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, Read, Write};
use std::rc::Rc;

#[derive(Eq, PartialEq)]
struct Node {
    ch: Option<char>,
    freq: usize,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq) // Min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_code_table(
    node: &Rc<RefCell<Node>>,
    code: String,
    huffman_code: &mut HashMap<char, String>,
) {
    let node_ref = node.borrow();
    if let Some(ch) = node_ref.ch {
        huffman_code.insert(ch, code);
    } else {
        if let Some(left) = &node_ref.left {
            build_code_table(left, format!("{}0", code), huffman_code);
        }
        if let Some(right) = &node_ref.right {
            build_code_table(right, format!("{}1", code), huffman_code);
        }
    }
}

fn serialize_tree(node: &Rc<RefCell<Node>>, tree_structure: &mut String) {
    let node_ref = node.borrow();
    if let Some(ch) = node_ref.ch {
        tree_structure.push('1');
        tree_structure.push(ch);
    } else {
        tree_structure.push('0');
        if let Some(left) = &node_ref.left {
            serialize_tree(left, tree_structure);
        }
        if let Some(right) = &node_ref.right {
            serialize_tree(right, tree_structure);
        }
    }
}

fn main() -> io::Result<()> {
    let mut filename = String::new();
    println!("Enter the filename: ");
    io::stdin().read_line(&mut filename)?;
    let filename = filename.trim();

    let mut file = File::open(filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut freq = HashMap::new();
    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }

    let mut pq = BinaryHeap::new();
    for (&ch, &freq) in &freq {
        pq.push(Rc::new(RefCell::new(Node {
            ch: Some(ch),
            freq,
            left: None,
            right: None,
        })));
    }

    while pq.len() > 1 {
        let left = pq.pop().unwrap();
        let right = pq.pop().unwrap();
        let new_node = Rc::new(RefCell::new(Node {
            ch: None,
            freq: left.borrow().freq + right.borrow().freq,
            left: Some(left.clone()),
            right: Some(right.clone()),
        }));
        pq.push(new_node);
    }

    let root = pq.pop().unwrap();
    let mut huffman_code = HashMap::new();
    build_code_table(&root, "".to_string(), &mut huffman_code);

    let encoded_text: String = text.chars().map(|ch| huffman_code[&ch].clone()).collect();
    let mut tree_structure = String::new();
    serialize_tree(&root, &mut tree_structure);

    let mut out_file = File::create("encoded.txt")?;
    writeln!(out_file, "{}", tree_structure)?;
    writeln!(out_file, "{}", encoded_text)?;

    println!("Encoded text and Huffman tree written to file.");
    Ok(())
}
