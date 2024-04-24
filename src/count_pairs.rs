/// 2316.统计无向图中无法互相到达点对数

pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let mut table = vec![vec![]; n as usize];

    for p in &edges {
        let (x, y) = (p[0], p[1]);
        table[x as usize].push(y);
    }

    println!("{:?}====table", table);

    let mut table_link = vec![vec![]; n as usize];

    for i in 0..n {
        dfs(i, i, &table, &mut table_link);
    }

    println!("{:?}====", table_link);

    0
}

fn dfs(num1: i32, num2: i32, table: &Vec<Vec<i32>>, table_link: &mut Vec<Vec<i32>>) {
    println!("{:?} == {:?}", &table[num1 as usize], num1);
    for l in &table[num1 as usize] {
        table_link[num2 as usize].push(*l);
        dfs(*l, num2, table, table_link);
    }
}

#[test]
fn test_1() {
    assert_eq!(count_pairs(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]), 0);
}

#[test]
fn test_2() {
    assert_eq!(
        count_pairs(
            7,
            vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]]
        ),
        14
    );
}


/// 2824. 统计和小于目标的下面对数目
pub fn count_pairs_2824(nums: Vec<i32>, target: i32) -> i32 {

    let nums_bak = nums.clone().sort();


    




    0


}