/// 318. 最大单词长度乘积
///
///
///

pub fn max_product(words: Vec<String>) -> i32 {
    let mut bits_words = vec![0; words.len()];
    for i in 0..words.len() {
        for char in words[i].chars() {
            bits_words[i] |= 1 << (char as i32 - 97);
        }
    }

    println!("{:?}", bits_words);

    let mut ans = 0;
    for (index, w) in words.iter().enumerate() {
        for j in (index + 1)..words.len() {
            if bits_words[index] & bits_words[j] == 0 {
                ans = ans.max(w.len() * words[j].len());
            }
        }
    }

    ans as i32
}

#[test]
fn test_() {
    let words = vec![
        "abcw".to_string(),
        "baz".to_string(),
        "foo".to_string(),
        "bar".to_string(),
        "xtfn".to_string(),
        "abcdef".to_string(),
    ];
    assert_eq!(max_product(words), 16);
}
