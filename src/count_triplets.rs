/// 982. 按位与为零的三元组
/// 给你一个整数数组 nums ，返回其中 按位与三元组 的数目。
/// 按位与三元组 是由下标 (i, j, k) 组成的三元组，并满足下述全部条件：
/// 0 <= i < nums.length
/// 0 <= j < nums.length
/// 0 <= k < nums.length
/// nums[i] & nums[j] & nums[k] == 0 ，其中 & 表示按位与运算符。
///

pub fn count_triplets(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    for i in nums.iter().enumerate() {
        for j in nums.iter().enumerate() {
            for k in nums.iter().enumerate() {
                if i.1 & j.1 & k.1 == 0 {
                    total += 1;
                }
            }
        }
    }

    total
}

#[test]
fn test_count_triplets() {
    let nums = vec![2, 1, 3];
    assert_eq!(count_triplets(nums), 12);
}
#[test]
fn test1_count_triplets() {
    let nums = vec![0, 0, 0];
    assert_eq!(count_triplets(nums), 27);
}
