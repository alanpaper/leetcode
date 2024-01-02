/// ### 无重复字符的最长子串
///
/// 提示：
/// + 0 <= s.length <= 5 * 10000
/// + `s`由英文字母、数字、符号和空格组成
///
/// 题解：滑动窗口
///
/// 1. 根据要求s由英文、数字、符号、空格组成 定义一个长度为128的vec用来存放字符位置
/// 2. 用vec记录每个字符最后出现的位置
/// 3. 字符最后出现的位置与left边界进行比较 如果大于left边界 则更新left
/// 4. 用ans记录滑动窗口的大小
/// 5. 每次循环更新计算滑动窗口的大小与上一个最大ans做比较
/// 6. 返回最大ans
///
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut last: [i32; 128] = [-1; 128];
    let mut left = -1;
    let mut ans = 0;
    for (i, v) in s.chars().enumerate() {
        println!("{:?}--{:?}", left, ans);
        left = left.max(last[v as usize]);
        last[v as usize] = i as i32;
        ans = ans.max((i as i32) - left);
    }
    println!("{:?}", last);
    return ans;
}

#[cfg(test)]
mod tests {
    use crate::longest;
    #[test]
    fn longest_substring() {
        let larger_len = longest::length_of_longest_substring("abcdefg".to_string());
        assert_eq!(larger_len, 7)
    }
}
