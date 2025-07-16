use std::cmp::Ordering;
/// 35. 搜索插入位置
///
/// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
/// 请必须使用时间复杂度为 O(log n) 的算法。
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let middle = (right + left) / 2;
        match nums[middle].cmp(&target) {
            Ordering::Less => left = middle + 1,
            Ordering::Equal | Ordering::Greater => right = middle,
        }
    }
    left as i32
}

#[test]
fn test_1() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
}

#[test]
fn test_2() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
}

#[test]
fn test_3() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
}
