/// 3151. 特殊数组
/// 如果数组的每一对相邻元素都是两个奇偶性不同的数字，则该数组被认为是一个 特殊数组 。
/// Aging 有一个整数数组 nums。如果 nums 是一个 特殊数组 ，返回 true，否则返回 false。
///
pub fn is_array_special(nums: Vec<i32>) -> bool {
    for i in 1..nums.len() {
        if nums[i] % 2 == nums[i - 1] % 2 {
            return false;
        }
    }
    true
}

#[test]
fn test_1() {
    assert_eq!(is_array_special(vec![1]), true);
}

#[test]
fn test_2() {
    assert_eq!(is_array_special(vec![2, 1, 4]), true);
}

#[test]
fn test_3() {
    assert_eq!(is_array_special(vec![4, 3, 1, 6]), false);
}

/// 3152. 特殊数组II
/// 如果数组的每一对相邻元素都是两个奇偶性不同的数字，则该数组被认为是一个 特殊数组 。
/// 你有一个整数数组 nums 和一个二维整数矩阵 queries，对于 queries[i] = [fromi, toi]，请你帮助你检查
/// 子数组
///  nums[fromi..toi] 是不是一个 特殊数组 。
/// 返回布尔数组 answer，如果 nums[fromi..toi] 是特殊数组，则 answer[i] 为 true ，否则，answer[i] 为 false 。
///
pub fn is_array_special_ii(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut ans = vec![];
    for q in queries {
        let mut is_special = true;
        for i in (q[0] + 1)..(q[1] + 1) {
            if nums[i as usize] % 2 == nums[i as usize - 1] % 2 {
                is_special = false;
            }
        }
        ans.push(is_special);
    }
    ans
}

#[test]
fn test_ii_1() {
    assert_eq!(
        is_array_special_ii(vec![3, 4, 1, 2, 6], vec![vec![0, 4]]),
        vec![false]
    );
}

#[test]
fn test_ii_2() {
    assert_eq!(
        is_array_special_ii(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]]),
        vec![false, true]
    );
}
