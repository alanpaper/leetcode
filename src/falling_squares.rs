/// 699. 掉落的方块
/// 在二维平面上的 x 轴上，放置着一些方块。
// 给你一个二维整数数组 positions ，其中 positions[i] = [lefti, sideLengthi] 表示：第 i 个方块边长为 sideLengthi ，其左侧边与 x 轴上坐标点 lefti 对齐。
// 每个方块都从一个比目前所有的落地方块更高的高度掉落而下。方块沿 y 轴负方向下落，直到着陆到 另一个正方形的顶边 或者是 x 轴上 。
/// 一个方块仅仅是擦过另一个方块的左侧边或右侧边不算着陆。一旦着陆，它就会固定在原地，无法移动。
// 在每个方块掉落后，你必须记录目前所有已经落稳的 方块堆叠的最高高度 。
// 返回一个整数数组 ans ，其中 ans[i] 表示在第 i 块方块掉落后堆叠的最高高度。
///
pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = vec![0; 100000000 + 100000];
    let mut res = vec![];
    let mut max = 0;
    for p in positions {
        let x1 = p[0];
        let h = p[1];
        let mut height = h;
        for i in 0..h {
            let index = (x1 + i + 1) as usize;
            ans[index] += h;
            height = height.max(ans[index]);
        }
        for i in 0..h {
            let index = (x1 + i + 1) as usize;
            ans[index] = height;

        }
        max = max.max(height);
        res.push(max);
    }
    res
}

#[test]
fn test1() {
    let positions = vec![vec![1, 2], vec![2, 3], vec![6, 1]];
    assert_eq!(falling_squares(positions), vec![2, 5, 5]);
}

#[test]
fn test2() {
    let positions = vec![vec![1, 2], vec![1, 3]];
    assert_eq!(falling_squares(positions), vec![2, 5]);
}

#[test]
fn test3() {
    let positions = vec![vec![9, 10], vec![4, 1], vec![2, 1], vec![7, 4], vec![6, 10]];
    assert_eq!(falling_squares(positions), vec![10, 10, 10, 14, 24]);
}

#[test]
fn test4() {
    let positions = vec![vec![9, 7], vec![1, 9], vec![3, 1]];
    assert_eq!(falling_squares(positions), vec![7, 16, 17]);
}


