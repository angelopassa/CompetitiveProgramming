use std::cmp::max;

fn main() {
    println!("{}", lcs("ABCDGH", "AEDFHR"));
}

fn lcs(s1: &str, s2: &str) -> u64 {
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let mut matrix = vec![vec![0; s1.len()]; s2.len()];

    matrix[0][0] = if s2[0] == s1[0] { 1 } else { 0 };

    for j in 1..s1.len() {
        matrix[0][j] = if s2[0] == s1[j] { 1 } else { matrix[0][j - 1] };
    }

    for i in 1..s2.len() {
        matrix[i][0] = if s2[i] == s1[0] { 1 } else { matrix[i - 1][0] };
    }

    for i in 1..s2.len() {
        for j in 1..s1.len() {
            if s2[i] == s1[j] {
                matrix[i][j] = *[matrix[i - 1][j - 1] + 1, matrix[i - 1][j], matrix[i][j - 1]]
                    .iter()
                    .max()
                    .unwrap();
            } else {
                matrix[i][j] = max(matrix[i - 1][j], matrix[i][j - 1]);
            }
        }
    }

    matrix[s2.len() - 1][s1.len() - 1]
}
