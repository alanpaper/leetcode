/// 228. 汇总区间
///
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut stack = vec![];
    let mut result = vec![];
    for num in nums {
        match stack.last() {
            Some(last) => {
                if num - *last == 1 {
                    stack.push(num);
                } else {
                    if stack.len() > 1 {
                        result.push(format!("{}->{}", stack[0], stack.last().unwrap()));
                        stack = vec![];
                    } else {
                        result.push(stack.pop().unwrap().to_string());
                    }
                    stack.push(num);
                }
            }
            None => {
                stack.push(num);
            }
        }
    }

    if stack.len() > 1 {
        result.push(format!("{}->{}", stack[0], stack.last().unwrap()));
    } else if stack.len() == 1 {
        result.push(stack.pop().unwrap().to_string());
    }

    result
}

#[test]
fn test_summary_ranges() {
    let larger_len: Vec<String> = summary_ranges(vec![]);
    assert_eq!(larger_len, vec!["0->2", "4->5", "7"])
}
