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
    let mut map = vec![];
    let mut next = VecDeque::new();
    for b in 0..buses.len() {
        let mut mid = vec![];
        for n in 0..next.len() {
            if let Some(n) = next.get(n) {
                if *n <= buses[b] && mid.len() < capacity as usize {
                    mid.push(*n);
                    next.pop_front();
                } else {
                    break;
                }
            }
        }
        if !next.is_empty() {
            continue;
        }
        for _ in 0..capacity {
            if let Some(v) = vp.pop_front() {
                if v <= buses[b] && mid.len() < capacity as usize {
                    mid.push(v);
                } else {
                    next.push_back(v);
                    break;
                }
            }
        }
        map.push(mid);
    }

    println!("{:?}\n", buses);
    println!("{:?}\n", map);
    println!("{:?}\n", passengers);

    let last_buses = map.last().unwrap();

    if !last_buses.is_empty() {
        let mut last_value = 0;
        if let Some(l) = last_buses.last() {
            last_value = *l;
        }
        if last_buses.len() < capacity as usize {
            last_value = *buses.last().unwrap();
        }

        let mut max = 0;
        for i in 0..(last_value + 1) {
            if !passengers.contains(&i) {
                max = max.max(i);
            }
        }
        return max;
    } else {
        return *buses.last().unwrap();
    }
}

#[test]
fn test_1() {
    let buses = vec![10, 20];
    let passengers = vec![2, 17, 18, 19];
    let capacity = 2;
    assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 16)
}

#[test]
fn test_2() {
    let buses = vec![20, 30, 10];
    let passengers = vec![19, 13, 26, 4, 25, 11, 21];
    let capacity = 2;
    assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 20)
}

#[test]
fn test_3() {
    let buses = vec![3];
    let passengers = vec![2, 4];
    let capacity = 2;
    assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 3);
}

#[test]
fn test_4() {
    let buses = vec![2];
    let passengers = vec![2];
    let capacity = 2;
    assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 1);
}

#[test]
fn test_5() {
    let buses = vec![3];
    let passengers = vec![4];
    let capacity = 1;
    assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 3);
}

#[test]
fn test_6() {
    let buses = vec![2, 3];
    let passengers = vec![3, 2];
    let capacity = 2;
    assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 1);
}

#[test]
fn test_7() {
    let buses = vec![18, 8, 3, 12, 9, 2, 7, 13, 20, 5];
    let passengers = vec![13, 10, 8, 4, 12, 14, 18, 19, 5, 2, 30, 34];
    let capacity = 1;
    assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 11);
}

#[test]
fn test_8() {
    let buses = vec![
        7153, 3582, 565, 6739, 4805, 2050, 6347, 4575, 3432, 7770, 5501, 4244, 7358, 1950, 2886,
        5489, 9499, 3822, 4162, 2739, 1342, 7468, 1928, 9345, 1046, 550, 9705, 68, 1847, 1202, 955,
        6295, 6902, 241, 204, 3936, 7830, 1952, 3855, 812, 5199, 3213, 4860, 1687, 3501, 5361,
        5694, 1605, 3986, 3465, 8103, 3036, 903, 7785, 8504, 5339, 4031, 2015, 8384, 6153, 7451,
        4277, 3943, 6593, 3657, 7789, 5673, 6133, 6814, 2964, 5383, 9630, 4006, 3107, 2537, 3621,
        9972, 8308, 9114, 898, 3181, 6376, 9701, 1388, 6667, 8774, 8105, 6675, 3042, 1006, 2950,
        2812, 8894, 9175, 7329, 982, 6570, 7021, 9254, 8511,
    ];
    let passengers = vec![
        3644, 4876, 6299, 919, 989, 7636, 6698, 206, 2557, 2516, 3298, 752, 3471, 1939, 5550, 4941,
        7497, 8436, 1070, 4798, 4091, 3889, 3571, 4344, 9880, 4825, 7860, 89, 5805, 9108, 7054,
        3436, 6726, 3967, 2998, 5828, 8094, 565, 646, 6717, 5300, 5010, 557, 8036, 5738, 7300,
        5563, 8090, 6315, 2438, 1182, 4341, 9374, 6355, 4852, 9474, 6730, 7350, 363, 6011, 832,
        2864, 4650, 6075, 3729, 1623, 8739, 7637, 191, 1339, 4364, 3136, 3945, 6538, 6341, 2478,
        7555, 2477, 9684, 9630, 4609, 9897, 4406, 2442, 6826, 2676, 2590, 1982, 2269, 2819, 7775,
        670, 3002, 7911, 2284, 9009, 3317, 2649, 3801, 7854, 29080, 25338, 13436, 24387, 22658,
        29352, 11958, 16258,
    ];
    let capacity = 2;
    assert_eq!(latest_time_catch_the_bus(buses, passengers, capacity), 9896);
}
