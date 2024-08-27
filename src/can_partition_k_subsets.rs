/// 698. 划分为k个相等的子集
///
pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let mut last = nums.len() - 1;
    let mut nums = nums.clone();
    nums.sort();

    false
}

#[test]
fn test_1() {
    let nums = vec![4, 3, 2, 3, 5, 2, 1];
    let k = 4;
    assert_eq!(can_partition_k_subsets(nums, k), true);
}

#[test]
fn test_2() {
    let nums = vec![1, 2, 3, 4];
    let k = 3;
    assert_eq!(can_partition_k_subsets(nums, k), false);
}
