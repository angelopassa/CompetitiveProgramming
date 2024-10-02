use std::collections::VecDeque;

fn main() {
    println!("{:?}", next_greater_element(Vec::from([11, 13, 21, 3])));
}

fn next_greater_element(nums: Vec<i32>) -> VecDeque<i32> {
    let mut deque = VecDeque::with_capacity(nums.len());
    let mut res = VecDeque::with_capacity(nums.len());

    for n in nums.iter().rev() {
        while !deque.is_empty() && *n >= *deque.front().unwrap() {
            deque.pop_front();
        }

        if deque.is_empty() {
            res.push_front(-1);
        } else {
            res.push_front(*deque.front().unwrap());
        }

        deque.push_front(*n);
    }

    res
}
