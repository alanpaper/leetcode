use std::collections::BinaryHeap;

/// 2558. 从数量最多的堆取走礼物
/// 题解
/// 最大堆
/// 
pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
  let mut heap = BinaryHeap::from(gifts);
  for _ in 0..k {
    match heap.pop() {
        Some(top)=> {
          heap.push((top as f64).sqrt() as i32);
          if *heap.peek().unwrap() == 1 {
            break;
          }
        }
        None=>{}
    }
  }
  heap.iter().map(|&val| val as i64).sum()
}
