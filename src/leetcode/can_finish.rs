/// 207. 课程表
fn dfs(u: i32, edges: &Vec<Vec<i32>>, lines: &mut Vec<i32>) -> bool {
    let mut ans = true;
    lines.push(u);
    for line in edges[u as usize].iter() {
        if !lines.contains(line) {
            println!("{:?}-----{:?}", line, lines);
            return dfs(*line, edges, lines);
        } else {
            ans = false;
            break;
        }
    }
    println!("{:?}=lines", lines);
    lines.clear();
    ans
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut edges = vec![vec![]; (num_courses + 1) as usize];

    let mut lines = vec![];

    let mut ans = true;
    for i in prerequisites {
        let (next, pre) = (i[0], i[1]);
        edges[pre as usize].push(next);
    }

    println!("{:?}", edges);

    for i in 0..(num_courses + 1) {
        println!("{:?}========", ans);
        if ans {
            ans = dfs(i, &edges, &mut lines);
        }
    }
    ans
}

// #[test]
// fn solve_test_two() {
//     assert_eq!(can_finish(2, vec![vec![1, 0]]), true)
// }
// #[test]
// fn solve_test_three() {
//     assert_eq!(can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
// }

#[test]
fn solve_test_four() {
    assert_eq!(
        can_finish(2, vec![vec![1, 0], vec![2, 0], vec![0, 2]]),
        false
    );
}

// #[test]
// fn solve_test_five() {
//     assert_eq!(
//         can_finish(5, vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]),
//         true
//     );
// }
