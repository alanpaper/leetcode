/// 2511. 最多可以摧毁的敌人城堡数目
///
pub fn capture_forts(forts: Vec<i32>) -> i32 {
    let mut stack = vec![];
    let mut ans = 0;

    for f in forts {
        if (f == 1 || f == -1) && stack.len() == 0 {
            stack.push(f);
        } else if f == -1 && stack.len() > 0 && stack[0] == 1 {
            if ans < stack.len() - 1 {
                ans = stack.len() - 1;
            }
            stack = vec![];
            stack.push(f);
        } else if f == 1 && stack.len() > 0 && stack[0] == -1 {
            if ans < stack.len() - 1 {
                ans = stack.len() - 1;
            }
            stack = vec![];
            stack.push(f);
        } else if stack.len() > 0 && f == 0 {
            stack.push(f);
        } else {
            stack = vec![];
            stack.push(f);
        }
    }

    ans as i32
}

#[test]
fn test_cap() {
    let forts = vec![1, 0, 0, -1, 0, 0, 0, 0, 1];
    assert_eq!(capture_forts(forts), 4);
}

#[test]
fn test_cap_1() {
    let forts = vec![0, 0, 1, -1];
    assert_eq!(capture_forts(forts), 0);
}
