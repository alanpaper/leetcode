/// 2708. 一个小组的最大实力值
/// 给你一个下标从 0 开始的整数数组 nums ，它表示一个班级中所有学生在一次考试中的成绩。老师想选出一部分同学组成一个 非空 小组，
/// 且这个小组的 实力值 最大，如果这个小组里的学生下标为 i0, i1, i2, ... , ik ，
/// 那么这个小组的实力值定义为 nums[i0] * nums[i1] * nums[i2] * ... * nums[ik​] 。
/// 请你返回老师创建的小组能得到的最大实力值为多少。

pub fn max_strength(nums: Vec<i32>) -> i64 {
    let mut nums = nums;
    nums.sort();
    if nums.len() == 1 {
        return nums[0] as i64;
    }

    let mut num_l = 0;
    let mut num_m = 0;
    for n in &nums {
      if *n < 0 {
        num_l += 1;
      }
      if *n > 0 {
        num_m += 1;
      }
    }
    if num_l <= 1 && num_m == 0 {
      return 0;
    }

    let mut ans: i64 = 1;
    let mut n = 0;
    while n < nums.len() - 1 {
        if nums[n] < 0 && nums[n + 1] < 0 {
            ans *= (nums[n] * nums[n + 1]) as i64;
        }
        if nums[n] > 0 && nums[n + 1] > 0 {
            ans *= (nums[n] * nums[n + 1]) as i64;
        }
        if nums[n] == 0 && nums[n + 1] > 0 {
            ans *= nums[n + 1] as i64;
        }
        if nums[n] < 0 && nums[n + 1] > 0 {
            ans *= nums[n + 1] as i64;
        }
        n += 2;
    }
    if nums.len() % 2 == 1 {
        if let Some(last) = nums.last() {
            if *last > 0 {
                ans *= *last as i64
            }
        }
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(max_strength(vec![3, -1, -5, 2, 5, -9]), 1350);
}

#[test]
fn test_2() {
    assert_eq!(max_strength(vec![-4, -5, -4]), 20);
}
#[test]
fn test_4() {
    assert_eq!(max_strength(vec![0, -4, -7]), 28);
}

#[test]
fn test_5() {
    assert_eq!(max_strength(vec![0, 0, -1]), 0);
}

#[test]
fn test_3() {
    assert_eq!(max_strength(vec![-5, 5, -3, -7, -1, -5, 5, 1, -8]), 105000);
}
