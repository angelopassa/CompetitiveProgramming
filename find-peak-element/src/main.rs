fn main() {
    println!("Hello, world!");
}

fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut mid = 0;

    while start < end {
        mid = (start + end) / 2;

        if nums[mid] < nums[mid + 1] {
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    return start.try_into().unwrap();
}
