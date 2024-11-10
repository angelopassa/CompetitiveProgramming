use std::{
    cmp::{max, min},
    fs, io,
};

const N_TEST: usize = 11;

fn main() -> io::Result<()> {
    for test_num in 0..N_TEST {
        println!("Executing Test #{}", test_num);
        let content = fs::read_to_string(format!("Testset_handson2_p1/input{}.txt", test_num))?;

        let nums: Vec<u64> = content
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let n = nums[0] as usize;
        let m = nums[1] as usize;

        let mut i = 2;

        let mut s = MinMax::with_len(n);
        for num in 0..n {
            s.put_item(num + 1, nums[num + i]);
        }

        i += n;

        let mut results = String::new();

        let mut query_type;
        for x in 0..m {
            query_type = nums[x + i];
            if query_type == 0 {
                s.update(
                    nums[x + i + 1] as usize,
                    nums[x + i + 2] as usize,
                    nums[x + i + 3],
                );
                i += 3;
            } else {
                let m = s.max(nums[x + i + 1] as usize, nums[x + i + 2] as usize);
                results.push_str(&m.to_string());
                results.push('\n');
                i += 2;
            }
        }

        let output = fs::read_to_string(format!("Testset_handson2_p1/output{}.txt", test_num))?;

        assert_eq!(output, results);
    }

    Ok(())
}

pub struct MinMax {
    segment_tree: Vec<u64>,
    lazy_tree: Vec<u64>,
    size: usize,
}

impl MinMax {
    pub fn with_len(n: usize) -> Self {
        assert!(n > 0);

        Self {
            segment_tree: vec![u64::MIN; 4 * n],
            lazy_tree: vec![u64::MAX; 4 * n],
            size: n,
        }
    }

    fn update_node_from_lazy(&mut self, pos: usize, my_i: usize, my_j: usize) {
        if self.lazy_tree[pos] != u64::MAX {
            if self.segment_tree[pos] > self.lazy_tree[pos] || self.segment_tree[pos] == 0 {
                self.segment_tree[pos] = self.lazy_tree[pos];

                if my_i != my_j {
                    let mut children = 2 * pos + 1;
                    self.lazy_tree[children] = min(self.lazy_tree[children], self.lazy_tree[pos]);

                    children += 1;
                    self.lazy_tree[children] = min(self.lazy_tree[children], self.lazy_tree[pos]);
                }
            }

            self.lazy_tree[pos] = u64::MAX;
        }
    }

    pub fn max(&mut self, i: usize, j: usize) -> u64 {
        assert!(i > 0);
        assert!(j > 0);
        assert!(j - 1 < self.size);

        self.max_inner(i - 1, j - 1, 0, self.size - 1, 0)
    }

    fn max_inner(&mut self, i: usize, j: usize, my_i: usize, my_j: usize, my_pos: usize) -> u64 {
        self.update_node_from_lazy(my_pos, my_i, my_j);

        if i > my_j || j < my_i {
            return u64::MIN;
        }

        if i <= my_i && j >= my_j {
            return self.segment_tree[my_pos];
        }

        let mid = (my_i + my_j) / 2;

        max(
            self.max_inner(i, j, my_i, mid, 2 * my_pos + 1),
            self.max_inner(i, j, mid + 1, my_j, 2 * my_pos + 2),
        )
    }

    pub fn update(&mut self, i: usize, j: usize, t: u64) {
        assert!(i > 0);
        assert!(j > 0);
        assert!(j - 1 < self.size);

        self.update_inner(i - 1, j - 1, 0, self.size - 1, 0, t);
    }

    fn update_inner(
        &mut self,
        i: usize,
        j: usize,
        my_i: usize,
        my_j: usize,
        my_pos: usize,
        t: u64,
    ) -> u64 {
        self.update_node_from_lazy(my_pos, my_i, my_j);

        if i > my_j || j < my_i {
            return self.segment_tree[my_pos];
        }

        if i <= my_i && j >= my_j {
            if t < self.segment_tree[my_pos] || self.segment_tree[my_pos] == 0 {
                self.lazy_tree[my_pos] = t;
                return self.lazy_tree[my_pos];
            }

            return self.segment_tree[my_pos];
        }

        let mid = (my_i + my_j) / 2;

        self.segment_tree[my_pos] = max(
            self.update_inner(i, j, my_i, mid, 2 * my_pos + 1, t),
            self.update_inner(i, j, mid + 1, my_j, 2 * my_pos + 2, t),
        );

        self.segment_tree[my_pos]
    }

    pub fn put_item(&mut self, pos: usize, val: u64) {
        self.update(pos, pos, val);
    }
}
