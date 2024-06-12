/// 3038. 相同分数的最大操作数目
///

pub fn max_operations(nums: Vec<i32>) -> i32 {
    let score = nums[0] + nums[1];
    let mut ans = 1;
    let mut index = 2;
    while index < nums.len() - 1 {
        if (nums[index] + nums[index + 1]) != score {
            break;
        }
        ans += 1;
        index += 2;
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(max_operations(vec![3, 2, 1, 4, 5]), 2)
}

#[test]
fn test_2() {
    assert_eq!(max_operations(vec![3, 2, 6, 1, 4]), 1)
}
#[test]
fn test_3() {
    assert_eq!(max_operations(vec![1, 1, 1, 1, 1, 1]), 3)
}
