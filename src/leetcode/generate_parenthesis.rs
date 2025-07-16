pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut ans: Vec<Vec<String>> = vec![vec!["".to_string()], vec!["()".to_string()]];

    for num in 2..=n {
        let mut temp: Vec<String> = vec![];

        for i in 0..num {
            for p in &ans[i as usize] {
                for q in &ans[(num - i - 1) as usize] {
                    let parenth_str = format!("({}){}", p, q);
                    temp.push(parenth_str);
                }
            }
        }
        ans.push(temp);
    }
    ans.pop().unwrap()
}

#[test]
fn tets_1() {
    assert_eq!(
        generate_parenthesis(3),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    )
}
