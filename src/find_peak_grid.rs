use std::collections::HashMap;

/// 1901. 寻找峰值 II

pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = vec![];

    for x in mat.iter().enumerate() {
        let max_y = maxElement(&mat[x.0]);
        ans.push(vec![x.0 as i32, max_y as i32, mat[x.0][max_y]])
    }

    ans.sort_by(|a, b| a[2].cmp(&b[2]));

    match ans.pop() {
        Some(mut last) => {
            last.pop();
            last
        }
        None => {
            vec![]
        }
    }
}

fn maxElement(arr: &Vec<i32>) -> usize {
    let mut i = 0;
    for j in arr.iter().enumerate() {
        if arr[i] < *j.1 {
            i = j.0;
        }
    }
    i
}

#[test]
fn _test() {
    assert_eq!(find_peak_grid(vec![vec![1, 4], vec![3, 2]]), vec![0, 1])
}
