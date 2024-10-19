use std::collections::HashMap;

fn main() {
    println!(
        "{:?}",
        longest_k_good_segment(vec![1, 1, 1, 2, 3, 4, 4, 5, 6, 6, 1, 2, 3, 4, 5, 6], 2)
    );
}

fn longest_k_good_segment(v: Vec<i32>, k: u32) -> i32 {
    let mut hm: HashMap<i32, u32> = HashMap::new();

    let mut left = 0;
    let mut out = 0;
    for (i, x) in v.iter().enumerate() {
        if let Some(&value) = hm.get(x) {
            if value + 1 > k {
                while v[left] != *x {              
                    hm.insert(v[left], *hm.get(&v[left]).unwrap() - 1);
                    left += 1;
                }
                left += 1;
            } else {
                hm.insert(*x, *hm.get(&v[left]).unwrap() + 1);
            }
        } else {
            hm.insert(*x, 1);
        }

        if i - left + 1 > out {
            out = i - left + 1;
        }
    }

    out.try_into().unwrap()
}
