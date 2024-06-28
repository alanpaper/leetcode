use std::collections::{HashMap, HashSet};

/// 2670. 找出不同元素数目差数组
///
/// 给你一个下标从 0 开始的数组 nums ，数组长度为 n 。
/// nums 的 不同元素数目差 数组可以用一个长度为 n 的数组 diff 表示，其中 diff[i] 等于前缀 nums[0, ..., i]
/// 中不同元素的数目 减去 后缀 nums[i + 1, ..., n - 1] 中不同元素的数目。
/// 返回 nums 的 不同元素数目差 数组。
/// 注意 nums[i, ..., j] 表示 nums 的一个从下标 i 开始到下标 j 结束的子数组（包含下标 i 和 j 对应元素）。
/// 特别需要说明的是，如果 i > j ，则 nums[i, ..., j] 表示一个空子数组。

pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for n in 0..nums.len() {
        map.entry(nums[n]).or_insert(Vec::new()).push(nums[n]);
    }
    let mut ans = vec![];

    for n in &nums {
        set.insert(n);
        map.entry(*n).and_modify(|f| {
            f.pop();
        });
        if let Some(x) = map.get(n) {
            if x.is_empty() {
                map.remove(n);
            }
        }
        ans.push(set.len() as i32 - map.len() as i32);
    }

    ans
}
#[test]
fn test_1() {
    assert_eq!(
        distinct_difference_array(vec![1, 2, 3, 4, 5]),
        vec![-3, -1, 1, 3, 5]
    )
}

#[test]
fn test_2() {
    assert_eq!(
        distinct_difference_array(vec![3, 2, 3, 4, 2]),
        vec![-2, -1, 0, 2, 3]
    )
}
