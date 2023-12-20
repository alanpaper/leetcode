use std::collections::HashMap;


 pub fn length_of_longest_substring(s: String) -> i32 {

  let (mut ans, mut cnt) = (0,0);
  let mut map = HashMap::new();
  let s = s.chars().collect::<Vec<_>>();
  let mut l = 0;

  s.iter().enumerate().for_each(|(i,c)| {
    match map.get(c) {
        None => {
          cnt += 1;
          ans=ans.max(cnt);
        }
        Some(&i)=> {
          for c in &s[l..=i] {
            map.remove(c);
          }
          cnt -= i - 1;
          l = i+1;
        }
    }
    map.insert(*c, i);
  });

  ans.try_into().unwrap()

 }




#[cfg(test)]
mod tests {
  use crate::longest;
  #[test]
  fn longest_substring() {
    let larger_len = longest::length_of_longest_substring("aabbccc".to_string());
    assert_eq!(larger_len, 3)
  }
}

