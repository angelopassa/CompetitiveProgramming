use std::fs;

const N_TEST: usize = 11;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for test_num in 0..N_TEST {
        print!("Executing Test #{}  ", test_num);
        let content = fs::read_to_string(format!("TestSet/input{}.txt", test_num))?;

        let rows: Vec<&str> = content.split('\n').filter(|s| !s.is_empty()).collect();
        let n: usize = rows[0].parse()?;

        let mut topics = vec![(0, 0); n];

        for i in 1..rows.len() {
            let nums = rows[i].split_whitespace().collect::<Vec<&str>>();
            topics[i - 1] = (nums[0].parse()?, nums[1].parse()?);
        }

        let mut res = design_a_course(&mut topics).to_string();
        res.push('\n');

        let output = fs::read_to_string(format!("TestSet/output{}.txt", test_num))?;

        assert_eq!(output, res);
        println!("Passed!");
    }

    Ok(())
}

fn design_a_course(topics: &mut Vec<(usize, usize)>) -> usize {
    topics.sort();

    let mut sequence = Vec::with_capacity(topics.len());

    sequence.push(topics[0]);

    for &mut t in topics {
        if gt_tuple(t, *sequence.last().unwrap()) {
            sequence.push(t);
        } else {
            let idx = binary_search(&sequence, t);

            if sequence[idx].0 != t.0 || sequence[idx].1 > t.1 {
                sequence[idx] = t;
            }
        }
    }

    sequence.len()
}

fn binary_search(vector: &[(usize, usize)], k: (usize, usize)) -> usize {
    let mut i = 0;
    let mut j = vector.len() as i32 - 1;
    let mut mid = 0;

    while i <= j {
        mid = (i + j) / 2;

        if gt_tuple(k, vector[mid as usize]) {
            i = mid + 1;
        } else {
            j = mid - 1;
        }
    }

    if gt_tuple(k, vector[mid as usize]) {
        mid as usize + 1
    } else {
        mid as usize
    }
}

fn gt_tuple(t1: (usize, usize), t2: (usize, usize)) -> bool {
    t1.0 > t2.0 && t1.1 > t2.1
}
