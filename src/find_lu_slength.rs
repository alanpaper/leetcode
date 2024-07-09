/// 522. 最长特殊序列II
///
// 给定字符串列表 strs ，返回其中 最长的特殊序列 的长度。如果最长特殊序列不存在，返回 -1 。
// 特殊序列 定义如下：该序列为某字符串 独有的子序列（即不能是其他字符串的子序列）。
// s 的 子序列可以通过删去字符串 s 中的某些字符实现。
// 例如，"abc" 是 "aebdc" 的子序列，因为您可以删除"aebdc"中的下划线字符来得到 "abc" 。"aebdc"的子序列还包括"aebdc"、 "aeb" 和 "" (空字符串)。
//
// 输入: strs = ["aba","cdc","eae"]
// 输出: 3

// 输入: strs = ["aaa","aaa","aa"]
// 输出: -1
pub fn find_lu_slength(strs: Vec<String>) -> i32 {
    let mut ans = -1;
    for i in 0..strs.len() {
        let mut check = true;
        for j in 0..strs.len() {
            if i != j && is_sub_str(&strs[i], &strs[j]) {
                check = false;
                break;
            }
        }
        if check {
            ans = ans.max(strs[i].len() as i32);
        }
    }
    ans
}

fn is_sub_str(s: &String, t: &String) -> bool {
    let mut pt_s = 0;
    let mut pt_t = 0;

    let str_s = s.chars().collect::<Vec<char>>();
    let str_t = t.chars().collect::<Vec<char>>();

    while pt_s < s.len() && pt_t < t.len() {
        if str_s[pt_s] == str_t[pt_t] {
            pt_s += 1;
        }
        pt_t += 1;
    }
    return pt_s == s.len();
}

#[test]
fn test_1() {
    assert_eq!(
        find_lu_slength(vec![
            "aba".to_string(),
            "cdc".to_string(),
            "eae".to_string()
        ]),
        3
    );
}

#[test]
fn test_2() {
    assert_eq!(
        find_lu_slength(vec!["aaa".to_string(), "aaa".to_string(), "aa".to_string()]),
        -1
    );
}
