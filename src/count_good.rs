/// 2537. 统计好子数组的数目
/// 给你一个整数数组 nums 和一个整数 k ，请你返回 nums 中 好 子数组的数目。
/// 一个子数组 arr 如果有 至少 k 对下标 (i, j) 满足 i < j 且 arr[i] == arr[j] ，那么称它是一个 好 子数组。
/// 子数组 是原数组中一段连续 非空 的元素序列。
pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {

    



    0
}




#[test]
fn test() {
    assert_eq!(count_good(vec![1,1,1,1,1], 10), 1);
}

#[test]
fn test1() {
    assert_eq!(count_good(vec![3,1,4,3,2,2,4], 2), 4);
}