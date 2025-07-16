/// 2661.找出完整访问过一遍的行或列对应的arr下标i
pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let mut position = vec![(0, 0); arr.len()];
    let n = mat.len();
    let m = mat[0].len();

    for i in 0..n {
        for j in 0..m {
            position[(mat[i][j] - 1) as usize] = (i, j);
        }
    }

    let mut col = vec![0; n];
    let mut row = vec![0; m];
    for (i, num) in arr.iter().enumerate() {
        if let Some((x, y)) = position.get((*num - 1) as usize) {
            col[*x] += 1;
            row[*y] += 1;
            if col[*x] == m || row[*y] == n {
                return i as i32;
            }
        }
    }
    0
}

#[test]
fn _test() {
    let arr = vec![6, 2, 3, 1, 4, 5];
    let mat = vec![vec![5, 1], vec![2, 4], vec![6, 3]];
    assert_eq!(first_complete_index(arr, mat), 2);
}

#[test]
fn _test_one() {
    let arr = vec![2, 8, 7, 4, 1, 3, 5, 6, 9];
    let mat = vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]];
    assert_eq!(first_complete_index(arr, mat), 3);
}
