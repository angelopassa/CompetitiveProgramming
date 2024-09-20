fn main() {
    println!(
        "Test 1: {}",
        trap(Vec::from([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]))
    );
    println!("Test 1: {}", trap(Vec::from([4, 2, 0, 3, 2, 5])));
}

fn trap(height: Vec<i32>) -> i32 {
    let mut max_l = 0;
    let mut max_r = height.len() - 1;
    let mut i = max_l;
    let mut j = max_r;
    let mut sum = 0;

    while i < j {
        if height[max_l] < height[max_r] {
            i += 1;
            if height[i] < height[max_l] {
                sum += height[max_l] - height[i];
            } else {
                max_l = i;
            }
        } else {
            j -= 1;
            if height[j] < height[max_r] {
                sum += height[max_r] - height[j];
            } else {
                max_r = j;
            }
        }
    }

    sum
}
