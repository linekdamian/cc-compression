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
    fn generate_tree(&mut self) {
        while !(self.tree.len() == 1) {
            let left_node = self.tree.remove(0);
            let right_node = self.tree.remove(0);
            self.add(HuffNode::new_internal(left_node, right_node));
        }
    }

    pub fn generate_prefix_map(&self) -> Vec<(String, char)> {
        let map: &mut Vec<(String, char)> = &mut Vec::new();
        self.prefix_node_iterator(self.tree[0].left_node.as_ref(), map, "0".to_string());
        self.prefix_node_iterator(self.tree[0].right_node.as_ref(), map, "1".to_string());

        map.to_owned()
    }

    fn prefix_node_iterator(
        &self,
        node: Option<&Box<HuffNode>>,
        map: &mut Vec<(String, char)>,
        prefix: String,
    ) {
        if node.is_none() {
            return;
        }

        let unwraped_node = node.as_ref().unwrap();
        if unwraped_node.char.is_none() {
            self.prefix_node_iterator(unwraped_node.left_node.as_ref(), map, prefix.clone() + "0");
            self.prefix_node_iterator(unwraped_node.right_node.as_ref(), map, prefix.clone() + "1");
            return;
        }

        map.push((prefix, unwraped_node.char.unwrap()));
    }
}

impl From<BTreeMap<char, u32>> for HuffTree {
    fn from(map: BTreeMap<char, u32>) -> Self {
        let mut list = HuffTree { tree: Vec::new() };
        for (c, w) in map {
            list.add(HuffNode::new(c, w));
        }
        list.generate_tree();

        list
    }
}
