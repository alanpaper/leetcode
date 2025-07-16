/// 2477. 到达首都的最少油耗
pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let mut max_val = 0;
    for road in &roads {
        max_val = max_val.max(road[0].max(road[1]));
    }
    let mut edge = vec![vec![]; (max_val + 1) as usize];

    for road in roads {
        edge[road[0] as usize].push(road[1]);
        edge[road[1] as usize].push(road[0]);
    }

    fn dfs() {}

    println!("{:?}", edge);

    0
}

#[test]
fn _test() {
    let roads = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
    assert_eq!(minimum_fuel_cost(roads, 5), 3);
}
#[test]
fn _test_1() {
    let roads = vec![
        vec![3, 1],
        vec![3, 2],
        vec![1, 0],
        vec![0, 4],
        vec![0, 5],
        vec![4, 6],
    ];
    assert_eq!(minimum_fuel_cost(roads, 2), 7);
}
