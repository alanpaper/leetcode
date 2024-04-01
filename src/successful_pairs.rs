pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut ans = vec![];
    for spell in spells {
        let mut total = 0;
        for point in &potions {
            if (spell * *point) as i64 >= success {
                total += 1;
            }
        }
        ans.push(total);
    }
    ans
}

#[test]
fn test() {
    assert_eq!(
        successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
        vec![4, 0, 3]
    )
}

#[test]
fn test_1() {
    assert_eq!(
        successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
        vec![2, 0, 2]
    )
}
