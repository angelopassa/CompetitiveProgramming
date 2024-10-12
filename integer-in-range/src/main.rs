fn main() {
    println!("Hello, world!");
}

fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut v = vec![0; 52];

    for range in ranges {
        v[TryInto::<usize>::try_into(range[0]).unwrap()] += 1;
        v[TryInto::<usize>::try_into(range[1]).unwrap() + 1] -= 1;
    }

    let mut is_in = 0;
    for i in 0..v.len() {
        is_in += v[i];
        if i >= left.try_into().unwrap() && i <= right.try_into().unwrap() && is_in == 0 {
            return false;
        }
    }

    true
}
