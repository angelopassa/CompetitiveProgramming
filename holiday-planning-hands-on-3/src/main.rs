use std::fs;

const N_TEST: usize = 5;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for test_num in 0..N_TEST {
        print!("Executing Test #{}  ", test_num);
        let content = fs::read_to_string(format!("TestSet/input{}.txt", test_num))?;

        let rows: Vec<&str> = content.split('\n').collect();

        let n: Vec<usize> = rows[0]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let (n_cities, n_days) = (n[0], n[1]);

        let mut matrix = vec![vec![0; n_days]; n_cities];

        for i in 1..rows.len() {
            for (j, num) in rows[i].split_whitespace().enumerate() {
                matrix[i - 1][j] = num.parse()?;
            }
        }

        let mut res = holiday_planning(matrix).to_string();
        res.push('\n');

        let output = fs::read_to_string(format!("TestSet/output{}.txt", test_num))?;

        assert_eq!(output, res);
        println!("Passed!");
    }

    Ok(())
}

fn holiday_planning(cities: Vec<Vec<usize>>) -> usize {
    let mut dp = vec![vec![0; cities[0].len() + 1]; cities.len() + 1];

    for i in 1..cities.len() + 1 {
        for j in 1..cities[0].len() + 1 {
            let mut max = dp[i - 1][j];
            let mut pref_sum = 0;

            for k in 0..j {
                pref_sum += cities[i - 1][k];
                max = std::cmp::max(max, dp[i - 1][j - k - 1] + pref_sum);
            }

            dp[i][j] = max;
        }
    }

    dp[cities.len()][cities[0].len()]
}
