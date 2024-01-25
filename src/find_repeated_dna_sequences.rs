use std::collections::HashMap;

/// ### 187. 重复的DNA序列
///
/// DNA序列 由一系列核苷酸组成，缩写为 'A', 'C', 'G' 和 'T'.。
/// 例如，"ACGAATTCCG" 是一个 DNA序列 。
/// 在研究 DNA 时，识别 DNA 中的重复序列非常有用。
/// 给定一个表示 DNA序列 的字符串 s ，返回所有在 DNA 分子中出现不止一次的 长度为 10 的序列(子字符串)。你可以按 任意顺序 返回答案。

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() < 10 {
        return vec![];
    }
    let mut hash_map = HashMap::new();
    let mut ans = vec![];
    for i in 0..(s.len() - 9) {
        let key = &s[i..(i + 10)];
        hash_map
            .entry(key)
            .and_modify(|f| {
                *f += 1;
                ans.push(String::from(key));
            })
            .or_insert(1);
    }
    ans
}

#[test]
fn find_repeated_dna_sequences_test() {
    let s = String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT");
    let res = vec!["AAAAACCCCC".to_string(), "CCCCCAAAAA".to_string()];
    assert_eq!(find_repeated_dna_sequences(s), res);
}

#[test]
fn find_repeated_dna_sequences_test_1() {
    let s = String::from("AAAAAAAAAAA");
    let res = vec!["AAAAAAAAAA".to_string()];
    assert_eq!(find_repeated_dna_sequences(s), res);
}
