/// 1423. 可获得的最大点数
///
/// 1. 获取最大点数困难 反向判断求出剩余牌的最小点数
/// 2. 滑动窗口最佳
/// 3. 下面是暴力遍历求的最小值

pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let min_num = card_points.len() - (k as usize);

    let mut total = 0;
    for num in &card_points {
        total += num;
    }

    let mut min_val = total;
    for num in 0..(card_points.len() - min_num + 1) {
        let mut mid = 0;
        for index in num..(num + min_num) {
            mid += card_points[index];
        }
        min_val = min_val.min(mid);
    }

    total - min_val
}

#[test]
fn test() {
    assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
}
