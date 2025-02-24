use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct HuffNode {
    char: Option<char>,
    weight: u32,
    left_node: Option<Box<HuffNode>>,
    right_node: Option<Box<HuffNode>>,
}

impl HuffNode {
    pub fn new(char: char, weight: u32) -> HuffNode {
        HuffNode {
            char: Some(char),
            weight,
            left_node: None,
            right_node: None,
        }
    }
    pub fn get_weight(&self) -> u32 {
        self.weight
    }

    pub fn new_internal(left_node: HuffNode, right_node: HuffNode) -> HuffNode {
        HuffNode {
            char: None,
            weight: left_node.get_weight() + right_node.get_weight(),
            left_node: Some(Box::new(left_node)),
            right_node: Some(Box::new(right_node)),
        }
    }
}

#[derive(Debug)]
pub struct HuffTree {
    tree: Vec<HuffNode>,
}

impl HuffTree {
    fn add(&mut self, node: HuffNode) {
        let index = self
            .tree
            .binary_search_by(|huff_node| huff_node.get_weight().cmp(&node.get_weight()));

        match index {
            Ok(index) => self.tree.insert(index + 1, node),
            Err(index) => self.tree.insert(index, node),
        }
    }

    fn format(&mut self) {
        while !(self.tree.len() == 1) {
            let left_node = self.tree.remove(0);
            let right_node = self.tree.remove(0);
            self.add(HuffNode::new_internal(left_node, right_node));
        }
    }
}

impl From<BTreeMap<char, u32>> for HuffTree {
    fn from(map: BTreeMap<char, u32>) -> Self {
        let mut list = HuffTree { tree: Vec::new() };
        for (c, w) in map {
            list.add(HuffNode::new(c, w));
        }
        list.format();

        list
    }
}
