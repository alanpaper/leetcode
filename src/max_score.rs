/// 1423. 可获得的最大点数
///

pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;

    let len = card_points.len() as i32;

    ans
}

#[test]
fn test() {
    assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
}
