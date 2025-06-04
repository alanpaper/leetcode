/// 2894. 分类求和并作差
/// 给你两个正整数 n 和 m 。
/// 现定义两个整数 num1 和 num2 ，如下所示：
/// num1：范围 [1, n] 内所有 无法被 m 整除 的整数之和。
/// num2：范围 [1, n] 内所有 能够被 m 整除 的整数之和。
/// 返回整数 num1 - num2 。

pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    let mut num1 = 0;
    let mut num2 = 0;

    for i in 0..(n + 1) {
        if i % m == 0 {
            num2 += 1;
        }
    }

    n - num2 - num2
}

