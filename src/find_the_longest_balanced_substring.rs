///  2609. 最长平衡子字符串

pub fn find_the_longest_balanced_substring(s: String) -> i32 {
    let mut stack_len = vec![];
    let mut stack = vec![];

    let mut is_pop = false;
    let mut pre_len = 0;
    for char in s.chars() {
        if char == '0' && !is_pop {
            stack.push(char);
            pre_len = stack.len();
        } else if char == '0' && is_pop {
            stack_len.push((pre_len - stack.len()) * 2);
            stack = vec![];
            pre_len = 0;
            is_pop = false;
            stack.push(char);
        } else {
            if !is_pop && stack.len() == 0 {
                pre_len = 0;
            } else {
                is_pop = true;
                match stack.pop() {
                    Some(_) => {}
                    None => {
                        stack_len.push((pre_len - stack.len()) * 2);
                        pre_len = 0;
                        stack = vec![];
                        is_pop = false;
                    }
                }
            }
        }
    }

    if is_pop {
        stack_len.push((pre_len - stack.len()) * 2);
    }

    if let Some(&max) = stack_len.iter().max() {
        return max as i32;
    }
    0
}

#[test]
fn _test() {
    let s = String::from("01000111");
    assert_eq!(find_the_longest_balanced_substring(s), 6);
}

#[test]
fn _test_1() {
    let s = String::from("011011");
    assert_eq!(find_the_longest_balanced_substring(s), 2);
}

#[test]
fn _test_2() {
    let s = String::from("00111");
    assert_eq!(find_the_longest_balanced_substring(s), 4);
}
