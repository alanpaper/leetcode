/// 670. 最大交换
/// 给定一个非负整数，你至多可以交换一次数字中的任意两位。返回你能得到的最大值。

pub fn maximum_swap(num: i32) -> i32 {
    let mut max_num = 0;
    let binding = num.to_string();
    let mut nums = binding.chars();

    0
}

#[test]
fn _test() {
    assert_eq!(maximum_swap(2756), 7256);
}
