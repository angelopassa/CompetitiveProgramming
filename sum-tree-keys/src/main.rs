struct Node {
    key: i32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: i32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: i32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: i32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }
}

fn main() {
    println!("Hello, world!");
}

fn sum_tree_keys(tree: &Tree, u: Option<usize>, sum_to_u: i32) -> (i32, i32) {
    match u {
        None => (0, 0),
        Some(n) => {
            let node = tree.nodes.get(n).unwrap();
            let (n_left, sum_left) = sum_tree_keys(tree, node.id_left, sum_to_u + node.key);
            let (n_right, sum_right) = sum_tree_keys(tree, node.id_right, sum_to_u + node.key);
            let mut n = 0;
            if sum_to_u == sum_left + sum_right {
                n = 1;
            }
            return (n + n_left + n_right, sum_left + sum_right + node.key);
        }
    }
}
