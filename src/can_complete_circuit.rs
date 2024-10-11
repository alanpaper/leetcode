/// 134.加油站
/// 在一条环路上有 n 个加油站，其中第 i 个加油站有汽油 gas[i] 升。
/// 你有一辆油箱容量无限的的汽车，从第 i 个加油站开往第 i+1 个加油站需要消耗汽油 cost[i] 升。
/// 你从其中的一个加油站出发，开始时油箱为空。
/// 给定两个整数数组 gas 和 cost ，如果你可以按顺序绕环路行驶一周，
/// 则返回出发时加油站的编号，否则返回 -1 。如果存在解，则 保证 它是 唯一 的。
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut ans = 0;
    while ans < gas.len() {
        let mut stock = 0;
        let mut is_true = true;
        for i in 0..gas.len() {
            let mut index = ans + i;
            if index > gas.len() - 1 {
                index -= gas.len();
            }
            stock = stock + gas[index] - cost[index];
            if stock < 0 {
                is_true = false;
                break;
            }
        }
        if is_true {
            return ans as i32;
        }
        ans += 1;
    }
    -1
}

pub fn can_complete_circuit_ii(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut stocks = vec![];
    for i in 0..gas.len() {
        stocks.push(gas[i] - cost[i]);
    }
    let sum: i32 = stocks.iter().sum();
    if sum < 0 {
        return -1;
    }
    let mut ans = 0;
    let mut stock = 0;
    for i in 0..stocks.len() {
        if stocks[i] > 0 {
            if stock == 0 {
                ans = i;
                stock = stocks[i];
            } else {
                stock += stocks[i];
                continue;
            }
        } else if stocks[i] < 0 {
            stock += stocks[i];
            if stock < 0 {
                ans = 0;
                stock = 0;
            } else {
                continue;
            }
        }
    }
    ans as i32
}

#[test]
fn test_1() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    assert_eq!(can_complete_circuit_ii(gas, cost), 3);
}

#[test]
fn test_2() {
    let gas = vec![2, 3, 4];
    let cost = vec![3, 4, 3];
    assert_eq!(can_complete_circuit_ii(gas, cost), -1);
}

#[test]
fn test_3() {
    let gas = vec![3, 1, 1];
    let cost = vec![1, 2, 2];
    assert_eq!(can_complete_circuit_ii(gas, cost), 0);
}

#[test]
fn test_4() {
    let gas = vec![
        31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 1, 2, 3, 4,
        5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
        29, 30,
    ];
    let cost = vec![
        36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
        34, 35,
    ];
    assert_eq!(can_complete_circuit_ii(gas, cost), 15);
}
