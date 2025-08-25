use std::collections::{VecDeque};

/// 1493. 删掉一个元素以后全为 1 的最长子数组

pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut queue = VecDeque::new();
    let mut zero_num = 0;
    for i in 0..nums.len() {
        queue.push_back(nums[i]);
        if nums[i] == 0 {
            zero_num += 1;
            if zero_num == 2 {
                ans = ans.max(queue.len() - 1);
                while !queue.is_empty() {
                    if let Some(first) = queue.pop_front() {
                        if first == 0 {
                            zero_num -= 1;
                            break;
                        }
                    }
                }
            }
        }
    }
    if queue.len() == nums.len() {
        return nums.len() as i32 - 1;
    }
    ans = ans.max(queue.len());
    ans as i32 - 1
}


#[test]
fn test1() {
    assert_eq!(longest_subarray(vec![1,1,0,1]), 3);
}

#[test]
fn test2() {
    assert_eq!(longest_subarray(vec![0,1,1,1,0,1,1,0,1]), 5);
}

#[test]
fn test3() {
    assert_eq!(longest_subarray(vec![1,1,1]), 2);
}

#[test]
fn test4() {
    assert_eq!(longest_subarray(vec![1,1,0,0,1,1,1,0,1]), 5);
}