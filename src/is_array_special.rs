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
