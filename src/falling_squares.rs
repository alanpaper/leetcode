use std::collections::HashMap;
/// 699. 掉落的方块
/// 在二维平面上的 x 轴上，放置着一些方块。
// 给你一个二维整数数组 positions ，其中 positions[i] = [lefti, sideLengthi] 表示：第 i 个方块边长为 sideLengthi ，其左侧边与 x 轴上坐标点 lefti 对齐。
// 每个方块都从一个比目前所有的落地方块更高的高度掉落而下。方块沿 y 轴负方向下落，直到着陆到 另一个正方形的顶边 或者是 x 轴上 。
/// 一个方块仅仅是擦过另一个方块的左侧边或右侧边不算着陆。一旦着陆，它就会固定在原地，无法移动。
// 在每个方块掉落后，你必须记录目前所有已经落稳的 方块堆叠的最高高度 。
// 返回一个整数数组 ans ，其中 ans[i] 表示在第 i 块方块掉落后堆叠的最高高度。
///
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    pub x1: i32,
    pub x2: i32,
}

pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    let mut height_map: HashMap<Position, i32> = HashMap::new();

    let mut ans = vec![];
    let mut height = 0;
    for p in positions {
        let position = Position {
            x1: p[0],
            x2: p[0] + p[1],
        };

        if height_map.is_empty() {
            height_map.entry(position.clone()).or_insert(p[1]);
            height = p[1].max(height);
        } else {
            for m in height_map.clone().into_iter() {
                if m.0.x2 <= position.clone().x1 || position.clone().x2 <= m.0.x1 {
                    height_map.entry(position.clone()).or_insert(p[1]);
                    height = p[1].max(height);
                } else {
                    height_map.remove(&m.0);
                    let h = p[1] + m.1;
                    height = h.max(height);
                    if m.0.x1 >= position.clone().x1 || position.clone().x2 >= m.0.x2 {
                        height_map.entry(position).or_insert(h);
                    } else if m.0.x1 > position.clone().x1 || position.clone().x2 < m.0.x2 {
                        height_map
                            .entry(Position {
                                x1: position.clone().x1,
                                x2: m.0.x1,
                            })
                            .or_insert(h);
                        height_map
                            .entry(Position {
                                x1: position.clone().x2,
                                x2: m.0.x2,
                            })
                            .or_insert(m.1);
                    } else {
                        height_map
                            .entry(Position {
                                x1: m.0.x1,
                                x2: position.clone().x1,
                            })
                            .or_insert(m.1);
                        height_map
                            .entry(Position {
                                x1: m.0.x2,
                                x2: position.clone().x2,
                            })
                            .or_insert(h);
                    }
                }
            }
        }
        println!("{:?}", height_map);
        ans.push(height);
    }

    ans
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

// 9 19  10
// 4 5 1  10
// 2 3 1  10
// 7 11 4  14
// 6 16 10 24
