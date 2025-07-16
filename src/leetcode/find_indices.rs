/// 2903. 找出满足差值条件的下标I
/// 给你一个下标从 0 开始、长度为 n 的整数数组 nums ，以及整数 indexDifference 和整数 valueDifference 。
/// 你的任务是从范围 [0, n - 1] 内找出  2 个满足下述所有条件的下标 i 和 j ：
/// abs(i - j) >= indexDifference 且
/// abs(nums[i] - nums[j]) >= valueDifference
/// 返回整数数组 answer。如果存在满足题目要求的两个下标，则 answer = [i, j] ；否则，answer = [-1, -1] 。
/// 如果存在多组可供选择的下标对，只需要返回其中任意一组即可。
/// 注意：i 和 j 可能 相等 。

pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
    if nums.len() == 1 && index_difference == 0 && value_difference == 0 {
        return vec![0, 0];
    }

    let mut queue = vec![];
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if (i as i32 - j as i32).abs() >= index_difference
                && (nums[i] - nums[j]).abs() >= value_difference
            {
                queue.push((i, j));
            }
        }
    }

    println!("{:?}", queue);

    if queue.len() >= 2 {
        return vec![queue[0].0 as i32, queue[0].1 as i32];
    }
    vec![-1, -1]
}

#[test]
fn test_1() {
    assert_eq!(find_indices(vec![5, 1, 4, 1], 2, 4), vec![0, 3]);
}
#[test]
fn test_2() {
    assert_eq!(find_indices(vec![2, 1], 0, 0), vec![0, 0]);
}
#[test]
fn test_3() {
    assert_eq!(find_indices(vec![1, 2, 3], 2, 4), vec![-1, -1]);
}
#[test]
fn test_4() {
    assert_eq!(find_indices(vec![0], 0, 0), vec![0, 0]);
}
