/// 997. 找到小镇的法官
/// 小镇里有 n 个人，按从 1 到 n 的顺序编号。传言称，这些人中有一个暗地里是小镇法官。
// 如果小镇法官真的存在，那么：
// 小镇法官不会信任任何人。
// 每个人（除了小镇法官）都信任这位小镇法官。
// 只有一个人同时满足属性 1 和属性 2 。
// 给你一个数组 trust ，其中 trust[i] = [ai, bi] 表示编号为 ai 的人信任编号为 bi 的人。
// 如果小镇法官存在并且可以确定他的身份，请返回该法官的编号；否则，返回 -1 。
#[derive(Clone, Debug)]
struct Graph {
    pub in_degree: i32,
    pub out_degree: i32,
}

impl Graph {
    fn new(in_degree: i32, out_degree: i32) -> Self {
        Graph {
            in_degree,
            out_degree,
        }
    }
}

// 图的出度和入度模拟
pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    if n == 1 && trust.is_empty() {
        return 1;
    }
    let graph = Graph::new(0, 0);
    let mut ans = vec![graph; n as usize + 1];
    for t in trust {
        let _in = t[0];
        let _out = t[1];
        let cur_in = ans[_in as usize].clone();
        let cur_out = ans[_out as usize].clone();
        ans[_out as usize] = Graph::new(cur_out.in_degree + 1, cur_out.out_degree);
        ans[_in as usize] = Graph::new(cur_in.in_degree, cur_out.out_degree + 1);
    }
    for a in ans.iter().enumerate() {
        if a.1.in_degree == n - 1 && a.1.out_degree == 0 {
            return a.0 as i32;
        }
    }
    -1
}

#[test]
fn test_1() {
    assert_eq!(find_judge(2, vec![vec![1, 2]]), 2);
}

#[test]
fn test_2() {
    assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
}

#[test]
fn test_3() {
    assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]), -1);
}
