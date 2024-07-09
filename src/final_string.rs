/// 2810.故障键盘
pub fn final_string(s: String) -> String {
    let mut stack = vec![];

    for char in s.chars() {
        if char == 'i' {
            let mut stack_copy = vec![];
            for _ in 0..stack.len() {
                stack_copy.push(stack.pop().unwrap());
            }
            stack = stack_copy;
        } else {
            stack.push(char);
        }
    }

    let mut ans = String::new();
    for char in stack {
        ans = format!("{}{}", ans, char);
    }
    ans as String
}

#[test]
fn test() {
    assert_eq!(final_string(String::from("string")), String::from("rtsng"))
}
#[test]
fn test_1() {
    assert_eq!(
        final_string(String::from("poiinter")),
        String::from("ponter")
    )
}
