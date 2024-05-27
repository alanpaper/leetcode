use core::num;

/// 1673.找出最具竞争力的子序列
///
/// 给你一个整数数组 nums 和一个正整数 k ，返回长度为 k 且最具 竞争力 的 nums 子序列。
/// 数组的子序列是从数组中删除一些元素（可能不删除元素）得到的序列。
/// 在子序列 a 和子序列 b 第一个不相同的位置上，如果 a 中的数字小于 b 中对应的数字，
/// 那么我们称子序列 a 比子序列 b（相同长度下）更具 竞争力 。 例如，[1,3,4] 比 [1,3,5] 更具竞争力，
/// 在第一个不相同的位置，也就是最后一个位置上， 4 小于 5 。
///
pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let len = nums.len();
    let mut ans = vec![];

    for i in 0..len {
        while !ans.is_empty()
            && (ans.len() + len - i > k as usize)
            && *ans.last().unwrap() > nums[i]
        {
            ans.pop();
        }
        ans.push(nums[i]);
    }
    let mut result = vec![];
    for i in 0..k {
        result.push(ans[i as usize]);
    }
    result
}

#[test]
fn test_1() {
    assert_eq!(most_competitive(vec![3, 5, 2, 6], 2), [2, 6])
}
#[test]
fn test_2() {
    assert_eq!(
        most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4),
        [2, 3, 3, 4]
    )
}

#[test]
fn test_3() {
    assert_eq!(
        most_competitive(vec![71, 18, 52, 29, 55, 73, 24, 42, 66, 8, 80, 2], 3),
        [8, 80, 2]
    )
}
