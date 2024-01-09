/// 207. 课程表
///
///

fn dfs(u: i32, visited: &mut Vec<i32>, edges: &Vec<Vec<i32>>, valid: &mut bool) {
    visited[u as usize] = 1;
    for v in edges.get(u as usize).unwrap() {
        match visited.get(*v as usize).unwrap() {
            0 => dfs(*v, visited, edges, valid),
            1 => {
                *valid = false;
                return;
            }
            _ => {}
        }
    }
    visited[u as usize] = 1;
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut valid = true;
    let mut edges: Vec<Vec<i32>> = vec![];
    let mut visited = vec![num_courses; 0];

    for rela in prerequisites {
        match rela.get(1) {
            Some(i) => edges[*i as usize].push(*rela.get(0).unwrap()),
            None => {}
        }
    }

    for i in 0..num_courses {
        match visited.get(i as usize) {
            Some(_num) => {}
            None => {
                dfs(i, &mut visited, &edges, &mut valid);
            }
        }
    }

    valid
}

#[test]
fn solve_test_two() {
    assert_eq!(can_finish(2, vec![vec![1, 0]]), true)
}
