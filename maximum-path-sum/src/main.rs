use std::cmp;

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

fn maximum_path_sum(tree: &Tree, u: Option<usize>) -> (i32, i32) {
    match u {
        None => (0, i32::MIN),
        Some(v) => {
            let node = tree.nodes.get(v).unwrap();
            let (sum_l, curr_max_l) = maximum_path_sum(tree, node.id_left);
            let (sum_r, curr_max_r) = maximum_path_sum(tree, node.id_right);
            let mut curr_sum = node.key + sum_r;

            if node.id_left != None && node.id_right != None {
                if node.key + sum_l > node.key + sum_r {
                    curr_sum = node.key + sum_l;
                } else if node.id_right == None {
                    curr_sum = node.key + sum_l;
                }
            }

            if v == 0 && (node.id_left == None || node.id_right == None)
            {
                (
                    curr_sum,
                    cmp::max(
                        sum_l + sum_r + node.key,
                        cmp::max(curr_max_l, cmp::max(curr_max_r, curr_sum)),
                    ),
                )
            } else if node.id_left == None || node.id_right == None {
                (curr_sum, i32::MIN)
            } else {
                return (
                    curr_sum,
                    cmp::max(sum_l + sum_r + node.key, cmp::max(curr_max_l, curr_max_r)),
                );
            }
        }
    }
}
