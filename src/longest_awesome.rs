/// 1542. 找出最长的超赞子字符串
pub fn longest_awesome(s: String) -> i32 {
    let str_len = s.len();
    let chars = &s.chars().collect::<Vec<char>>();
    let mut index = 0;
    while index < str_len {
        let mut sub_len = str_len;
        while sub_len > 2 {
            let sub_chars = &chars[0..str_len];
            if is_palindrome_str(&sub_chars) {
                return sub_chars.len() as i32;
            }
            sub_len -= 1;
        }
        index += 1;
    }
    1
}

fn is_palindrome_str(chars: &[char]) -> bool {
    let len = chars.len();
    let mut ans = vec![0; 26];
    for char in chars {
        let ascii_value = ((*char as u32) - 97) as usize;
        ans[ascii_value] += 1;
    }
    if len % 2 == 0 {
        for num in ans {
            if num % 2 == 1 {
                return false;
            }
        }
    } else {
        let mut total = 0;
        for num in ans {
            if num % 2 == 1 {
                total += 1;
            }
        }
        if total > 1 {
            return false;
        }
    }
    true
}

#[test]
fn test_1() {
    assert_eq!(longest_awesome(String::from("3242415")), 5)
}
#[test]
fn test_2() {
    assert_eq!(longest_awesome(String::from("12345678")), 1)
}
