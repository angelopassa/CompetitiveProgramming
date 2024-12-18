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

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: i32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left child of the  
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
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
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
                        (None, None) => (n, i32::MIN),
                        (Some(v1), None) => (v1 + n, i32::MIN),
                        (None, Some(v2)) => (v2 + n, i32::MIN),
                        (Some(v1), Some(v2)) => (max(v1, v2) + n, n + v1 + v2),
                    }
                };

                let mut to_compute_max = vec![curr_mps_l, curr_mps_r, sum_l_to_r];

                if v == 0 && (self.nodes[v].id_left.is_none() || self.nodes[v].id_right.is_none()) {
                    to_compute_max.push(max_p);
                }

                (Some(max_p), *to_compute_max.iter().max().unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst() {
        let mut tree1 = Tree::with_root(10);
        tree1.add_node(0, 20, false); // parent_id = 1
        tree1.add_node(1, 15, true); // parent_id = 2
        tree1.add_node(1, 32, false); // parent_id = 3
        tree1.add_node(2, 17, false); // parent_id = 4
        tree1.add_node(3, 50, false); // parent_id = 5
        assert!(tree1.is_bst());

        let mut tree2 = Tree::with_root(-40);
        tree2.add_node(0, -50, true); // parent_id = 1
        tree2.add_node(0, 0, false); // parent_id = 2
        tree2.add_node(1, -45, false); // parent_id = 3
        tree2.add_node(2, -10, true); // parent_id = 4
        tree2.add_node(2, 120, false); // parent_id = 5
        assert!(tree2.is_bst());

        let mut tree3 = Tree::with_root(128);
        tree3.add_node(0, 190, false); // parent_id = 1
        tree3.add_node(1, 192, false); // parent_id = 2
        tree3.add_node(2, 202, false); // parent_id = 3
        tree3.add_node(3, 104, true); // parent_id = 4
        tree3.add_node(4, 303, true); // parent_id = 5
        assert!(!tree3.is_bst());

        let mut tree4 = Tree::with_root(-9);
        tree4.add_node(0, 10, false); // parent_id = 1
        tree4.add_node(0, -10, true); // parent_id = 2
        tree4.add_node(1, 45, false); // parent_id = 3
        tree4.add_node(2, -9, false); // parent_id = 4
        tree4.add_node(2, -13, true); // parent_id = 5
        assert!(tree4.is_bst());

        let mut tree5 = Tree::with_root(60);
        tree5.add_node(0, 70, false); // parent_id = 1
        tree5.add_node(0, 10, true); // parent_id = 2
        tree5.add_node(2, 20, false); // parent_id = 3
        tree5.add_node(2, 30, true); // parent_id = 4
        tree5.add_node(3, -13, true); // parent_id = 5
        assert!(!tree5.is_bst());
    }

    #[test]
    fn test_mps() {
        let mut tree1 = Tree::with_root(20);
        tree1.add_node(0, -10, false); // parent_id = 1
        tree1.add_node(0, -20, true); // parent_id = 2
        tree1.add_node(1, 45, false); // parent_id = 3
        tree1.add_node(2, 28, false); // parent_id = 4
        tree1.add_node(2, 7, true); // parent_id = 5
        tree1.add_node(4, -50, false); // parent_id = 6
        tree1.add_node(4, -30, true); // parent_id = 7
        assert_eq!(tree1.maximum_path_sum(), 42);

        let mut tree2 = Tree::with_root(100);
        tree2.add_node(0, -10, false); // parent_id = 1
        tree2.add_node(0, 30, true); // parent_id = 2
        tree2.add_node(2, -17, true); // parent_id = 3
        tree2.add_node(2, -40, false); // parent_id = 4
        tree2.add_node(3, -50, true); // parent_id = 5
        tree2.add_node(4, 8, false); // parent_id = 6
        tree2.add_node(1, 18, false); // parent_id = 7
        tree2.add_node(7, 57, true); // parent_id = 8
        tree2.add_node(7, 0, false); // parent_id = 9
        tree2.add_node(9, 200, true); // parent_id = 10
        assert_eq!(tree2.maximum_path_sum(), 306);

        let mut tree3 = Tree::with_root(20);
        tree3.add_node(0, 40, true); // parent_id = 1
        tree3.add_node(0, -30, false); // parent_id = 2
        tree3.add_node(1, 30, true); // parent_id = 3
        tree3.add_node(1, 0, false); // parent_id = 4
        tree3.add_node(3, -20, true); // parent_id = 5
        tree3.add_node(4, 15, false); // parent_id = 6
        assert_eq!(tree3.maximum_path_sum(), 65);

        let mut tree4 = Tree::with_root(5);
        tree4.add_node(0, 30, true); // parent_id = 1
        tree4.add_node(1, 10, true); // parent_id = 2
        tree4.add_node(1, -20, false); // parent_id = 3
        tree4.add_node(3, 20, false); // parent_id = 4
        assert_eq!(tree4.maximum_path_sum(), 45);

        let mut tree5 = Tree::with_root(-30);
        tree5.add_node(0, 24, true); // parent_id = 1
        tree5.add_node(1, -20, true); // parent_id = 2
        tree5.add_node(2, 10, false); // parent_id = 3
        tree5.add_node(3, 1, false); // parent_id = 4
        assert_eq!(tree5.maximum_path_sum(), -15);

        let tree6 = Tree::with_root(50);
        assert_eq!(tree6.maximum_path_sum(), 50);
    }
}
