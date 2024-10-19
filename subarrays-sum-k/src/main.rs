use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut prefix_sums = Vec::with_capacity(nums.len());
    let mut prefix_occured = HashMap::with_capacity(nums.len());

    let mut sum = 0;
    for x in nums {
        sum += x;
        prefix_sums.push(sum);
    }

    let mut out = 0;
    for x in prefix_sums {
        if x == k {
            out += 1;
        }

        if let Some(&v) = prefix_occured.get(&(x - k)) {
            out += v;
        }

        prefix_occured.insert(x, *prefix_occured.get(&x).unwrap_or(&0) + 1);
    }

    out
}
