use std::collections::{HashSet, VecDeque};

/// 2813. 子序列最大优雅度
pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: i32) -> i64 {
    items.sort_by(|a, b| b[0].cmp(&a[0]));
    let (mut categorySet, mut st) = (HashSet::new(), VecDeque::new());
    let (mut res, mut profit) = (0 as i64, 0 as i64);
    for (i, item) in items.iter().enumerate() {
        if i < k as usize {
            profit += item[0] as i64;
            if !categorySet.contains(&item[1]) {
                categorySet.insert(item[1]);
            } else {
                st.push_back(item[0]);
            }
        } else if (!categorySet.contains(&item[1]) && !st.is_empty()) {
            profit += (item[0] - st.back().unwrap()) as i64;
            st.pop_back();
            categorySet.insert(item[1]);
        }
        res = res.max(profit + (categorySet.len() * categorySet.len()) as i64);
    }
    res as i64
}

#[test]
fn test_1() {
    let items = vec![vec![0, 2], vec![5, 1], vec![0, 5], vec![0, 6], vec![10, 1]];
    let k = 3;
    assert_eq!(find_maximum_elegance(items, k), 18)
}

#[test]
fn test_2() {
    let items = vec![vec![3, 1], vec![3, 1], vec![2, 2], vec![5, 3]];
    let k = 3;
    assert_eq!(find_maximum_elegance(items, k), 19)
}

#[test]
fn test_3() {
    let items = vec![vec![1, 1], vec![2, 1], vec![3, 1]];
    let k = 3;
    assert_eq!(find_maximum_elegance(items, k), 7)
}
