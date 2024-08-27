use std::collections::HashMap;

pub fn find_permutation_difference(s: String, t: String) -> i32 {
    let mut map = HashMap::new();
    for i in s.chars().enumerate() {
        map.entry(i.1).or_insert(i.0);
    }
    let mut ans = 0;
    for i in t.chars().enumerate() {
        let num = map.get(&i.1).unwrap();
        ans += num.max(&i.0) - num.min(&i.0);
    }
    ans as i32
}
