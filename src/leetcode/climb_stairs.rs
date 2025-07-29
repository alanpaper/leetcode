/// 70. 爬楼梯
///
/// 1 2 3 4
pub fn climb_stairs(n: i32) -> i32 {
    let mut ans_stack = vec![0; (n + 1) as usize];
    ans_stack[0] = 1;
    ans_stack[1] = 1;
    for num in 2..(n + 1) {

        ans_stack[num as usize] = ans_stack[(num - 2) as usize] + ans_stack[(num - 1) as usize]
    }
    println!("{:?}", ans_stack);
    ans_stack.pop().unwrap() 
}

#[test]
fn test() {
    assert_eq!(climb_stairs(2), 2);
}

#[test]
fn test_1() {
    assert_eq!(climb_stairs(3), 3);
}
