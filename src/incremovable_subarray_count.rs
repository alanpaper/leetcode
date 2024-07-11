use std::collections::VecDeque;
/// 2972. 统计移除递增子数组的数目II
// 给你一个下标从 0 开始的 正 整数数组 nums 。
// 如果 nums 的一个子数组满足：移除这个子数组后剩余元素 严格递增，
// 那么我们称这个子数组为 移除递增 子数组。比方说，[5, 3, 4, 6, 7] 中的 [3, 4]
// 是一个移除递增子数组，因为移除该子数组后，[5, 3, 4, 6, 7] 变为 [5, 6, 7]是严格递增的。
// 请你返回 nums 中 移除递增 子数组的总数目。
// 注意 ，剩余元素为空的数组也视为是递增的。
// 子数组 指的是一个数组中一段连续的元素序列。
pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
    let mut ans_arr = vec![];
    let mut ans = 1;

    if nums.len() == 1 {
        return 1;
    }

    let mut queue = VecDeque::new();
    queue.push_back(0);
    while !queue.is_empty() {
        if let Some(i) = queue.pop_front() {
            let mut arr = vec![];
            arr.push(nums[i]);
            let mut pre = nums[i];
            let mut save = false;
            for index in (i + 1)..nums.len() {
                if nums[index] > pre {
                    arr.push(nums[index]);
                    pre = nums[index];
                } else if !save {
                    save = true;
                    queue.push_back(index);
                }
            }
            ans_arr.push(arr.clone());
            ans += (&arr.len() - 1 + 1) * (&arr.len() - 1) / 2;
        }
    }

    if ans_arr.len() == 1 {
        ans = (nums.len() - 1 + 1) * (nums.len() - 1) / 2 + nums.len() - 1 + 1
    }
    if ans_arr.len() == nums.len() {
        ans = 3
    }
    if ans_arr.len() == 2 || ans_arr.len() != nums.len() {
        ans += 2;
    }
    ans as i32
}

// 3 + 2 + 1

// 2 + 1

// 1

#[test]
fn test_1() {
    assert_eq!(incremovable_subarray_count(vec![1, 2, 3, 4]), 10);
}

#[test]
fn test_2() {
    assert_eq!(incremovable_subarray_count(vec![6, 5, 7, 8]), 7);
}

// 2 + 1
// 2 + 1
// 1

#[test]
fn test_3() {
    assert_eq!(incremovable_subarray_count(vec![8, 7, 6, 6, 5]), 3);
}

#[test]
fn test_4() {
    assert_eq!(incremovable_subarray_count(vec![2, 1, 6]), 5);
}

// 4 - 3 + 1

// 1
