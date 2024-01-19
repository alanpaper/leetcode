use std::collections::HashMap;

/// 2661.找出完整访问过一遍的行或列对应的arr下标i
pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let mut map_stack = HashMap::new();
    for x in mat.iter().enumerate() {
        for y in x.1.iter().enumerate() {
            map_stack.insert(y.1, (x.0, y.0));
        }
    }
    let mut row = vec![0; mat.len()];
    let mut col = vec![0; mat[0].len()];
    for (i, num) in arr.iter().enumerate() {
        if let Some(&x) = map_stack.get(&num) {
            row[x.0] += 1;
            col[x.1] += 1;
            if row[x.0] == mat[0].len() || col[x.1] == mat.len() {
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
