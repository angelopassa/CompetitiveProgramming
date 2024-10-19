use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    if nums.len() < 2 {
        return false;
    }

    let mut prefix_sums = HashSet::with_capacity(nums.len());

    let mut sum = nums[0];
    let mut prec_mod = sum % k;

    for &x in nums.iter().skip(1) {
        sum += x;

        if sum % k == 0 || prefix_sums.contains(&(sum % k)) {
            return true;
        }

        prefix_sums.insert(prec_mod);
        prec_mod = sum % k;
    }

    false
}
