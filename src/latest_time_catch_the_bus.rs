use std::collections::VecDeque;

/// 2332. 坐上公交的最晚时间
///
/// 给你一个下标从 0 开始长度为 n 的整数数组 buses ，其中 buses[i] 表示第 i 辆公交车的出发时间。
/// 同时给你一个下标从 0 开始长度为 m 的整数数组 passengers ，
/// 其中 passengers[j] 表示第 j 位乘客的到达时间。所有公交车出发的时间互不相同，所有乘客到达的时间也互不相同。
/// 给你一个整数 capacity ，表示每辆公交车 最多 能容纳的乘客数目。
/// 每位乘客都会搭乘下一辆有座位的公交车。如果你在 y 时刻到达，公交在 x 时刻出发，
/// 满足 y <= x  且公交没有满，那么你可以搭乘这一辆公交。最早 到达的乘客优先上车。
/// 返回你可以搭乘公交车的最晚到达公交站时间。你 不能 跟别的乘客同时刻到达。
/// 注意：数组 buses 和 passengers 不一定是有序的。
///
pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
    let mut buses = buses.clone();
    buses.sort();
    let mut passengers = passengers.clone();
    passengers.sort();

    let mut vp = VecDeque::from(passengers.clone());
    let mut index = 0;

    let mut ans = vec![vec![]; buses.len()];
    while !vp.is_empty() && index < buses.len() - 1 {
        if let Some(v) = vp.pop_front() {
            if v <= buses[index] && ans[index].len() <= capacity as usize {
                ans[index].push(v);
            } else {
                index += 1;
                ans[index].push(v);
            }
        }
    }

    println!("ans ==== {:?} \n", ans);

    // let mut map = vec![];
    // let mut next = VecDeque::new();
    // for b in 0..buses.len() {
    //     let mut mid = vec![];
    //     for n in 0..next.len() {
    //         if let Some(n) = next.get(n) {
    //             if *n <= buses[b] && mid.len() < capacity as usize {
    //                 mid.push(*n);
    //                 next.pop_front();
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    //     if !next.is_empty() {
    //         continue;
    //     }
    //     for _ in 0..capacity {
    //         if let Some(v) = vp.pop_front() {
    //             if v <= buses[b] && mid.len() < capacity as usize {
    //                 mid.push(v);
    //             } else {
    //                 next.push_back(v);
    //                 break;
    //             }
    //         }
    //     }
    //     map.push(mid);
    // }

    // println!("{:?}\n", buses);
    // println!("{:?}\n", map);
    // println!("{:?}\n", passengers);

    return 0;

    // let last_buses = map.last().unwrap();

    // if !last_buses.is_empty() {
    //     let mut last_value = 0;
    //     if let Some(l) = last_buses.last() {
    //         last_value = *l;
    //     }
    //     if last_buses.len() < capacity as usize {
    //         last_value = *buses.last().unwrap();
    //     }

    //     let mut max = 0;
    //     for i in 0..(last_value + 1) {
    //         if !passengers.contains(&i) {
    //             max = max.max(i);
    //         }
    //     }
    //     return max;
    // } else {
    //     return *buses.last().unwrap();
    // }
}

#[test]
fn test_1() {
    let buses = vec![10, 20];
    let passengers = vec![2, 17, 18, 19];
    let capacity = 2;
    assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 16)
}

#[test]
fn test_3() {
    let buses = vec![3];
    let passengers = vec![2, 4];
    let capacity = 2;
    assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 3);
}

#[test]
fn test_7() {
    let buses = vec![18, 8, 3, 12, 9, 2, 7, 13, 20, 5];
    let passengers = vec![13, 10, 8, 4, 12, 14, 18, 19, 5, 2, 30, 34];
    let capacity = 1;
    assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 11);
}
