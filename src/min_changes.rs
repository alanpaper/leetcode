pub fn min_changes(n: i32, k: i32) -> i32 {
    let num = format!("{:b}", n ^ k);
    let mut ans = 0;
    for c in num.chars() {
        if c == '1' {
            ans += 1;
        }
    }
    ans
}
