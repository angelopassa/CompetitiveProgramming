fn main() {
    println!("Hello, world!");
}

fn find_min(nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut mid = 0;

    while start <= end {
        mid = (start + end) / 2;

        if mid != nums.len() - 1 && nums[mid] > nums[mid + 1] {
            return nums[mid + 1];
        }
        if mid != 0 && nums[mid] < nums[mid - 1] {
            return nums[mid];
        }

        if mid != 0 && nums[start] > nums[mid] {
            end = mid - 1;
        } else if nums[mid] > nums[end] {
            start = mid + 1;
        } else {
            return nums[start];
        }
    }

    return nums[mid];
}
