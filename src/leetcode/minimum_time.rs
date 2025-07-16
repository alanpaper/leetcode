/// 2187. 完成旅途的最少时间
/// 给你一个数组 time ，其中 time[i] 表示第 i 辆公交车完成 一趟旅途 所需要花费的时间。
/// 每辆公交车可以 连续 完成多趟旅途，也就是说，一辆公交车当前旅途完成后，可以 立马开始 下一趟旅途。
/// 每辆公交车 独立 运行，也就是说可以同时有多辆公交车在运行且互不影响。
/// 给你一个整数 totalTrips ，表示所有公交车 总共 需要完成的旅途数目。请你返回完成 至少 totalTrips 趟旅途需要花费的 最少 时间。
pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {

}

#[test]
fn test_1() {
    assert_eq!(minimum_time(vec![1, 2, 3], 5), 3);
}

#[test]
fn test_2() {
    assert_eq!(minimum_time(vec![2], 1), 2);
}
    