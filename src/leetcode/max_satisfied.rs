/// 1052. 爱生气的书店老板
pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let mut ans = 0;
    for g in 0..(grumpy.len() - minutes as usize + 1) {
        let mid_customer = &customers[g..g + minutes as usize];
        let mid_grumpy = &grumpy[g..g + minutes as usize];
        let mut total = 0;
        for num in 0..minutes {
            if mid_grumpy[num as usize] == 1 {
                total += mid_customer[num as usize];
            }
        }
        ans = ans.max(total);
    }

    let mut total = 0;
    for g in grumpy.iter().enumerate() {
        if *g.1 == 0 {
            total += customers[g.0];
        }
    }

    total + ans
}

#[test]
fn test() {
    assert_eq!(
        max_satisfied(vec![6, 10, 2, 1, 7, 9], vec![1, 0, 0, 0, 0, 1], 3),
        29
    )
}
