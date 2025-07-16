/// 1845. 座位预约管理系统
///
/// 请你设计一个管理 n 个座位预约的系统，座位编号从 1 到 n 。
/// 请你实现 SeatManager 类：
/// SeatManager(int n) 初始化一个 SeatManager 对象，它管理从 1 到 n 编号的 n 个座位。所有座位初始都是可预约的。
/// int reserve() 返回可以预约座位的 最小编号 ，此座位变为不可预约。
/// void unreserve(int seatNumber) 将给定编号 seatNumber 对应的座位变成可以预约。

struct SeatManager {
    seat: Vec<bool>,
    reserve: Vec<i32>,
    unreserve: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        SeatManager {
            seat: vec![true; n as usize],
            reserve: vec![],
            unreserve: vec![],
        }
    }

    fn reserve(&mut self) -> i32 {
        for i in 0..self.seat.len() {
            if self.seat[i] {
                self.seat[i] = false;
                return (i + 1) as i32;
            }
        }
        -1
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.seat[(seat_number - 1) as usize] = true;
    }
}
