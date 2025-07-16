use std::{
    collections::{HashMap, HashSet, VecDeque},
    f32::INFINITY,
    i32::MAX,
};

/// 815. 公交路线
/// 给你一个数组 routes ，表示一系列公交线路，其中每个 routes[i] 表示一条公交线路，第 i 辆公交车将会在上面循环行驶。
/// 例如，路线 routes[0] = [1, 5, 7] 表示第 0 辆公交车会一直按序列 1 -> 5 -> 7 -> 1 -> 5 -> 7 -> 1 -> ... 这样的车站路线行驶。
/// 现在从 source 车站出发（初始时不在公交车上），要前往 target 车站。 期间仅可乘坐公交车。
/// 求出 最少乘坐的公交车数量 。如果不可能到达终点车站，返回 -1 。
pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    let mut station = HashSet::new();
    let mut station_map = HashMap::new();
    for r in &routes {
        for s in r {
            station.insert(*s);
        }
    }
    for s in station.iter() {
        for r in &routes {
            if r.contains(s) {
                let mut hs = HashSet::new();
                for l in r {
                    if s != l {
                        hs.insert(l);
                    }
                }
                station_map
                    .entry(s)
                    .and_modify(|f: &mut HashSet<&i32>| {
                        for h in hs.iter() {
                            f.insert(*h);
                        }
                    })
                    .or_insert(hs);
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(source);

    let mut station_set = HashSet::new();

    let mut ans_bool = false;
    let mut res = MAX;
    let mut ans = 1;
    while !queue.is_empty() {
        if let Some(x) = queue.pop_front() {
            if !station_set.contains(&x) {
                station_set.insert(x);
                if let Some(s) = station_map.get(&x) {
                    if s.contains(&target) {
                        ans_bool = true;
                        res = res.min(ans);
                        ans = 1;
                        continue;
                    } else {
                        ans += 1;
                        for i in s.iter() {
                            queue.push_back(**i);
                        }
                    }
                }
            }
        }
    }

    if ans_bool {
        return ans;
    }

    -1
}

#[test]
fn test1() {
    assert_eq!(
        num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6),
        2
    );
}

#[test]
fn test2() {
    assert_eq!(
        num_buses_to_destination(
            vec![
                vec![7, 12],
                vec![4, 5, 15],
                vec![6],
                vec![15, 19],
                vec![9, 12, 13, 19]
            ],
            15,
            12
        ),
        2
    );
}
