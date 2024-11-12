use std::{collections::HashSet, fs, io};

const N_TEST: usize = 8;

fn main() -> io::Result<()> {
    for test_num in 0..N_TEST {
        println!("Executing Test #{}", test_num);
        let content = fs::read_to_string(format!("Testset_handson2_p2/input{}.txt", test_num))?;

        let nums: Vec<usize> = content
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let n = nums[0];
        let m = nums[1];

        let mut i = 2;

        let mut s = SegmentTree::with_len(n);
        let mut sgmts = Vec::with_capacity(n);

        for num in (0..n).map(|x| x * 2) {
            sgmts.push((nums[num + i], nums[num + i + 1]));
        }

        i += 2 * n;

        let mut results = String::new();
        s.load_segments(sgmts);

        for x in 0..m {
            let v = s.is_there(nums[x + i], nums[x + i + 1], nums[x + i + 2] as u64);
            results.push_str(if v { "1" } else { "0" });
            results.push('\n');
            i += 2;
        }

        let output = fs::read_to_string(format!("Testset_handson2_p2/output{}.txt", test_num))?;

        assert_eq!(output, results);
    }

    Ok(())
}

pub struct SegmentTree {
    tree: Vec<HashSet<u64>>,
    size: usize,
}

impl SegmentTree {
    pub fn with_len(n: usize) -> Self {
        Self {
            tree: vec![HashSet::new(); 2 * n + 1],
            size: n + 1,
        }
    }

    pub fn load_segments(&mut self, sgmts: Vec<(usize, usize)>) {
        let mut sweep_line = vec![0; sgmts.len() + 1];

        for (start, end) in sgmts {
            sweep_line[start] += 1;
            sweep_line[end + 1] -= 1;
        }

        let mut count = 0;
        let mut val;
        for (i, &point) in sweep_line.iter().enumerate() {
            val = point + count;
            count += point;

            self.add_point(i, val as u64);
        }
    }

    fn add_point(&mut self, point: usize, point_sgmts: u64) {
        let mut left = 0;
        let mut right = self.size - 1;
        let mut current_node = 0;
        let mut mid;

        while left <= right {
            self.tree[current_node].insert(point_sgmts);

            mid = (left + right) / 2;

            if left == right {
                break;
            }

            if point <= mid {
                current_node = current_node + 1;
                right = mid;
            } else {
                current_node = current_node + 2 * (mid - left + 1);
                left = mid + 1;
            }
        }
    }

    pub fn is_there(&self, i: usize, j: usize, k: u64) -> bool {
        assert!(j < self.size - 1);

        self.is_there_inner(0, self.size - 1, 0, i, j, k)
    }

    fn is_there_inner(
        &self,
        my_i: usize,
        my_j: usize,
        my_pos: usize,
        i: usize,
        j: usize,
        k: u64,
    ) -> bool {
        if i > my_j || j < my_i {
            return false;
        }

        if i <= my_i && j >= my_j {
            return self.tree[my_pos].contains(&k);
        }

        let mid = (my_i + my_j) / 2;

        self.is_there_inner(my_i, mid, my_pos + 1, i, j, k)
            || self.is_there_inner(mid + 1, my_j, my_pos + 2 * (mid - my_i + 1), i, j, k)
    }
}
