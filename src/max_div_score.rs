/// 2644. 找出可整除性得分最大的整数
/// 给你两个整数数组 nums 和 divisors 。
/// divisors[i] 的 可整除性得分 等于满足 nums[j] 能被 divisors[i] 整除的下标 j 的数量。
/// 返回 可整除性得分 最大的整数 divisors[i] 。如果有多个整数具有最大得分，则返回数值最小的一个。
pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut num = -1;
    for d in &divisors {
        let mut min_num = 0;
        for n in &nums {
            if n % d == 0 {
                min_num += 1;
            }
        }
        if min_num > num {
            num = min_num;
            ans = *d;
        } else if min_num == num {
            ans = ans.min(*d);
        }
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(max_div_score(vec![2, 9, 15, 50], vec![5, 3, 7, 2]), 2);
}
#[test]
fn test_2() {
    assert_eq!(max_div_score(vec![4, 7, 9, 3, 9], vec![5, 2, 3]), 3);
}
