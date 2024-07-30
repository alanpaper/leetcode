/// 2961. 双模幂运算
///
/// 给你一个下标从 0 开始的二维数组 variables ，其中 variables[i] = [ai, bi, ci, mi]，以及一个整数 target 。
/// 如果满足以下公式，则下标 i 是 好下标：
/// 0 <= i < variables.length
/// ((aibi % 10)ci) % mi == target
// 返回一个由 好下标 组成的数组，顺序不限。
///
pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
    let pow = |mut x, mut n, m| {
      let mut res = 1;
      while n > 0 {
          if n % 2 > 0 {
              res = res * x % m;
          }
          x = x * x % m;
          n /= 2;
      }
      res
  };
  let check = |v: &Vec<_>| pow(pow(v[0], v[1], 10), v[2], v[3]) == target;
  variables.iter()
      .enumerate()
      .filter_map(|(i, v)| check(v).then_some(i as i32))
      .collect()
}

#[test]
fn test1() {
    assert_eq!(
        get_good_indices(
            vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]],
            2
        ),
        vec![0, 2]
    )
}

#[test]
fn test2() {
    assert_eq!(get_good_indices(vec![vec![39, 3, 1000, 1000]], 17), vec![])
}
