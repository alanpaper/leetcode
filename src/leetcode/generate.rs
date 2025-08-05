/// 181. 杨辉三角


pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let zero = vec![1];
    ans.push(zero);
    for n in 1..num_rows {
        let mut current = vec![1; n as usize + 1];
        if let Some(lasts) = ans.last() {
            let mut current_index = 1;
            let mut i = 0;
            if lasts.len() >= 2 {
                while i <= lasts.len() - 2 {
                    current[current_index] = lasts[i] + lasts[i+1];
                    i += 1;
                    current_index += 1;
                }
            }
        }
        ans.push(current)
    }
    ans
}


#[test]
fn test_1() {
    assert_eq!(generate(5), vec![
        vec![1],
        vec![1,1],
        vec![1,2,1],
        vec![1,3,3,1],
        vec![1,4,6,4,1]
    ]);
}
#[test]
fn test_2() {
    assert_eq!(generate(1), vec![
        vec![1],
    ]);
}






