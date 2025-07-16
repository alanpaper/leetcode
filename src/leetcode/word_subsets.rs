use std::collections::HashMap;
/// 916. 单词子集
pub fn word_subsets<'a>(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    let mut stack = vec![];
    let word2_map = word2_merge(words2);

    println!("{:?}", word2_map);

    for word in words1 {
        let mut ans = true;
        let mut word_map = HashMap::new();
        for c in word.chars() {
            word_map.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        for m in &word2_map {
            match word_map.get(&m.0) {
                Some(&x) => {
                    if x < *m.1 {
                        ans = false;
                    }
                }
                None => ans = false,
            }
        }

        if ans {
            stack.push(word);
        }
    }
    stack
}

fn word2_merge(words: Vec<String>) -> HashMap<char, i32> {
    // words中每个字符在字符串中出现的最大次数
    let mut word2_map = HashMap::new();
    for w in words {
        let mut word_sub_map = HashMap::new();
        for c in w.chars() {
            word_sub_map.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        for m in word_sub_map {
            match word2_map.get(&m.0) {
                Some(&x) => {
                    if x < m.1 {
                        word2_map.insert(m.0, m.1);
                    }
                }
                None => {
                    word2_map.entry(m.0).or_insert(m.1);
                }
            }
        }
    }
    word2_map
}

#[test]
fn test_word_subsets() {
    let words1 = vec![
        "amazon".to_string(),
        "apple".to_string(),
        "facebook".to_string(),
        "google".to_string(),
        "leetcode".to_string(),
    ];
    let words2 = vec!["e".to_string(), "o".to_string()];

    assert_eq!(
        word_subsets(words1, words2),
        vec![
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string()
        ]
    )
}
#[test]
fn test1_word_subsets() {
    let words1 = vec![
        "amazon".to_string(),
        "apple".to_string(),
        "facebook".to_string(),
        "google".to_string(),
        "leetcode".to_string(),
    ];
    let words2 = vec!["ec".to_string(), "oc".to_string(), "ceo".to_string()];

    assert_eq!(word_subsets(words1, words2), vec!["facebook", "leetcode"])
}

#[test]
fn test2_word_subsets() {
    let words1 = vec![
        "acaac".to_string(),
        "cccbb".to_string(),
        "aacbb".to_string(),
        "caacc".to_string(),
        "bcbbb".to_string(),
    ];
    let words2 = vec!["c".to_string(), "cc".to_string(), "b".to_string()];

    assert_eq!(word_subsets(words1, words2), vec!["cccbb"])
}
