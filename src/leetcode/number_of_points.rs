use std::collections::HashSet;
/// 2848. 与车相交的点
/// 给你一个下标从 0 开始的二维整数数组 nums 表示汽车停放在数轴上的坐标。
/// 对于任意下标 i，nums[i] = [starti, endi] ，其中 starti 是第 i 辆车的起点，endi 是第 i 辆车的终点。
/// 返回数轴上被车 任意部分 覆盖的整数点的数目。

pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
    let mut hash_set = HashSet::new();
    for n in nums {
        for i in n[0]..(n[1] + 1) {
            hash_set.insert(i);
        }
    }
    hash_set.len() as i32
}

#[test]
fn test() {
    assert_eq!(
        number_of_points(vec![vec![3, 6], vec![1, 5], vec![4, 7]]),
        7
    );
}

#[test]
fn test_1() {
    assert_eq!(number_of_points(vec![vec![1, 3], vec![5, 8]]), 7);
}
