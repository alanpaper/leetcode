/// 2535. 数组元素和数字和的绝对差
/// 给你一个正整数数组 nums 。
/// 元素和 是 nums 中的所有元素相加求和。
/// 数字和 是 nums 中每一个元素的每一数位（重复数位需多次求和）相加求和。
/// 返回 元素和 与 数字和 的绝对差。
/// 注意：两个整数 x 和 y 的绝对差定义为 |x - y| 。

pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut sum = 0;
    for n in nums {
        total += n;
        sum += sum_number(n);
    }
    (total - sum).abs()
}

fn sum_number(num: i32) -> i32 {
    let mut sum = 0;
    let mut num = num;
    while num > 0 {
        let remainder = num % 10;
        sum += remainder;
        num -= remainder;
        num /= 10;
    }
    sum
}

#[test]
fn test_1() {
    assert_eq!(difference_of_sum(vec![1, 15, 6, 3]), 9);
}

#[test]
fn test_2() {
    assert_eq!(difference_of_sum(vec![1, 2, 3, 4]), 0);
}
