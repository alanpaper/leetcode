use std::collections::BinaryHeap;
/// 1962. 移除石子使总数最小
pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::from(piles);

    for _ in 0..k {
        if let Some(max) = heap.pop() {
            heap.push(max - max / 2);
        }
    }

    heap.iter().sum()
}

#[test]
fn _test() {
    let roads = vec![4, 3, 6, 7];
    assert_eq!(min_stone_sum(roads, 3), 12);
}
#[test]
fn _test_1() {
    let roads = vec![10000];
    assert_eq!(min_stone_sum(roads, 10000), 1);
}
