/// 2416. 字符串的前缀分数和
///
///
pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    let mut ans = vec![];
    for word in &words {
        let mut total = 0;
        let mut i = 0;
        while i < word.len() {
            let sub_word = &word[0..i+1];
            for w in &words {
                if w.len() > i {
                    let sub_word_1 = &w[0..i+1];
                    if sub_word == sub_word_1 {
                        total += 1;
                    }
                } else {
                    break;
                }
            }
            i += 1;
        }
        ans.push(total);
    }
    ans
}

#[test]
fn sum_prefix_scores_test() {
    let words = vec![
        "abc".to_string(),
        "ab".to_string(),
        "bc".to_string(),
        "b".to_string(),
    ];
    assert_eq!(sum_prefix_scores(words), vec![5, 4, 3, 2])
}
