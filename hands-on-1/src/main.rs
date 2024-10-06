use std::cmp::{max, min};

fn main() {
    println!("Hello, world!");
}

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

    pub fn is_bst(&self) -> bool {
        self.is_bst_priv(Some(0)).2
    }

    fn is_bst_priv(&self, node_id: Option<usize>) -> (i32, i32, bool) {
        match node_id {
            None => (i32::MAX, i32::MIN, true),
            Some(v) => {
                let (min_l, max_l, is_bst_l) = self.is_bst_priv(self.nodes[v].id_left);
                let (min_r, max_r, is_bst_r) = self.is_bst_priv(self.nodes[v].id_right);
                let n = self.nodes[v].key;
                (
                    min(min_l, min(min_r, n)),
                    max(max_l, max(max_r, n)),
                    is_bst_l && is_bst_r && n >= max_l && n < min_r,
                )
            }
        }
    }

    pub fn maximum_path_sum(&self) -> i32 {
        self.maximum_path_sum_priv(Some(0)).1
    }

    fn maximum_path_sum_priv(&self, u: Option<usize>) -> (Option<i32>, i32) {
        match u {
            None => (None, i32::MIN),
            Some(v) => {
                let n = self.nodes[v].key;
                let (max_p_l, curr_mps_l) = self.maximum_path_sum_priv(self.nodes[v].id_left);
                let (max_p_r, curr_mps_r) = self.maximum_path_sum_priv(self.nodes[v].id_right);

                let (max_p, sum_l_to_r) = {
                    match (max_p_l, max_p_r) {
                        (None, None) => (n, 0),
                        (Some(v1), Some(v2)) => (max(v1, v2) + n, v1 + v2),
                        (Some(v1), None) => (v1 + n, v1),
                        (None, Some(v2)) => (v2 + n, v2),
                    }
                };

                (
                    Some(max_p),
                    max(curr_mps_l, max(curr_mps_r, n + sum_l_to_r)),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.is_bst(), true);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.is_bst(), true);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.is_bst(), true);

        tree.add_node(3, -30, true); // id 5
        tree.add_node(1, 0, true); // id 6
        assert_eq!(tree.maximum_path_sum(), 57);
    }

    #[test]
    fn test_mps() {}
}
