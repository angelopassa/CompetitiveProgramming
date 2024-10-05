fn main() {
    println!("Hello, world!");
}

fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut start = 0;
    let mut end = TryInto::<i32>::try_into(nums.len()).unwrap() - 1;
    let mut mid: i32;
    let mut i = -1;

    while start <= end {
        mid = (start + end) / 2;

        if nums[TryInto::<usize>::try_into(mid).unwrap()] == target {
            i = mid;
            end = mid - 1;
        } else if nums[TryInto::<usize>::try_into(mid).unwrap()] < target {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    start = 0;
    end = TryInto::<i32>::try_into(nums.len()).unwrap() - 1;
    let mut j = -1;

    while start <= end {
        mid = (start + end) / 2;

        if nums[TryInto::<usize>::try_into(mid).unwrap()] == target {
            j = mid;
            start = mid + 1;
        } else if nums[TryInto::<usize>::try_into(mid).unwrap()] < target {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    let out = Vec::from([i, j]);
    out
}
