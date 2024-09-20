use std::collections::VecDeque;

fn main() {
    println!(
        "Test 1: {:?}",
        max_sliding_window(Vec::from([1, 3, -1, -3, 5, 3, 6, 7]), 3)
    );
    println!("Test 2: {:?}", max_sliding_window(Vec::from([1]), 1));
}

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut maxs = Vec::new();
    let mut deque = VecDeque::new();

    let mut j = 0;
    for i in 0..nums.len() {
        while deque.back().is_some_and(|&x| x < nums[i]) {
            deque.pop_back();
        }

        deque.push_back(nums[i]);

        let c: i32 = (i - j + 1).try_into().expect("Error");

        if c == k {
            maxs.push(*deque.front().expect("Error"));
            if *deque.front().expect("Error") == nums[j] {
                deque.pop_front();
            }

            j += 1;
        }
    }

    maxs
}
