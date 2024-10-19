fn main() {
    println!("Hello, world!");
}

pub struct FenwickTree {
    tree: Vec<i64>,
}

impl FenwickTree {
    pub fn with_len(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() <= 1
    }

    pub fn get_sum(&self, mut idx: usize) -> i64 {
        idx += 1;

        assert!(idx < self.len());

        let mut sum = self.tree[idx];
        while let Some(idx_p) = self.get_parent(idx) {
            sum += self.tree[idx_p];
            idx = idx_p;
        }

        sum
    }

    pub fn update_value(&mut self, mut idx: usize, val: i64) {
        idx += 1;

        assert!(idx < self.len());

        let mut idx_curr = idx;
        while let Some(idx_s) = self.get_next_sibling(idx_curr) {
            self.tree[idx_s] += val;
            idx_curr = idx_s;
        }
    }

    fn get_parent(&self, idx: usize) -> Option<usize> {
        let rm_one = idx & (!idx + 1);
        match rm_one {
            0 => None,
            _ => Some(rm_one),
        }
    }

    fn get_next_sibling(&self, idx: usize) -> Option<usize> {
        let rm_one = idx & (!idx + 1);
        let next_s = idx + rm_one;
        if next_s >= self.len() {
            None
        } else {
            Some(next_s)
        }
    }

    pub fn update_range(&mut self, left: usize, right: usize, val: i64) {
        self.update_value(left, val);
        if right < self.len() - 1 {
            self.update_value(right + 1, -val);
        }
    }

    pub fn get_range(&self, left: usize, right: usize) -> i64 {
        self.get_sum(right) - if left > 0 { self.get_sum(left - 1) } else { 0 }
    }
}
