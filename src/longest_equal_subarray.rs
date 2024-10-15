/// 2831. 找出最长等值子数组
/// 给你一个下标从 0 开始的整数数组 nums 和一个整数 k。
/// 如果子数组中所有元素都相等，则认为子数组是一个 等值子数组 。注意，空数组是 等值子数组。
/// 从 nums 中删除最多 k 个元素后，返回可能的最长等值子数组的长度。
/// 子数组 是数组中一个连续且可能为空的元素序列。
pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut pos_lists = vec![vec![]; nums.len() + 1];
    for (i, &x) in nums.iter().enumerate() {
        let pos = &mut pos_lists[x as usize];
        pos.push(i - pos.len());
    }

    let mut ans = 0;
    for pos in pos_lists {
        let mut left = 0;
        for (right, &p) in pos.iter().enumerate() {
            while p - pos[left] > k as usize {
                // 要删除的数太多了
                left += 1;
            }
            ans = ans.max(right - left + 1);
        }
    }
    0
}

#[test]
fn test_1() {
    assert_eq!(longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3), 3)
}

#[test]
fn test_2() {
    assert_eq!(longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2), 4)
}
