use std::collections::{HashSet};



pub fn set_zeroes(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let m = matrix.len();
    let n = matrix[0].len();
    let mut zero_row = HashSet::new();
    let mut zero_col = HashSet::new();

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                zero_row.insert(i);
                zero_col.insert(j);
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            if zero_row.contains(&i) || zero_col.contains(&j) {
                matrix[i][j] = 0;
            }
        }
    }

    vec![]
}



#[test]
fn test_1() {
    assert_eq!(set_zeroes(vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]]),  vec![vec![1,0,1], vec![0,0,0], vec![1,0,1]]);
}






