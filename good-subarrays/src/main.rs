use std::collections::HashMap;

fn main() {
    println!("{}", good_subarrays(Vec::from([1, 1, 1, 1])));
}

fn good_subarrays(nums: Vec<u32>) -> u32 {
    let mut difference = HashMap::new();

    let mut sum = 0;
    let mut goods = 0;
    for (i, &x) in nums.iter().enumerate() {
        sum += x;
        if sum == (i + 1) as u32 {
            goods += 1;
        }

        let diff = sum as i32 - (i + 1) as i32;

        if let Some(&v) = difference.get(&diff) {
            goods += v;
        }

        //difference.insert(diff, *difference.get(&diff).unwrap_or(&0) + 1);
        *difference.entry(diff).or_insert(0) += 1;
    }

    goods
}
