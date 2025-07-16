pub fn min_changes(n: i32, k: i32) -> i32 {
    let num = format!("{:b}", n);
    let num1 = format!("{:b}", k);
    println!("{:?} ^ {:?} = {:?} ", num, num1, n ^ k);
    if num.len() != num1.len() {
        return -1;
    }
    let mut ans = 0;

    for c in num.chars() {
        if c == '1' {
            ans += 1;
        }
    }
    ans
}

#[test]
fn test() {
    assert_eq!(min_changes(13, 4), 2);
}
#[test]
fn test_1() {
    assert_eq!(min_changes(21, 21), 0);
}

#[test]
fn test_2() {
    assert_eq!(min_changes(14, 13), -1);
}
