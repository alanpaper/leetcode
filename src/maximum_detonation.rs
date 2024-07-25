use std::collections::{HashSet, VecDeque};
/// 2101. 引爆最多的炸弹
///
pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for bomb in bombs.iter().enumerate() {
        let mut total = 1;
        let mut bombs_set = HashSet::new();
        let mut next_bombs_queue = VecDeque::new();
        next_bombs_queue.push_back(bomb);
        bombs_set.insert(bomb);
        while next_bombs_queue.len() != 0 {
            if let Some(next_b) = next_bombs_queue.pop_front() {
                for b in bombs.iter().enumerate() {
                    if !bombs_set.contains(&b) {
                        let x = (next_b.1[0] as i64 - b.1[0] as i64).abs();
                        let y = (next_b.1[1] as i64 - b.1[1] as i64).abs();
                        let distance = (((x * x) + y * y) as f64).sqrt();
                        if distance - next_b.1[2] as f64 <= 0.0 {
                            next_bombs_queue.push_back(b);
                            bombs_set.insert(b);
                            total += 1;
                        }
                    }
                }
            }
        }
        ans = ans.max(total);
    }
    ans
}

#[test]
fn test_1() {
    let bombs = vec![vec![2, 1, 3], vec![6, 1, 4]];
    assert_eq!(maximum_detonation(bombs.clone()), 2);
}

#[test]
fn test_2() {
    let bombs = vec![
        vec![1, 2, 3],
        vec![2, 3, 1],
        vec![3, 4, 2],
        vec![4, 5, 3],
        vec![5, 6, 4],
    ];
    assert_eq!(maximum_detonation(bombs.clone()), 5);
}

#[test]
fn test_3() {
    let bombs = vec![
        vec![85024, 58997, 3532],
        vec![65196, 42043, 9739],
        vec![85872, 75029, 3117],
        vec![73014, 91183, 7092],
        vec![29098, 40864, 7624],
        vec![11469, 13607, 4315],
        vec![98722, 69681, 9656],
        vec![75140, 42250, 421],
        vec![92580, 44040, 4779],
        vec![58474, 78273, 1047],
        vec![27683, 4203, 6186],
        vec![10714, 24238, 6243],
        vec![60138, 81791, 3496],
        vec![16227, 92418, 5622],
        vec![60496, 64917, 2463],
        vec![59241, 62074, 885],
        vec![11961, 163, 5815],
        vec![37757, 43214, 3402],
        vec![21094, 98519, 1678],
        vec![49368, 22385, 1431],
        vec![6343, 53798, 159],
        vec![80129, 9282, 5139],
        vec![69565, 32036, 6827],
        vec![59372, 64978, 6575],
        vec![44948, 71199, 7095],
        vec![46390, 91701, 1667],
        vec![37144, 98691, 8128],
        vec![13558, 81505, 4653],
        vec![41234, 48161, 9304],
        vec![14852, 3206, 5369],
    ];
    assert_eq!(maximum_detonation(bombs.clone()), 3);
}
