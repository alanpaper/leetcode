/// 45. 跳跃游戏II
/// 给定一个长度为 n 的 0 索引整数数组 nums。初始位置为 nums[0]。
/// 每个元素 nums[i] 表示从索引 i 向后跳转的最大长度。换句话说，如果你在 nums[i] 处，你可以跳转到任意 nums[i + j] 处:
/// 0 <= j <= nums[i] 
/// i + j < n
/// 返回到达 nums[n - 1] 的最小跳跃次数。生成的测试用例可以到达 nums[n - 1]。

pub fn jump(nums: Vec<i32>) -> i32 {
  let mut max_index = 0;
  let mut steps = 0;
  let mut end = 0;
  for i in 0..nums.len() - 1 {
    if max_index >= i {
      max_index = max_index.max(i + nums[i] as usize);
      if i == end {
        end = max_index;
        steps += 1;
      }
    }
  }
  steps
}

#[test]
fn jump_test() {
  assert_eq!(jump(vec![2,3,1,1,4]), 2);
}
#[test]
fn jump_test_1() {
  assert_eq!(jump(vec![2,3,0,1,2,2,0,1]), 4);
}

#[test]
fn jump_test_2() {
  assert_eq!(jump(vec![1,2]), 1);
}

#[test]
fn jump_test_3() {
  assert_eq!(jump(vec![2,1]), 1);
}
